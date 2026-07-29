#!/bin/bash

# This builds rust binaries and prepares them for release
echo works
cargo install cross --git https://github.com/cross-rs/cross
rustup target add aarch64-unknown-linux-gnu
cargo build --release
cross build --release --target=aarch-unknown-linux-gnu