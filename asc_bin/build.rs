static ENV_KEY_PROFILE: &str = "PROFILE";
static ENV_VALUE_PROFILE_DEBUG: &str = "debug";
static ENV_VALUE_PROFILE_RELEASE: &str = "release";
static ENV_KEY_CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

static LIB_NAME_EXT: &str = ".rlib";
static LIB_RS_CONTAINER_FFI: &str = "librs_container_ffi";

static DEPS_DIR_NAME: &str = "deps";
static TARGET_DIR_NAME: &str = "target";
static CARGO_INSTALL_DIR_NAME_PREFIEX: &str = "cargo-install";

static RPATH_EXE_DIR_LINUX: &str = "$ORIGIN";
static RPATH_EXE_DIR_MACOS: &str = "@executable_path";

fn main() {
    // get paths
    let profile = std::env::var(ENV_KEY_PROFILE).unwrap_or(ENV_VALUE_PROFILE_DEBUG.to_string());
    let package_dir = std::env::var(ENV_KEY_CARGO_MANIFEST_DIR)
        .unwrap()
        .replace(r"\", "/");
    let workspace_dir = std::path::Path::new(&package_dir)
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    let target_profile_dir = format!("{workspace_dir}/{TARGET_DIR_NAME}/{profile}");

    // find latest rs_container_ffi lib
    let (lib_path, lib_search_dirs) = get_lib_rs_container_ffi_path(&target_profile_dir);
    if !lib_path.is_empty() {
        println!("cargo:rustc-link-arg={lib_path}");
    } else {
        panic!(
            "can not found any rs_container_ffi lib in {:#?}",
            lib_search_dirs
        )
    }

    // set runtime library search paths
    if !cfg!(target_os = "windows") {
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{RPATH_EXE_DIR_MACOS}");
        } else {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{RPATH_EXE_DIR_LINUX}");
        }
    }
}

fn get_lib_rs_container_ffi_path(target_profile_dir: &str) -> (String, Vec<String>) {
    let mut search_dir_paths = vec![
        target_profile_dir.to_string(),
        format!("{target_profile_dir}/{DEPS_DIR_NAME}"),
    ];
    // find cargo-install* dir path in user temp dir
    if let Ok(entries) = std::fs::read_dir(std::env::temp_dir()) {
        for entry in entries.filter_map(Result::ok) {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.starts_with(CARGO_INSTALL_DIR_NAME_PREFIEX) {
                    search_dir_paths.push(format!(
                        "{}/{ENV_VALUE_PROFILE_RELEASE}/{DEPS_DIR_NAME}",
                        entry.path().to_str().unwrap().replace(r"\", "/")
                    ));
                }
            }
        }
    }

    // find rs_container_ffi lib path
    let mut lib_file_paths = vec![];
    for path in &search_dir_paths {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with(LIB_RS_CONTAINER_FFI) && file_name.ends_with(LIB_NAME_EXT) {
                        lib_file_paths.push(entry.path().to_str().unwrap().replace(r"\", "/"));
                    }
                }
            }
        }
    }

    // find latest rs_container_ffi lib path
    let mut latest_lib_path = String::new();
    let mut latest_modified_ts = 0u128;
    for path in lib_file_paths {
        let ts = get_last_modified_time(&path);
        if ts >= latest_modified_ts {
            latest_modified_ts = ts;
            latest_lib_path = path;
        }
    }

    return (latest_lib_path, search_dir_paths);
}

fn get_last_modified_time(file_path: &str) -> u128 {
    if let Ok(meta) = std::fs::metadata(file_path) {
        if let Ok(sys_time) = meta.modified() {
            if let Ok(durcation) = sys_time.duration_since(std::time::UNIX_EPOCH) {
                return durcation.as_micros();
            }
        }
    }
    return 0;
}
