#!/bin/bash

# Release setup
mkdir package
mkdir package/weatherbird-linux-amd64

# Man setup
gzip package/deb/weatherbird/usr/share/man/man1/weatherbird.1

# Debian setup
mkdir -p package/deb/weatherbird/DEBIAN/
mkdir -p package/deb/weatherbird/usr/share/bash-completion/completions/
touch package/deb/weatherbird/DEBIAN/control
mkdir -p package/deb/weatherbird/usr/bin
mkdir -p package/deb/weatherbird/usr/share/man/man1/

# RPM setup
cargo install cargo-generate-rpm
mkdir signed_rpm
