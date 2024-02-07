#!/bin/bash

set -euo pipefail
cd $(dirname $0)/

rm -rf dist
mkdir -p dist

cargo build --target aarch64-apple-darwin --release
cp ./target/aarch64-apple-darwin/release/filepicker dist/filepicker-mac-arm64

cargo build --target x86_64-apple-darwin --release
cp ./target/x86_64-apple-darwin/release/filepicker dist/filepicker-mac-x86_64

cross build --target x86_64-unknown-linux-gnu --release
cp ./target/x86_64-unknown-linux-gnu/release/filepicker dist/filepicker-linux-x86_64

cross build --target i686-unknown-linux-gnu --release
cp ./target/i686-unknown-linux-gnu/release/filepicker dist/filepicker-linux-i686

cross build --target aarch64-unknown-linux-gnu --release
cp ./target/aarch64-unknown-linux-gnu/release/filepicker dist/filepicker-linux-aarch64

cross build --target x86_64-pc-windows-gnu --release
cp ./target/x86_64-pc-windows-gnu/release/filepicker.exe dist/filepicker-windows-x86_64.exe

cross build --target i686-pc-windows-gnu --release
cp ./target/i686-pc-windows-gnu/release/filepicker.exe dist/filepicker-windows-i686.exe
