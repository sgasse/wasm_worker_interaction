#!/bin/bash

set -e

OUT_DIR="./out"

mkdir "${OUT_DIR}"
./build_all.sh


DIRECTORIES=(
    'wasm_module_js_worker'
    'wasm_no_modules_js_worker'
    'wasm_no_modules_wasm_worker'
    'wasm_worker_interaction'
)

for directory in "${DIRECTORIES[@]}"
do
    mkdir -p "${OUT_DIR}/${directory}"
    cp -r "${directory}/www" "${OUT_DIR}/${directory}/"
done

for static_file in "index.html" "style.css"
do
    cp ${static_file} "${OUT_DIR}/"
done
