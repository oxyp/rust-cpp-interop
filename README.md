# Rust C++ Interoperability test

The main loop of this demo is written in Rust.  We use println for the demo but otherwise the code is suitable for a `no-std` environment.  The demo shows how to call the constructor on a fixed-size C++ object and call methods on that object.

## Preflight
Make sure that you have the C++ compiler `g++` installed, and the rust compiler `cargo`.  The latter may be found here: https://www.rust-lang.org/tools/install

## Steps
Create a static library from C++ code:
```
cd cpp_src
./make.sh
cd ..
```
Call the library from Rust:
```
cd rust_src
cargo run
```
