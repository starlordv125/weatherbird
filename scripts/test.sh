#!/bin/bash

# This builds rust binaries and prepares them for release
cargo install cross --git https://github.com/cross-rs/cross
cargo build --release
cross build --release --target=aarch64-unknown-linux-gnu