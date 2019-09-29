extern crate cmake;

use cmake::Config;

fn main()
{
    // compile the c++ lib
    let dst = Config::new("libffbuffer")
    .define("CMAKE_CXX_COMPILER", "clang++")
    .define("CMAKE_CXX_FLAGS", "-flto=thin -O3")
    .build(); 

    // link library
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=ffbuffer");

    // set rerun build if c++ lib change
    println!("cargo:rerun-if-changed=./libffbuffer/ff_ubuffer.cpp");
    println!("cargo:rerun-if-changed=./libffbuffer/CMakeLists.txt");
}
