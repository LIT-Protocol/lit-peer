#!/bin/bash

# build for apple silicon
cargo build --release
mkdir -p release/mac-apple-silicon
cp target/release/lit-recovery release/mac-apple-silicon/

# build for intel
readonly arch="$(uname -m)"
if [ $arch == "arm64" ]; then
    rustup target add x86_64-apple-darwin
fi

rustup target add x86_64-apple-darwin
cargo build --target x86_64-apple-darwin --release
mkdir -p release/mac-intel-silicon
cp target/x86_64-apple-darwin/release/lit-recovery release/mac-intel-silicon/