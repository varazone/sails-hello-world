## The **sails-hello-world** program

The program workspace includes the following packages:
- `sails-hello-world` is the package allowing to build WASM binary for the program and IDL file for it.  
  The package also includes integration tests for the program in the `tests` sub-folder
- `sails-hello-world-app` is the package containing business logic for the program represented by the `SailsHelloWorldService` structure.  
- `sails-hello-world-client` is the package containing the client for the program allowing to interact with it from another program, tests, or
  off-chain client.

## Dev Environment setup

Install rust stable toolchain with `wasm32v1-none` toolchain and `rust-src` component, assuming you have `rustup` installed:

```bash
rustup default stable
rustup target add wasm32v1-none
rustup component add rust-src
```

Alternatively you can skip manually run the above commands if you have `rust-toolchain.toml` file in the root of the repository with the following content:

```toml
[toolchain]
channel = "stable"
targets = ["wasm32v1-none"]
components = ["rust-src"]
profile = "default"
```

Or you can simply open the repository in GitHub Codespaces by clicking the button below, which will automatically setup the dev environment for you:

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/varazone/sails-hello-world/tree/main)

## Compile and test the program

Compile the program with the following command:

```bash
cargo build --release
```

After that, you will find the WASM binary in the `./target/wasm32-gear/release` sub-folder:

```
sails_hello_world.idl
sails_hello_world.opt.wasm
sails_hello_world.wasm
```

Run the unit tests and integration tests with the following command:

```
cargo test
```

## Deploy the program

[Gear IDEA](https://idea.gear-tech.io)
