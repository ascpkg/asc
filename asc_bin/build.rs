fn main() {
    // build ast lib
    let paths = PathBuilder::new("ast");
    println!("cargo:rerun-if-changed={}", paths.lib_ast_header_path());
    println!("cargo:rerun-if-changed={}", paths.lib_ast_source_path());
    let library_path = paths.library_path();
    if std::fs::metadata(&library_path).is_err() {
        let ast_dir = paths.ast_dir_path();
        let source_dir = format!("{ast_dir}/{}", paths.asc_dir_name());
        let profile = &paths.profile;

        // generate
        let mut args = vec![
            String::from("-S"),
            source_dir.clone(),
            String::from("-B"),
            paths.target_dir_path(),
        ];
        let output = std::process::Command::new("cmake")
            .args(&args)
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
            paths.target_dir_path(),
            String::from("--config"),
            profile.to_string(),
        ];
        let output = std::process::Command::new("cmake")
            .args(&args)
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
        if std::fs::metadata(&library_path).is_err() {
            panic!("build ast error, library_path: {library_path}");
        }
    }

    // search lib in ast target dir
    println!("cargo:rustc-link-search={}", paths.library_dir());
    println!("cargo:rustc-link-search={}", paths.target_dir_path());

    // link ast
    println!("cargo:rustc-link-lib={}", paths.library_name);
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
            if text.contains("#define HAVE_CXX_LIBRARY 1") {
                // link c++
                println!("cargo:rustc-link-lib=c++");
            } else {
                if text.contains("#define HAVE_STD_CXX_LIBRARY 1") {
                    // link stdc++
                    println!("cargo:rustc-link-lib=stdc++");
                }
            }
            if text.contains("#undef HAVE_STD_CXX_FS_LIBRARY") {
                // link stdc++fs
                println!("cargo:rustc-link-lib=stdc++fs");
            }
        }
    }

    if !cfg!(target_os = "windows") {
        // add executable directory to deps search paths
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
        } else {
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        }
    }

    // generate bindings
    if std::fs::metadata(paths.lib_ast_header_path()).is_err() {
        bindgen::Builder::default()
            .header(paths.lib_ast_header_path())
            .generate()
            .unwrap()
            .write_to_file(paths.bindings_path())
            .unwrap();
    }
}

mod platform {
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

    fn asc_dir_name(&self) -> String {
        String::from(".asc")
    }

    fn ast_dir_path(&self) -> String {
        format!("{}/{}", self.cargo_manifest_dir, self.library_name)
    }

    fn target_dir_name(&self) -> String {
        String::from("target")
    }

    fn target_dir_path(&self) -> String {
        format!(
            "{}/{}/{}",
            self.cargo_manifest_dir,
            self.library_name,
            self.target_dir_name()
        )
    }

    fn lib_ast_header_path(&self) -> String {
        format!(
            "{}/{}/src/lib.h",
            self.cargo_manifest_dir, self.library_name
        )
    }

    fn lib_ast_source_path(&self) -> String {
        format!(
            "{}/{}/src/lib.cpp",
            self.cargo_manifest_dir, self.library_name
        )
    }

    fn config_h_path(&self) -> String {
        format!(
            "{}/{}/{}/{}/config.h",
            self.cargo_manifest_dir,
            self.library_name,
            self.target_dir_name(),
            self.library_name
        )
    }

    fn library_dir(&self) -> String {
        if cfg!(target_os = "windows") {
            format!(
                "{}/{}/{}/{}/{}",
                self.cargo_manifest_dir,
                self.library_name,
                self.target_dir_name(),
                self.library_name,
                self.profile
            )
        } else {
            format!(
                "{}/{}/{}/{}",
                self.cargo_manifest_dir,
                self.library_name,
                self.target_dir_name(),
                self.library_name
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
            if cfg!(target_os = "windows") {
                self.library_name.clone()
            } else {
                format!("lib{}", self.library_name)
            },
            platform::get_lib_extension()
        )
    }
}
