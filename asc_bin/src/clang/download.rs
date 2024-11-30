use reqwest;
use zstd;

use crate::config;

pub fn download_lib_clang_if_not_exists() -> String {
    let name = "libclang";
    let version = "13.0.0";
    let tag = "libclang-13.0-d7b669b-20210915";
    let url_prefix = "https://github.com/ascpkg/asc/releases/download";

    let arch = match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        name => {
            panic!("unsupported arch {name}");
        }
    };

    let lib_dir = config::system_paths::DataPath::lib_clang_dir();
    let (url, lib_path) = if cfg!(target_os = "windows") {
        let file_name = format!("{name}-{version}-{arch}.dll");
        (
            format!("{url_prefix}/{tag}/{file_name}.zst"),
            format!("{lib_dir}/{file_name}"),
        )
    } else if cfg!(target_os = "macos") {
        let file_name = format!("{name}-{version}-{arch}.dylib");
        (
            format!("{url_prefix}/{tag}/{file_name}.zst"),
            format!("{lib_dir}/{file_name}"),
        )
    } else {
        let file_name = format!("{name}-{version}-{arch}.so");
        (
            format!("{url_prefix}/{tag}/{file_name}.zst"),
            format!("{lib_dir}/{file_name}"),
        )
    };
    let zst_path = format!("{lib_path}.zst");

    let info = format!("url: '{url}', lib_path: '{lib_path}'");

    // download if not exists or not file
    let meta = std::fs::metadata(&zst_path);
    if meta.is_err() || !meta.unwrap().is_file() {
        tracing::info!(message = "downloading", url = url);

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .expect(&format!("reqwest builder error, {info}"));
        let response = client
            .get(url)
            .send()
            .expect(&format!("reqwest download error, {info}"));
        let mut file = std::fs::File::create(&zst_path)
            .expect(&format!("std::fs::File::create error, {info}"));
        let content = response
            .bytes()
            .expect(&format!("reqwest read response error, {info}"));
        std::io::Write::write_all(&mut file, &content)
            .expect(&format!("std::fs::File::write_all error, {info}"));
    }

    // extract if not exists, not file
    let meta = std::fs::metadata(&lib_path);
    if meta.is_err() || !meta.unwrap().is_file() {
        tracing::info!(message = "extracting", zst = zst_path);

        let zst_file =
            std::fs::File::open(zst_path).expect(&format!("std::fs::File::open error, {info}"));
        let output_file = std::fs::File::create(&lib_path).expect(&format!(
            "std::fs::File::create({:#?}) error, {info}",
            lib_path
        ));
        zstd::stream::copy_decode(zst_file, output_file)
            .expect(&format!("zstd::stream::copy_decode error, {info}"));
    }

    return lib_path;
}
