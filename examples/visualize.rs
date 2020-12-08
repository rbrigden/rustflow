use clap;
use image::io::Reader as ImageReader;
use rustflow::coarse2fine_flow;

pub fn main() {
    let matches = clap::App::new("RustFlow")
        .about("Coarse2Fine dense optical flow")
        .arg(clap::Arg::with_name("img1").long("img1").takes_value(true))
        .arg(clap::Arg::with_name("img2").long("img2").takes_value(true))
        .arg(clap::Arg::with_name("out").long("out").short("o").takes_value(true))
        .get_matches();

    let img1_path = matches.value_of("img1").unwrap_or("./assets/car1.jpg");
    let img2_path = matches.value_of("img2").unwrap_or("./assets/car2.jpg");
    let out_path = matches.value_of("out").unwrap_or("./assets/flow.jpg");
    println!("Computing flow {} ~> {}", img1_path, img2_path);

    let img1 = match ImageReader::open(img1_path).unwrap().decode() {
        Ok(img) => img,
        Err(err) => panic!("Problem opening the image: {:?}", err),
    };

    let img2 = match ImageReader::open(img2_path).unwrap().decode() {
        Ok(img) => img,
        Err(err) => panic!("Problem opening the image: {:?}", err),
    };
    let flow = coarse2fine_flow(&img1, &img2, 0.012, 0.75, 20, 7, 1, 30);
    let out = flow.visualize_rgb();
    out.save(&out_path).unwrap();
    println!("Visualized flow written to {}", out_path);
}
