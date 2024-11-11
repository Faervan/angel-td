#!/bin/bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name angel-td-wasm --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/angel-td.wasm
