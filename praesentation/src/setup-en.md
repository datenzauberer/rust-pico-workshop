# Setup development environemt

The following setup description assumes that you use Ubuntu 22.04. 

## Ubuntu 22.04 setup

Please ensure and verify that your Ubuntu 22.04 is set up as described.

```sh
sudo apt install update
sudo apt install -y git
# curl needed for installing rust
sudo apt install -y curl
# libudev-dev needed for debugging
sudo apt install -y libudev-dev

# for cargo generate:
sudo apt install -y build-essential
sudo apt install -y pkg-config libssl-dev
```

## Visual Studio Code (VSCode)

Download vscode from
https://code.visualstudio.com/download
Install it, e.g. for ubuntu:

```sh
sudo apt install ~/Downloads/code_1.87.2-1709912201_amd64.deb
```

Install the extensions (from the command line):

```sh
# rust development
code --install-extension rust-lang.rust-analyzer
# debug rust code
code --install-extension vadimcn.vscode-lldb # on macOS/Linux
#code --install-extension ms-vscode.cpptools # on Windows
# for Debugging with probe-rs:
code --install-extension probe-rs.probe-rs-debugger
```


start `code` from terminal or via applications

## Install Rust

In order to install Rust (including the compiler and package manager `cargo`) follow the instructions there: [Install Rust](https://www.rust-lang.org/tools/install). For Ubuntu:
Execute the following installation command and do a "1 Standard installation".

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart your shell or source env (as described in the terminal, e.g. for bash`source $HOME/.cargo/env`).

## Verify that Rust is installed correctly

```sh
rustc --version
```

This command should return the version of the Rust compiler, `rustc`, that's currently installed on your system. For example, it might output something like `rustc 1.77.1 (7cf61ebde 2024-03-27)`, indicating the version number and the release date.

Cargo is Rust's package manager and build system. To check if Cargo is installed correctly and view its version, type:

```sh
cargo --version
```

This should return the version of Cargo installed on your system, similar to the Rust compiler version check.

## Embedded Rust

Install development dependencies:

```sh
cargo install cargo-generate
```

If it fails install packages as described in SMTODO: SECTION.

Follow the installation instructions here [`rp-rs/rp-hal` *Getting Started*](https://github.com/rp-rs/rp-hal?tab=readme-ov-file#getting-started) to get ready:

```sh
rustup self update
rustup update stable
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs --locked
cargo install probe-rs --features cli --locked
cargo install flip-link
```

If it fails install packages as described in SMTODO: SECTION.


**ATTENTION: Adjust the `/etc/udev/rules.d`** (as described in <https://probe.rs/docs/getting-started/probe-setup/#linux%3A-udev-rules>):

```sh
curl -o ~/Downloads/69-probe-rs.rules https://probe.rs/files/69-probe-rs.rules
sudo cp ~/Downloads/69-probe-rs.rules /etc/udev/rules.d
sudo udevadm control --reload
sudo udevadm trigger
```

## Optional: OpenOCD support

