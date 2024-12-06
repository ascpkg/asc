#!/bin/bash

export HTTP_PROXY="http://172.26.240.1:10809"
export HTTPS_PROXY="http://172.26.240.1:10809"
export NO_PROXY="localhost,127.0.0.1,172.26.240.1:10809"

# https://github.com/ascpkg/asc/releases/tag/zig-0.13.0-cf90dfd-20240607
export PATH=/opt/zig:$PATH
# https://github.com/ascpkg/asc/releases/tag/MacOSX11.3.sdk
export SDKROOT=/opt/MacOSX11.3.sdk

cargo install cargo-zigbuild

rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-pc-windows-gnullvm
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

cargo zigbuild --release --target x86_64-pc-windows-gnu
cargo zigbuild --release --target aarch64-pc-windows-gnullvm  # not working
cargo zigbuild --release --target x86_64-apple-darwin
cargo zigbuild --release --target aarch64-apple-darwin
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.17
cargo zigbuild --release --target aarch64-unknown-linux-gnu.2.17

