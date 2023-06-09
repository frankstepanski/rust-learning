## Install

### Rust and Cargo

https://doc.rust-lang.org/cargo/getting-started/installation.html

`curl https://sh.rustup.rs -sSf | sh`

Re-start terminal (and any IDE)

`rustup --version` => toolchain manager version

`rustc --version` => rust compiler version

`rustup update` => get udpates

## Setup

Can use [Visual Studio Code](https://code.visualstudio.com/docs/languages/rust) and [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension

Start a new package

`cargo new hello_world`
  - Creates a **hello_world** folder
  - Creates a Cargo.toml
  - Creates a /src folder 
  - Creates a main.rs
  - Create boilerplate code

  ```
  fn main() {
      println!("Hello, world!");
  }
  ```

### Build code

`cargo build`

Creates:
 - /target folder 
 - .rustc_info.json
 - CACHEDIR.TAG
 - and Cargo.lock

 ### Run code

 `cargo run`

or execute the hello_world.exe directly by typing:

`.\target\debug\hello_world`
