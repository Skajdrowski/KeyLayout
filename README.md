# KeyLayout
A lightweight Keyboard layout for minimal latency

![Untitled](https://github.com/user-attachments/assets/69e817b8-2c9d-4916-8d3f-9b2d1e9e73f5)


## Compilation

This application is written in Rust. In order to compile it, you need to
install the Rust compiler: [Install Rust](https://www.rust-lang.org/tools/install).

Afterwards install the desired target:
```sh
rustup target add x86_64-pc-windows-msvc --toolchain stable
```
For linux(X11):
```sh
rustup target add x86_64-unknown-linux-gnu --toolchain stable
```

Application can now be compiled:
```sh
cargo b --release
```

Binary is then available at:
```
target/release/
```
