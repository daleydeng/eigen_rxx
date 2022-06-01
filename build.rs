use std::env;

fn main() {
    let prefix = env::var("CONDA_PREFIX").unwrap();

    let inc_dirs = [
        "include",
        &(prefix.clone() + "/include"),
        &(prefix.clone() + "/include/eigen3"),
    ];

    cxx_build::bridge("src/wrapper.rs")
        .flag_if_supported("-std=c++14")
        .includes(&inc_dirs)
        .compile("eigen_rsxx");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/wrapper.hh");
}
