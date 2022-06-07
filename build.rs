extern crate cc;
use std::env;

fn main() {
    let prefix = env::var("CONDA_PREFIX").unwrap();

    cc::Build::new()
	.file("src/gen/ffi.cc")
	.cpp(true)
        .flag_if_supported("-std=c++14")
        .include("include")
	.include(prefix.clone() + "/include")
	.include(prefix.clone() + "/include/eigen3")
        .compile("eigen_rxx");

    println!("cargo:rerun-if-changed=src/gen/ffi.cc");
    println!("cargo:rerun-if-changed=include/wrapper.hh");
}
