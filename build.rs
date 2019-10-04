extern crate cc;

fn main()
{

    cc::Build::new()
        .file("libffbuffer/ff_ubuffer.cpp")
        .file("libffbuffer/benches/producer_consumer.cpp")
        .cpp(true)
        .compiler("clang++")
        .opt_level(3)
        .warnings(false)
        .flag("-flto=thin")
        // TODO: add some autodiscovery of the fastflow library
        .include("/home/luca/fastflow")
        .compile("ffbuffer");

    // set rerun build if c++ lib change
    println!("cargo:rerun-if-changed=./libffbuffer/ff_ubuffer.cpp");
    println!("cargo:rerun-if-changed=./libffbuffer/ff_ubuffer.hpp");
    println!("cargo:rerun-if-changed=./libffbuffer/benches/producer_consumer.cpp");
}
