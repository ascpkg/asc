fn main() {
    println!("cargo:rerun-if-changed=src/lib.h");
    println!("cargo:rerun-if-changed=src/lib.c");

    cc::Build::new()
        .file("src/lib.c")
        .include("src/")
        .compile("c_source_parser_ffi");
}
