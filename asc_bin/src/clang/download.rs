use sha1::{Digest, Sha1};
use ureq;
use zstd;

use crate::config;

static LIB_CLANG_NAME: &str = "libclang";
static LIB_CLANG_VERSION: &str = "13.0.0";
static LIB_CLANG_TAG: &str = "libclang-13.0-d7b669b-20210915";
static LIB_CLANG_URL: &str = "https://github.com/ascpkg/asc/releases/download";

static LIB_CLANG_ZST_SHA1: [(&str, &str); 6] = [
    (
        "libclang-13.0.0-amd64.dll.zst",
        "c1940181707d78210ac7ff425f101eea405f7a06",
    ),
    (
        "libclang-13.0.0-arm64.dll.zst",
        "45c626fdfffaaebeb45f790a910f328b82b2c757",
    ),
    (
        "libclang-13.0.0-amd64.dylib.zst",
        "af824d696ceae7d7e37e349ed339f8bef34d15ed",
    ),
    (
        "libclang-13.0.0-arm64.dylib.zst",
        "c00f45dd1eb780526b75494d85c4e29a65f6a1ea",
    ),
    (
        "libclang-13.0.0-amd64.so.zst",
        "11a5ceb04d5eef73aafd3520b805f1636a0a7771",
    ),
    (
        "libclang-13.0.0-arm64.so.zst",
        "7fafe2bf8ba633efc7e8ae224a9af4384b5b4d63",
    ),
];

pub fn download_lib_clang_if_not_exists() -> String {
    let name = LIB_CLANG_NAME;
    let version = LIB_CLANG_VERSION;
    let tag = LIB_CLANG_TAG;
    let url_prefix = LIB_CLANG_URL;

    let arch = match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        name => {
            panic!("unsupported arch {name}");
        }
    };

    let lib_dir = config::system_paths::DataPath::lib_clang_dir();
    let (zst_name, url, lib_path) = if cfg!(target_os = "windows") {
        let file_name = format!("{name}-{version}-{arch}.dll");
        (
            format!("{file_name}.zst"),
            format!("{url_prefix}/{tag}/{file_name}.zst"),
            format!("{lib_dir}/{file_name}"),
        )
    } else if cfg!(target_os = "macos") {
        let file_name = format!("{name}-{version}-{arch}.dylib");
        (
            format!("{file_name}.zst"),
            format!("{url_prefix}/{tag}/{file_name}.zst"),
            format!("{lib_dir}/{file_name}"),
        )
    } else {
        let file_name = format!("{name}-{version}-{arch}.so");
        (
            format!("{file_name}.zst"),
            format!("{url_prefix}/{tag}/{file_name}.zst"),
            format!("{lib_dir}/{file_name}"),
        )
    };
    let zst_path = format!("{lib_path}.zst");

    let info = format!("url: '{url}', lib_path: '{lib_path}'");

    // download if not exists or not file or sha1 mismatch
    let zst_sha1 = std::collections::HashMap::from(LIB_CLANG_ZST_SHA1);
    let meta = std::fs::metadata(&zst_path);
    if meta.is_err()
        || !meta.as_ref().unwrap().is_file()
        || &calculate_sha1(&zst_path).as_str() != zst_sha1.get(zst_name.as_str()).unwrap_or(&"")
    {
        for _ in 0..3 {
            tracing::info!(message = "downloading", url = url);

            if meta.as_ref().is_ok() {
                let _ = std::fs::remove_file(&zst_path);
            }

            let agent = ureq::AgentBuilder::new()
                .try_proxy_from_env(true)
                .timeout_read(std::time::Duration::from_secs(15))
                .timeout_write(std::time::Duration::from_secs(5))
                .build();

            let response = agent
                .get(&url)
                .call()
                .expect(&format!("ureq::get error, {info}"));

            let mut zst_file = std::fs::File::create(&zst_path)
                .expect(&format!("std::fs::File::create error, {info}"));
            std::io::copy(&mut response.into_reader(), &mut zst_file)
                .expect(&format!("std::io::copy error, {info}"));

            let calculated_sha1 = calculate_sha1(&zst_path);
            if let Some(expected_sha1) = zst_sha1.get(zst_name.as_str()) {
                if &calculated_sha1.as_str() != expected_sha1 {
                    tracing::error!(
                        message = "sha1 mismatch",
                        expected = expected_sha1,
                        calculated = calculated_sha1
                    );
                    continue;
                }
            }
            break;
        }
    }
    let meta = std::fs::metadata(&zst_path);
    if meta.is_ok() && meta.unwrap().is_file() {
        let calculated_sha1 = calculate_sha1(&zst_path);
        if let Some(expected_sha1) = zst_sha1.get(zst_name.as_str()) {
            if &calculated_sha1.as_str() != expected_sha1 {
                tracing::error!(
                    message = "sha1 mismatch",
                    expected = expected_sha1,
                    calculated = calculated_sha1
                );
            }
        }
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

fn calculate_sha1(file_path: &str) -> String {
    let mut file = std::fs::File::open(file_path).unwrap();

    let mut hasher = Sha1::new();

    let mut buffer = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut buffer).unwrap();
    hasher.update(&buffer);

    let result = hasher.finalize();

    return format!("{:x}", result);
}
