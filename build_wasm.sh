#!/bin/bash
cargo build --profile wasm-release --target wasm32-unknown-unknown
wasm-bindgen --out-name angel-td-wasm --out-dir wasm/target --target web target/wasm32-unknown-unknown/wasm-release/angel-td.wasm
