use std::io::Write;

use directories;
use reqwest;
use zip;

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

    // download libclang if not exists
    download_libclang(&target_dir, &target_profile_dir);

    // set link library search paths
    println!("cargo:rustc-link-search={target_dir}");
    println!("cargo:rustc-link-search={target_profile_dir}");

    // link libraries
    println!("cargo:rustc-link-lib=c_source_parser_ffi");
    println!("cargo:rustc-link-lib=rs_container_ffi");
    println!("cargo:rustc-link-lib=libclang");

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

fn download_libclang(target_dir: &str, target_profile_dir: &str) {
    let name = "libclang";
    let version = "13.0.0";
    let tag = "libclang-13.0-d7b669b-20210915";
    let url_prefix = format!("https://github.com/ascpkg/asc/releases/download");

    let arch = match std::env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or(String::from("unknown"))
        .as_str()
    {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        name => {
            panic!("unsupported arch {name}");
        }
    };

    let (url, zip_path, lib_path, lib_arch_path) = if cfg!(target_os = "windows") {
        let zip_name = format!("{name}-{version}-{arch}.dll.zip");
        (
            format!("{url_prefix}/{tag}/{zip_name}"),
            format!("{target_dir}/{zip_name}"),
            format!("{target_profile_dir}/{name}.dll"),
            format!("{target_profile_dir}/{name}.dll.arch"),
        )
    } else if cfg!(target_os = "macos") {
        let zip_name = format!("{target_dir}/{name}-{version}-{arch}.dylib.zip");
        (
            format!("{url_prefix}/{tag}/{zip_name}"),
            format!("{target_dir}/{zip_name}"),
            format!("{target_profile_dir}/{name}.dylib"),
            format!("{target_profile_dir}/{name}.dylib.arch"),
        )
    } else {
        let zip_name = format!("{target_dir}/{name}-{version}-{arch}.so.zip");
        (
            format!("{url_prefix}/{tag}/{zip_name}"),
            format!("{target_dir}/{zip_name}"),
            format!("{target_profile_dir}/{name}.so"),
            format!("{target_profile_dir}/{name}.so.arch"),
        )
    };

    let info = format!("url: '{url}', zip_path: '{zip_path}', lib_path: '{lib_path}'");

    // download if not exists or not file
    let meta = std::fs::metadata(&zip_path);
    if meta.is_err() || !meta.unwrap().is_file() {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .expect(&format!("reqwest builder error, {info}"));
        let response = client
            .get(url)
            .send()
            .expect(&format!("reqwest download error, {info}"));
        let mut file = std::fs::File::create(&zip_path)
            .expect(&format!("std::fs::File::create error, {info}"));
        let content = response
            .bytes()
            .expect(&format!("reqwest read response error, {info}"));
        file.write_all(&content)
            .expect(&format!("std::fs::File::write_all error, {info}"));
    }

    // extract if not exists, not file, or arch mismatch
    let meta = std::fs::metadata(&lib_path);
    if meta.is_err()
        || !meta.unwrap().is_file()
        || arch
            != std::fs::read_to_string(&lib_arch_path)
                .unwrap_or_default()
                .as_str()
    {
        let zip_file =
            std::fs::File::open(zip_path).expect(&format!("std::fs::File::open error, {info}"));
        let mut archive =
            zip::ZipArchive::new(zip_file).expect(&format!("zip::ZipArchive::new error, {info}"));
        let output_dir = std::path::Path::new(target_profile_dir);
        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .expect(&format!("archive.by_index({i}) error, {info}"));
            let output_path = output_dir.join(file.name());
            if file.name().ends_with('/') {
                std::fs::create_dir_all(&output_path).expect(&format!(
                    "std::fs::create_dir_all({:#?}) error, {info}",
                    output_path
                ));
            } else {
                let mut output_file = std::fs::File::create(&output_path).expect(&format!(
                    "std::fs::File::create({:#?}) error, {info}",
                    output_path
                ));
                std::io::copy(&mut file, &mut output_file)
                    .expect(&format!("std::io::copy({:#?}) error, {info}", output_path));
            }
        }

        std::fs::write(&lib_arch_path, arch.as_bytes())
            .expect(&format!("std::fs::write({lib_arch_path}) error, {info}"));
    }
}
