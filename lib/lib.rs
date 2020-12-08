#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use image;
use image::DynamicImage;
use image::ImageBuffer;
use std::f64::consts::PI;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn normalize_pixel_value(pixel_value: &u8) -> f64 {
    (*pixel_value as f64) / 255.
}

pub struct Flow {
    height: usize,
    width: usize,
    vx: Vec<f64>,
    vy: Vec<f64>,
}

fn cart_to_polar(x: f64, y: f64) -> (f64, f64) {
    let mag = (x.powi(2) + y.powi(2)).sqrt();
    let angle = (y / x).atan();
    (mag, angle)
}

#[inline]
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    assert!(0. <= h && h <= 360.);
    assert!(0. <= s && s <= 1.);
    assert!(0. <= v && v <= 1.);

    let chroma = v * s;
    let h_prime = (h / 60.) as i32;
    let x = chroma * (h_prime % 2 - 1).abs() as f64;
    let (r1, g1, b1) = match h_prime {
        h if (0 <= h && h <= 1) => (chroma, x, 0.),
        h if (1 <= h && h <= 2) => (x, chroma, 0.),
        h if (2 <= h && h <= 3) => (0., chroma, x),
        h if (3 <= h && h <= 4) => (0., x, chroma),
        h if (4 <= h && h <= 5) => (x, 0., chroma),
        h if (5 <= h && h <= 6) => (chroma, 0., x),
        _ => (0., 0., 0.),
    };
    let m = v - chroma;
    let r = r1 + m;
    let g = g1 + m;
    let b = b1 + m;
    return ((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8);
}

impl Flow {
    pub fn visualize_rgb(&self) -> image::RgbImage {
        let mut rgb = ImageBuffer::new(self.width as u32, self.height as u32);
        let cdepth_bytes = 3;
        let mut hsv = vec![0.; self.width * self.height * cdepth_bytes];
        let mut max_mag = 0.;
        for (i, row) in hsv.chunks_exact_mut(self.width * cdepth_bytes).enumerate() {
            for (j, pixel) in row.chunks_exact_mut(3).enumerate() {
                let dx = self.vx.get(i * self.width + j).unwrap();
                let dy = self.vy.get(i * self.width + j).unwrap();
                let (mag, angle) = cart_to_polar(dx.clone(), dy.clone());
                let angle_deg = (angle * 180. / PI) + 180.;
                if mag > max_mag {
                    max_mag = mag;
                }
                pixel[0] = angle_deg;
                pixel[1] = 1.;
                pixel[2] = mag;
            }
        }
        for ((_, _, out_pixel), in_pixel) in rgb.enumerate_pixels_mut().zip(hsv.chunks_exact(3)) {
            let h: f64 = in_pixel[0];
            let s: f64 = in_pixel[1];
            let v: f64 = in_pixel[2] / max_mag;
            let (r, g, b) = hsv_to_rgb(h, s, v);
            *out_pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
        return rgb;
    }
}

pub fn coarse2fine_flow(
    img1: &DynamicImage,
    img2: &DynamicImage,
    alpha: f64,
    ratio: f64,
    min_width: i32,
    num_outer_fp_iters: i32,
    num_inner_fp_iters: i32,
    num_sor_iters: i32,
) -> Flow {
    let img1_rgb = img1.to_rgb8();
    let h: usize = img1_rgb.height() as usize;
    let w: usize = img1_rgb.width() as usize;
    let img1_buf: Vec<u8> = img1_rgb.into_raw();
    let mut img1_buf_f64: Vec<f64> = img1_buf.iter().map(normalize_pixel_value).collect();

    let img2_rgb = img2.to_rgb8();
    let img2_buf: Vec<u8> = img2_rgb.into_raw();
    let mut img2_buf_f64: Vec<f64> = img2_buf.iter().map(normalize_pixel_value).collect();

    let mut vx: Vec<f64> = vec![0.0; h * w];
    let mut vy: Vec<f64> = vec![0.0; h * w];
    let mut warpI2: Vec<f64> = vec![0.0; h * w * 3];
    unsafe {
        Coarse2FineFlowWrapper(
            vx.as_mut_ptr(),
            vy.as_mut_ptr(),
            warpI2.as_mut_ptr(),
            img1_buf_f64.as_mut_ptr(),
            img2_buf_f64.as_mut_ptr(),
            alpha,
            ratio,
            min_width,
            num_outer_fp_iters,
            num_inner_fp_iters,
            num_sor_iters,
            0,
            h as i32,
            w as i32,
            3,
        )
    }
    let flow = Flow {
        height: h,
        width: w,
        vx: vx,
        vy: vy,
    };
    return flow;
}
