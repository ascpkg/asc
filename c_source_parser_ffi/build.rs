fn main() {
    println!("cargo:rerun-if-changed=src/clang.h");
    println!("cargo:rerun-if-changed=src/dylib.h");
    println!("cargo:rerun-if-changed=src/dylib.c");
    println!("cargo:rerun-if-changed=src/lib.h");
    println!("cargo:rerun-if-changed=src/lib.c");

    cc::Build::new()
        .file("src/dylib.c")
        .file("src/lib.c")
        .include("src/")
        .compile("c_source_parser_ffi");
}
