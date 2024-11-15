use std::collections::HashMap;

use config_file_derives::ConfigFile;
use config_file_types;

use serde::{Deserialize, Serialize};

use crate::{
    cli::{self, commands::VcpkgArgs},
    config::system_paths,
    util,
};

static OS_MAP: [(&str, &str); 3] = [
    ("windows", "windows-static"),
    ("macos", "osx"),
    ("linux", "linux"),
];

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
    let vcpkg_conf = VcpkgArgs::load_or_default();
    let vcpkg_clone_dir = vcpkg_conf.directory.unwrap();

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

    util::shell::run("cmake", &args, ".", false, false, false).unwrap();
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("toml")]
pub struct ArchOsVcpkgTriplet {
    #[serde(skip)]
    pub path: String,

    pub os_map: HashMap<String, String>,
    pub arch_map: HashMap<String, String>,
}

pub fn default_vcpkg_triplet() -> String {
    let mut conf =
        ArchOsVcpkgTriplet::load(&system_paths::ConfigPath::arch_os_to_vcpkg_triplet(), true)
            .unwrap();
    if conf.os_map.is_empty() {
        for (k, v) in OS_MAP {
            conf.os_map.insert(k.to_string(), v.to_string());
        }
        conf.dump(true, false);
    }
    if conf.arch_map.is_empty() {
        for (k, v) in ARCH_MAP {
            conf.arch_map.insert(k.to_string(), v.to_string());
        }
        conf.dump(true, false);
    }

    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    if let Some(a) = conf.arch_map.get(arch) {
        if let Some(o) = conf.os_map.get(os) {
            return format!("{a}-{o}");
        } else {
            tracing::error!(message = "unsupported", os = os, config_path = conf.path);
        }
    } else {
        tracing::error!(
            message = "unsupported",
            arch = arch,
            config_path = conf.path
        );
    }

    return String::new();
}
