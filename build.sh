#! /usr/bin/env bash

# cargo build --release --target wasm32-unknown-unknown
# wasm-bindgen --out-dir ./out/ --target web ./target/

change_urls() {
    local file_path=$1
    if [ -f "$file_path" ]; then
        sed -i 's|="\(/[^"]*\)"|="./\1"|g' "$file_path"
        echo "URLs modified successfully."
    else
        echo "File not found."
    fi
}

rm -r dist/

trunk build --release

cp -r assets dist/assets

# change_urls "dist/index.html"

# wasm-opt -O -ol 100 -s 100 -o my_game.wasm my_game.wasm