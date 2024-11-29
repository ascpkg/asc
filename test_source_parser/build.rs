fn main() {
    let profile = std::env::var("PROFILE").unwrap_or(String::from("debug"));
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .unwrap_or(String::from("target"))
        .replace(r"\", "/");

    println!("cargo:rustc-link-search={target_dir}/");
    println!("cargo:rustc-link-search={target_dir}/{profile}");

    println!("cargo:rustc-link-lib=c_clang_parser_ffi");
    println!("cargo:rustc-link-lib=rs_container_to_c");
    println!("cargo:rustc-link-lib=libclang");
}
