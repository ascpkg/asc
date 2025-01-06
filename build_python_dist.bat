@echo off

python cross_build.py --add_rust_targets=True --build_rust_targets=True --build_python_dist=True --target x86_64-pc-windows-msvc
python cross_build.py --add_rust_targets=True --build_rust_targets=True --build_python_dist=True --target aarch64-pc-windows-msvc
