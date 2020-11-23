extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {

	// Compile cpp files into static library
	cc::Build::new()
        .cpp(true)
        .file("vendor/Coarse2FineFlowWrapper.cpp")
        .file("vendor/GaussianPyramid.cpp")
        .file("vendor/OpticalFlow.cpp")
		.compile("Coarse2FineFlowWrapper");
    

    let bindings = bindgen::Builder::default()

        .header("vendor/Coarse2FineFlowWrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
