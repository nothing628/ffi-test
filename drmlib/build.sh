#!/bin/sh

# Install wasmpack if not exists
cargo install wasm-pack

wasm-pack build --target web --release
