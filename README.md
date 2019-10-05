# FF Buffer

An ongoing porting of the c++ FastFlow lock-free queue to Rust.

The library is a simple interface that mimic the mpsc queue of standard Rust and internally use the C++ version of the FastFlow unbandeau lock-free buffer.

## Build
The library building is not straightforward, because it relies on the quite new cross language linking time optimization of LLVM ([more](http://blog.llvm.org/2019/09/closing-gap-cross-language-lto-between.html)).

The building process require that the LLVM version of `clang` and `lld` match the one of `rustc`. I currently use `clang` version 8.0.0 with `rustc` version 1.37.0 that both have the LLVM 8. For the complete compatibility list see the table [here](https://doc.rust-lang.org/rustc/linker-plugin-lto.html#toolchain-compatibility)

### Dependency 
- [FastFlow](https://github.com/fastflow/fastflow)
- clang
- lld

Ubuntu 19 comes with LLVM 8 on the main repository, thus just:
```
sudo apt install clang-8 lld-8
``` 
In other Ubuntu/Debian version use this external [repository](https://apt.llvm.org/).

### Build guide

Download the FastFlow library in the home directory
```
cd ~
git clone https://github.com/fastflow/fastflow.git
```

Download the `ff_buffer` repository and cd inside 
```
git clone https://github.com/lucarin91/ff_buffer.git
cd ff_buffer
```

Than, using cargo it is possible to build the library, run the example or execute the benchmarks.
```
cargo build --release
cargo run --example simple
cargo bench
```

# Licence
MIT licence