fn main() {
    // Debug/Release
    let mut profile = std::env::var("PROFILE").unwrap();
    profile = format!("{}{}", profile.to_uppercase().chars().nth(0).unwrap(), &profile[1..]);

    // package Cargo.toml dir
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap().replace(r"\", "/");
    
    // path
    let library_name = "ast";
    let header_path = format!("{cargo_manifest_dir}/{library_name}/src/lib.h");
    let library_dir = format!("{cargo_manifest_dir}/{library_name}/target/{library_name}/{profile}");
    let bindings_path = std::path::PathBuf::from(format!("{cargo_manifest_dir}/src/clang/ast.rs"));

    // link
    println!("cargo:rustc-link-lib={}", library_name);
    println!("cargo:rustc-link-search={}", library_dir);
    
    // bind
    println!("cargo:rerun-if-changed={}", header_path);
    bindgen::Builder::default().header(header_path).generate().unwrap().write_to_file(bindings_path).unwrap();
}
