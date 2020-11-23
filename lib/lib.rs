#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use image::DynamicImage as DynamicImage;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn normalize_pixel_value(pixel_value: &u8) -> f64 {
    (*pixel_value as f64) / 255.
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
    col_type: i32,
) {
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
    unsafe{
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
            col_type,
            h as i32,
            w as i32,
            3
        ) 
    }


}

