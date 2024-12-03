#!/bin/bash

export ENV_HTTP_PROXY="http://172.26.240.1:10809"
export ENV_HTTPS_PROXY="http://172.26.240.1:10809"
export ENV_NO_PROXY="localhost,127.0.0.1,172.26.240.1:10809"

# /lib/systemd/system/docker.service
# [Service]
# Environment="HTTP_PROXY=http://172.26.240.1:10809"
# Environment="HTTPS_PROXY=http://172.26.240.1:10809"
# Environment="NO_PROXY=localhost,127.0.0.1,172.26.240.1:10809"

docker run --env HTTP_PROXY=$ENV_HTTP_PROXY --env HTTPS_PROXY=$ENV_HTTPS_PROXY --env NO_PROXY=$ENV_NO_PROXY --rm -it -v $(pwd):/io -w /io messense/cargo-zigbuild cargo zigbuild --release --target x86_64-apple-darwin
docker run --env HTTP_PROXY=$ENV_HTTP_PROXY --env HTTPS_PROXY=$ENV_HTTPS_PROXY --env NO_PROXY=$ENV_NO_PROXY --rm -it -v $(pwd):/io -w /io messense/cargo-zigbuild cargo zigbuild --release --target aarch64-apple-darwin
docker run --env HTTP_PROXY=$ENV_HTTP_PROXY --env HTTPS_PROXY=$ENV_HTTPS_PROXY --env NO_PROXY=$ENV_NO_PROXY --rm -it -v $(pwd):/io -w /io messense/cargo-zigbuild cargo zigbuild --release --target x86_64-unknown-linux-musl
docker run --env HTTP_PROXY=$ENV_HTTP_PROXY --env HTTPS_PROXY=$ENV_HTTPS_PROXY --env NO_PROXY=$ENV_NO_PROXY --rm -it -v $(pwd):/io -w /io messense/cargo-zigbuild cargo zigbuild --release --target aarch64-unknown-linux-musl
docker run --env HTTP_PROXY=$ENV_HTTP_PROXY --env HTTPS_PROXY=$ENV_HTTPS_PROXY --env NO_PROXY=$ENV_NO_PROXY --rm -it -v $(pwd):/io -w /io messense/cargo-zigbuild cargo zigbuild --release --target x86_64-pc-windows-gnu
docker run --env HTTP_PROXY=$ENV_HTTP_PROXY --env HTTPS_PROXY=$ENV_HTTPS_PROXY --env NO_PROXY=$ENV_NO_PROXY --rm -it -v $(pwd):/io -w /io messense/cargo-zigbuild cargo zigbuild --release --target aarch64-pc-windows-gnullvm

