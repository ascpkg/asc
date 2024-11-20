use directories;

fn main() {
    // build ast lib
    let library_name = "ast";
    let paths = PathBuilder::new(library_name);
    let library_path = paths.library_path();
    if std::fs::metadata(&library_path).is_err() {
        let vcpkg = VcpkgConfig::find();
        vcpkg.build_library(
            &format!(".{}", "asc"),
            &format!("{}/{}", paths.cargo_manifest_dir, library_name),
            "target",
            &paths.profile,
        );
        if std::fs::metadata(&library_path).is_err() {
            panic!("build ast error, library_path: {library_path}");
        }
    }

    // search lib in ast target dir
    println!("cargo:rustc-link-search={}", paths.library_dir());

    // search lib in vcpkg installed dir
    let triplet = get_default_vcpkg_triplet();
    println!("cargo:rustc-link-search={}", paths.vcpkg_lib_path(&triplet));

    // link ast
    println!("cargo:rustc-link-lib={}", library_name);
    // link fmt
    if paths.profile == "Release" {
        println!("cargo:rustc-link-lib=fmt");
    } else {
        println!("cargo:rustc-link-lib=fmtd");
    }
    // link libclang
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=libclang");
    } else {
        println!("cargo:rustc-link-lib=clang");
    }
    if cfg!(target_os = "windows") {
        // link libcmt
        println!(
            "cargo:rustc-link-lib={}",
            if paths.profile == "Release" {
                "libcmt"
            } else {
                "libcmtd"
            }
        );
    } else {
        if let Ok(text) = std::fs::read_to_string(paths.config_h_path()) {
            for line in text.split("\n") {
                if line.contains("#define HAVE_STD_CXX_LIB 1") {
                    // link stdc++
                    println!("cargo:rustc-link-lib=stdc++");
                }
                if line.contains("#define HAVE_CXX_FILESYSTEM_LIB 1") {
                    // link stdc++fs
                    println!("cargo:rustc-link-lib=stdc++fs");
                }
            }
        }
    }

    // generate bindings
    println!("cargo:rerun-if-changed={}", paths.header_path());
    if std::fs::metadata(paths.header_path()).is_err() {
        std::env::set_var("LIBCLANG_PATH", paths.vcpkg_bin_path(&triplet));
        bindgen::Builder::default()
            .header(paths.header_path())
            .generate()
            .unwrap()
            .write_to_file(paths.bindings_path())
            .unwrap();
    }
}

mod platform {
    pub static OS_MAP: [(&str, &str); 3] = [
        ("windows", "windows-static"),
        ("macos", "osx"),
        ("linux", "linux"),
    ];

    pub static ARCH_MAP: [(&str, &str); 8] = [
        ("x86", "x86"),
        ("i386", "x86"),
        ("AMD64", "x64"),
        ("x86_64", "x64"),
        ("arm", "arm"),
        ("armv7l", "arm"),
        ("arm64", "arm64"),
        ("aarch64", "arm64"),
    ];

    pub fn get_lib_extension() -> &'static str {
        if cfg!(target_os = "windows") {
            ".lib"
        } else {
            ".a"
        }
    }
}

struct PathBuilder {
    cargo_manifest_dir: String,
    library_name: String,
    profile: String,
}

impl PathBuilder {
    fn new(library_name: &str) -> Self {
        let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or(format!(
                "{}/asc_bin",
                std::env::current_dir().unwrap().to_str().unwrap()
            ))
            .replace(r"\", "/");
        let profile = Self::get_profile();

        Self {
            cargo_manifest_dir,
            library_name: library_name.to_string(),
            profile,
        }
    }

    fn get_profile() -> String {
        let profile = std::env::var("PROFILE").unwrap_or(String::from("debug"));
        format!(
            "{}{}",
            profile.to_uppercase().chars().next().unwrap(),
            &profile[1..]
        )
    }

    fn header_path(&self) -> String {
        format!(
            "{}/{}/src/lib.h",
            self.cargo_manifest_dir, self.library_name
        )
    }

    fn config_h_path(&self) -> String {
        format!(
            "{}/{}/target/{}/config.h",
            self.cargo_manifest_dir, self.library_name, self.library_name
        )
    }

    fn library_dir(&self) -> String {
        if cfg!(target_os = "windows") {
            format!(
                "{}/{}/target/{}/{}",
                self.cargo_manifest_dir, self.library_name, self.library_name, self.profile
            )
        } else {
            format!(
                "{}/{}/target/{}",
                self.cargo_manifest_dir, self.library_name, self.library_name
            )
        }
    }

    fn bindings_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(format!(
            "{}/src/clang/{}.rs",
            self.cargo_manifest_dir, self.library_name
        ))
    }

