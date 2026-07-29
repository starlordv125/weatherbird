#!/bin/bash

# This builds rust binaries and prepares them for release
echo works
echo Installing cross
cargo install cross --git https://github.com/cross-rs/cross
echo Adding aarch64 target
rustup target add aarch64-unknown-linux-gnu
#cargo build --release
pwd
echo Starting cross
bash cross build --release --target=aarch-unknown-linux-gnu
