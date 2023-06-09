#!/bin/bash

set -e

cd "$(dirname "$0")/.."

TARGET_DIR=target/wasm32-unknown-unknown/debug

cargo rustc -p wasm-blob-example --target wasm32-unknown-unknown -- -Clinker=./fuckedup-wasm-ld

read -r -a exports <$TARGET_DIR/deps/wasm_blob_example.exports

cargo r -p wasm-blob-bundler -- FOO:example/sample.txt $TARGET_DIR/foo_blob.o

wasm-ld $TARGET_DIR/foo_blob.o $TARGET_DIR/libwasm_blob_example.a "${exports[@]/#/--export=}" --no-entry -o bundled-example.wasm

echo "Check out bundled-example.wasm!"

