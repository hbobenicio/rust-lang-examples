# wasm-greet

This example project is the result of [this](https://hacks.mozilla.org/2018/04/javascript-to-rust-and-back-again-a-wasm-bindgen-tale/) excelent article, by Alex Crichton.

## Setup

```
# Adds wasm target to the nightly toolchain
rustup target add wasm32-unknown-unknown --toolchain nightly

# CLI used to prepare the wasm module for use as a JS Module
cargo install wasm-bindgen-cli
```

## Build

```
cargo +nightly build --target wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_greet.wasm --out-dir .
```
