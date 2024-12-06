@echo off

set HTTP_PROXY="http://127.0.0.1:10809"
set HTTPS_PROXY="http://127.0.0.1:10809"
set NO_PROXY="localhost,127.0.0.1,127.0.0.1:10809"

REM https://github.com/ascpkg/asc/releases/tag/zig-0.13.0-cf90dfd-20240607
set PATH=C:\zig;%PATH%
REM https://github.com/ascpkg/asc/releases/tag/MacOSX11.3.sdk
set SDKROOT=C:\MacOSX11.3.sdk

REM https://learn.microsoft.com/en-us/visualstudio/install/workload-component-id-vs-community

cargo install cargo-zigbuild

rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target aarch64-pc-windows-msvc
cargo zigbuild --release --target x86_64-apple-darwin
cargo zigbuild --release --target aarch64-apple-darwin
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.17
cargo zigbuild --release --target aarch64-unknown-linux-gnu.2.17

