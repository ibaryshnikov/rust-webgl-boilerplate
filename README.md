# Rust WebGL boilerplate

Boilerplate to start a webgl project with `wasm-bindgen`


## Installation

In order to build the project `wasm-bindgen-cli` version should match the version from `Cargo.toml`

```bash
cargo install wasm-bindgen-cli --version 0.2.100
```


## Usage

```bash
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir pkg --target web ./target/wasm32-unknown-unknown/debug/webgl_boilerplate.wasm
# to get `http` util use `cargo install https`
http
```
