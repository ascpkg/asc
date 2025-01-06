#!/bin/bash

python3 cross_build.py --prepare_mac_os_sdk=True --add_rust_targets=True --build_rust_targets=True --build_python_dist=True --target=x86_64-apple-darwin
python3 cross_build.py --prepare_mac_os_sdk=True --add_rust_targets=True --build_rust_targets=True --build_python_dist=True --target=aarch64-apple-darwin
python3 cross_build.py --add_rust_targets=True --build_rust_targets=True --build_python_dist=True --target=x86_64-unknown-linux-gnu
python3 cross_build.py --add_rust_targets=True --build_rust_targets=True --build_python_dist=True --target=aarch64-unknown-linux-gnu
