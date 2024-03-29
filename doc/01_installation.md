# Installation

## Rust

In order to install Rust (including the compiler and package manager `cargo`) follow the instructions there: [Install Rust](https://www.rust-lang.org/tools/install).

## IDE

In this workshop, we use [Visual Studio Code](https://code.visualstudio.com/) as IDE as well as the extensions

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) (??)

## Embedded Rust

Follow the installation instructions here [`rp-rs/rp-hal` *Getting Started*](https://github.com/rp-rs/rp-hal?tab=readme-ov-file#getting-started) to get ready:

```sh
rustup self update
rustup update stable
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs --locked
cargo install probe-rs --features cli --locked
```

In order to be able to use the [Project template for the `rp2040-hal`](https://github.com/rp-rs/rp2040-project-template) make sure, you have installed all development dependencies mentioned there

```sh
cargo install flip-link
```

and you have [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) available:

```sh
cargo install cargo-generate
```

### Troubleshooting

- **Problem**

  ```sh
  error: could not find system library 'libudev' required by the 'libudev-sys' crate
  Package libudev was not found in the pkg-config search path.
  ```

  **Possible solution**

  ```sh
  sudo apt install libudev-dev
  ```
