extern crate cc;

use std::path::PathBuf;
use std::{env, fs};

fn get_ff_path() -> Option<String> {
    if let Some(val) = env::var_os("FF_PATH") {
        let s = val.to_str().unwrap();
        return Some(String::from(s));
    }
    if let Some(val) = dirs::home_dir() {
        let ff_home = val.join("fastflow");
        if ff_home.is_dir() {
            let s = ff_home.to_str().unwrap();
            return Some(String::from(s));
        }
    }
    return None;
}

fn main() {
    // search for FastFlow
    let ff_path = get_ff_path();
    if ff_path.is_none() {
        panic!("FastFlow not found!");
    }
    let ff_path = ff_path.unwrap();
    println!("FF_PATH={}", ff_path);

    // get c++ benches path
    let benches: Vec<PathBuf> = fs::read_dir("libffbuffer/benches")
        .expect("read_dir call failed")
        .map(|a| a.expect("read_file failed").path())
        .collect();

    // general configuration
    let mut build = cc::Build::new();
    build
        .file("libffbuffer/ff_ubuffer.cpp")
        .files(benches.clone())
        .compiler("clang")
        .cpp(true)
        .opt_level(3)
        .warnings(false)
        .include(ff_path);

    // optinal cross language feature
    if cfg!(feature = "crosslto") {
        build.flag("-flto=thin");
    }

    // comnpile c++ library
    build.compile("ffbuffer");

    // set rerun if c++ lib change
    println!("cargo:rerun-if-changed=./libffbuffer/ff_ubuffer.cpp");
    println!("cargo:rerun-if-changed=./libffbuffer/ff_ubuffer.hpp");
    for p in benches {
        println!("cargo:rerun-if-changed={}", p.display());
    }
}
