extern crate cc;

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
    let ff_path = get_ff_path();
    if ff_path.is_none() {
        panic!("FastFlow not found!");
    }
    let ff_path = ff_path.unwrap();

    println!("{}", ff_path);

    cc::Build::new()
        .file("libffbuffer/ff_ubuffer.cpp")
        .files(
            fs::read_dir("libffbuffer/benches")
                .expect("read_dir call failed")
                .map(|a| a.expect("read_file failed").path()),
        )
        .cpp(true)
        .compiler("clang++")
        .opt_level(3)
        .warnings(false)
        .flag("-flto=thin")
        .include(ff_path)
        .compile("ffbuffer");

    // set rerun build if c++ lib change
    println!("cargo:rerun-if-changed=./libffbuffer/ff_ubuffer.cpp");
    println!("cargo:rerun-if-changed=./libffbuffer/ff_ubuffer.hpp");
    println!("cargo:rerun-if-changed=./libffbuffer/benches/producer_consumer.cpp");
}
