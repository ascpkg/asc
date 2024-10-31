use std::collections::HashMap;

use crate::{
    cli::{self, commands::VcpkgArgs},
    config::{self, system_paths},
    util,
};

static ARCH_MAP: [(&str, &str); 8] = [
    ("x86", "x86"),
    ("i386", "x86"),
    ("AMD64", "x64"),
    ("x86_64", "x64"),
    ("arm", "arm"),
    ("armv7l", "arm"),
    ("arm64", "arm64"),
    ("aarch64", "arm64"),
];

pub fn gen(options: &cli::commands::scan::ScanOptions) {
    let vcpkg_clone_dir = VcpkgArgs::load(&config::system_paths::ConfigPath::vcpkg_toml(), true)
        .unwrap()
        .directory
        .unwrap_or(config::system_paths::DataPath::vcpkg_clone_dir());

    let cmake_toolchain_file = format!(
        "-D CMAKE_TOOLCHAIN_FILE={}",
        system_paths::DataPath::vcpkg_scripts_build_systems_cmake_path(&vcpkg_clone_dir)
    );
    let vcpkg_target_triplet = format!("-D VCPKG_TARGET_TRIPLET={}", default_vcpkg_triplet());
    let vcpkg_host_triplet = format!("-D VCPKG_HOST_TRIPLET={}", default_vcpkg_triplet());
    let mut args = vec![
        "-S",
        &options.project_dir,
        "-B",
        &options.target_dir,
        &cmake_toolchain_file,
        &vcpkg_target_triplet,
        &vcpkg_host_triplet,
    ];

    if options.shared_lib {
        args.push("-D BUILD_SHARED_LIBS=1");
    }

    util::shell::run("cmake", &args, false, false, false).unwrap();
}

pub fn default_vcpkg_triplet() -> String {
    let arch_map = HashMap::from(ARCH_MAP);

    let machine = std::env::consts::ARCH;
    let arch = match arch_map.get(machine) {
        Some(&arch) => arch,
        None => {
            tracing::error!("unsupported architecture: {}", machine);
            return String::new();
        }
    };

    let os = std::env::consts::OS;
    match os {
        "windows" => format!("{}-windows-static", arch),
        "macos" => format!("{}-osx", arch),
        "linux" => format!("{}-linux", arch),
        _ => {
            tracing::error!("unsupported os: {}", os);
            String::new()
        }
    }
}
