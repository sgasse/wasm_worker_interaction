#!/bin/bash

set -e

DIRECTORIES=(
    'wasm_module_js_worker'
    'wasm_no_modules_js_worker'
    'wasm_no_modules_wasm_worker'
    'wasm_worker_interaction'
)

for directory in "${DIRECTORIES[@]}"
do
    echo "Building $directory..."
    cd $directory
    ./build.sh
    cd -
done

echo "All projects built successfully."