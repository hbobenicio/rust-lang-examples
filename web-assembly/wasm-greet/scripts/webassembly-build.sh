#!/bin/sh

set -ex

# Build the `wasm_greet.wasm` file using Cargo/rustc
cargo +nightly build --target wasm32-unknown-unknown

mkdir -p src/

wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_greet.wasm --out-dir src/
