fn main() {
    // get paths
    let profile = std::env::var("PROFILE").unwrap_or(String::from("debug"));
    let package_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap()
        .replace(r"\", "/");
    let root_dir = std::path::Path::new(&package_dir)
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    let target_dir = format!("{root_dir}/target");
    let target_profile_dir = format!("{target_dir}/{profile}");

    // set link library search paths
    println!("cargo:rustc-link-search={target_dir}");
    println!("cargo:rustc-link-search={target_profile_dir}");

    // link libraries
    println!("cargo:rustc-link-lib=c_source_parser_ffi");
    println!("cargo:rustc-link-lib=rs_container_ffi");

    // link platform libraries
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=libcmt");
    }

    // set runtime library search paths
    if !cfg!(target_os = "windows") {
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
        } else {
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        }
    }
}