    fn library_path(&self) -> String {
        format!(
            "{}/{}{}",
            self.library_dir(),
            if cfg!(target_os = "windows") { self.library_name.clone() } else { format!("lib{}", self.library_name) },
            platform::get_lib_extension()
        )
    }

    fn vcpkg_lib_path(&self, triplet: &str) -> String {
        let subpath = if self.profile == "Release" {
            "lib"
        } else {
            "debug/lib"
        };
        format!(
            "{}/{}/target/vcpkg_installed/{}/{}",
            self.cargo_manifest_dir, self.library_name, triplet, subpath
        )
    }

    fn vcpkg_bin_path(&self, triplet: &str) -> String {
        let subpath = if self.profile == "Release" {
            "bin"
        } else {
            "debug/bin"
        };
        format!(
            "{}/{}/target/vcpkg_installed/{}/{}",
            self.cargo_manifest_dir, self.library_name, triplet, subpath
        )
    }
}

struct VcpkgConfig {
    cmake_path: String,
    downloads_path: String,
    binary_cache_path: String,
}

impl VcpkgConfig {
    fn find() -> Self {
        let (cmake_path, downloads_path, binary_cache_path) = find_vcpkg_cmake_path();
        Self {
            cmake_path,
            downloads_path,
            binary_cache_path,
        }
    }

