## Installing Rust

---
See [Rust Installation guide](https://www.rust-lang.org/tools/install)

If you already have Rust installed: `$ rustup update stable`

Or install Rust if it is not installed:

### On macOS, Linux, or another Unix-like OS
`$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Add Rust in your PATH:
`export PATH="$HOME/.cargo/bin:$PATH"`

Confirm the installed Rust version 1.59.0

`$ rustc --version`

> NOTE: Rust 1.60.0 has problem compiling substrate v0.9.18; Use Rust 1.59.0 instread.

 
### On Windows
download and run [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)

