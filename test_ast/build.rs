fn main() {
    let profile = std::env::var("PROFILE").unwrap_or(String::from("debug"));
    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or(String::from("target"));
    
    println!("cargo:rustc-link-search={target_dir}/");
    println!("cargo:rustc-link-search={target_dir}/{profile}");

    println!("cargo:rustc-link-lib=ast_c");
    println!("cargo:rustc-link-lib=ast_r");
    println!("cargo:rustc-link-lib=libclang");
}
