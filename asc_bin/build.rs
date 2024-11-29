fn main() {
    let profile = std::env::var("PROFILE").unwrap_or(String::from("debug"));
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .unwrap_or(String::from("target"))
        .replace(r"\", "/");

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=libcmt");
    }

    println!("cargo:rustc-link-search={target_dir}/{profile}");

    println!("cargo:rustc-link-lib=c_source_parser_ffi");
    println!("cargo:rustc-link-lib=rs_container_ffi");
    println!("cargo:rustc-link-lib=libclang");

    if !cfg!(target_os = "windows") {
        // add executable directory to runtime library search paths
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
        } else {
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        }
    }
}