    fn build_library(&self, project_dir: &str, source_dir: &str, target_dir: &str, profile: &str) {
        if self.cmake_path.is_empty() {
            return;
        }

        let triplet = get_default_vcpkg_triplet();
        if triplet.is_empty() {
            return;
        }

        let mut envs = std::collections::HashMap::new();
        if !self.downloads_path.is_empty() {
            envs.insert("VCPKG_DOWNLOADS", &self.downloads_path);
        }
        if !self.binary_cache_path.is_empty() {
            envs.insert("VCPKG_DEFAULT_BINARY_CACHE", &self.binary_cache_path);
        }
        println!("{:#?}", envs);

        // generate
        let mut args = vec![
            String::from("-S"),
            format!("{source_dir}/{project_dir}"),
            String::from("-B"),
            format!("{source_dir}/{target_dir}"),
            String::from("-D"),
            format!("CMAKE_TOOLCHAIN_FILE={}", self.cmake_path),
            String::from("-D"),
            format!("VCPKG_TARGET_TRIPLET={triplet}"),
            String::from("-D"),
            format!("VCPKG_HOST_TRIPLET={triplet}"),
        ];
        let output = std::process::Command::new("cmake")
            .args(&args)
            .envs(&envs)
            .current_dir(source_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
            .unwrap();
        println!(
            "@{source_dir}@ cmake {} -> {}",
            args.join(" "),
            output.status.code().unwrap(),
        );
        if !output.status.success() {
            println!(
                "stdout: {}\nstderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stdout)
            );
        }

        // build
        args = vec![
            String::from("--build"),
            format!("{source_dir}/{target_dir}"),
            String::from("--config"),
            profile.to_string(),
        ];
        let output = std::process::Command::new("cmake")
            .args(&args)
            .envs(&envs)
            .current_dir(source_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
            .unwrap();
        println!(
            "@{source_dir}@ cmake {} -> {}",
            args.join(" "),
            output.status.code().unwrap(),
        );
        if !output.status.success() {
            println!(
                "stdout: {}\nstderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stdout)
            );
        }
    }
}

fn get_default_vcpkg_triplet() -> String {
    let arch_map = std::collections::HashMap::from(platform::ARCH_MAP);
    let os_map = std::collections::HashMap::from(platform::OS_MAP);
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    match (arch_map.get(arch), os_map.get(os)) {
        (Some(a), Some(o)) => format!("{a}-{o}"),
        (None, _) => {
            panic!("unsupported arch {arch}");
        }
        (_, None) => {
            panic!("unsupported os {os}");
        }
    }
}

fn find_vcpkg_cmake_path() -> (String, String, String) {
    let vcpkg_cmake_postfix = "scripts/buildsystems/vcpkg.cmake";

    // from asc global vcpkg.toml
    if let Some(dir) = directories::ProjectDirs::from("", "", "asc") {
        let asc_toml_path = format!(
            "{}/vcpkg.toml",
            dir.config_dir().to_str().unwrap().replace(r"\", "/")
        );
        let mut vcpkg_cmake_path = String::new();
        let mut vcpkg_downloads_path = String::new();
        let mut vcpkg_binary_cache_path = String::new();
        if let Ok(metadata) = std::fs::metadata(&asc_toml_path) {
            if metadata.is_file() {
                let asc_toml_path = std::fs::read_to_string(&asc_toml_path).unwrap();
                for line in asc_toml_path.split("\n") {
                    if line.starts_with("directory = ") {
                        if let Some((_, vcpkg_root_dir)) = line.split_once(" = ") {
                            let path = format!(
                                "{}/{vcpkg_cmake_postfix}",
                                vcpkg_root_dir.replace("\"", "").trim()
                            );
                            if let Ok(metadata) = std::fs::metadata(&path) {
                                if metadata.is_file() {
                                    vcpkg_cmake_path = path;
                                    println!("found {vcpkg_cmake_path}");
                                }
                            }
                        }
                    } else if line.starts_with("env_downloads =") {
                        if let Some((_, path)) = line.split_once(" = ") {
                            vcpkg_downloads_path = path.replace("\"", "");
                        }
                    } else if line.starts_with("env_default_binary_cache =") {
                        if let Some((_, path)) = line.split_once(" = ") {
                            vcpkg_binary_cache_path = path.replace("\"", "");
                        }
                    }
                }
            }
        }

        return (
            vcpkg_cmake_path,
            vcpkg_downloads_path,
            vcpkg_binary_cache_path,
        );
    }

    // from PATH env
    if let Some(vcpkg_root_dir) = find_executable_dir_in_path("vcpkg") {
        let vcpkg_cmake_path = format!("{vcpkg_root_dir}/{vcpkg_cmake_postfix}");
        if let Ok(metadata) = std::fs::metadata(&vcpkg_cmake_path) {
            if metadata.is_file() {
                println!("found {vcpkg_cmake_path}");
                return (vcpkg_cmake_path, String::new(), String::new());
            }
        }
    }

    // from VCPKG_ROOT env
    if let Ok(vcpkg_root_dir) = std::env::var("VCPKG_ROOT") {
        if let Ok(metadata) = std::fs::metadata(&vcpkg_root_dir) {
            if metadata.is_dir() {
                let vcpkg_cmake_path: String = format!("{vcpkg_root_dir}/{vcpkg_cmake_postfix}");
                if let Ok(metadata) = std::fs::metadata(&vcpkg_root_dir) {
                    if metadata.is_file() {
                        println!("found {vcpkg_cmake_path}");
                        return (vcpkg_cmake_path, String::new(), String::new());
                    }
                }
            }
        }
    }

    panic!("missing {vcpkg_cmake_postfix}, add vcpkg executable to PATH, set VCPKG_ROOT_DIR, or use asc vcpkg set/update to manage vcpkg");
}

fn find_executable_dir_in_path(executable_name: &str) -> Option<String> {
    let env_path = std::env::var("PATH").unwrap();
    let partition = if cfg!(target_os = "windows") {
        ";"
    } else {
        ":"
    };

    for path in env_path.split(partition) {
        let full_path = std::path::PathBuf::from(path).join(if cfg!(target_os = "windows") {
            executable_name.to_string() + ".exe"
        } else {
            executable_name.to_string()
        });

        if std::fs::metadata(&full_path).is_ok() && is_executable(&full_path) {
            return Some(path.replace(r"\", "/"));
        }
    }

    None
}

fn is_executable(path: &std::path::PathBuf) -> bool {
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::fs::MetadataExt;
        if let Ok(metadata) = std::fs::metadata(path) {
            return metadata.mode() & 0o111 != 0;
        }
    }

    #[cfg(target_family = "windows")]
    {
        if path.with_extension(".exe").exists() {
            return true;
        }
    }

    false
}
