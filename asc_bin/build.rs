static ENV_KEY_PROFILE: &str = "PROFILE";
static ENV_VALUE_PROFILE_DEBUG: &str = "debug";
static ENV_VALUE_PROFILE_RELEASE: &str = "release";
static ENV_KEY_CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

static LIB_RS_CONTAINER_FFI: &str = "rs_container_ffi";

static DEPS_DIR_NAME: &str = "deps";
static TARGET_DIR_NAME: &str = "target";
static CARGO_INSTALL_DIR_NAME_PREFIEX: &str = "cargo-install";

static LINUX_RPATH_EXE_DIR: &str = "$ORIGIN";
static MACOS_RPATH_EXE_DIR: &str = "@executable_path";

fn main() {
    // get paths
    let profile = std::env::var(ENV_KEY_PROFILE).unwrap_or(ENV_VALUE_PROFILE_DEBUG.to_string());
    let package_dir = std::env::var(ENV_KEY_CARGO_MANIFEST_DIR)
        .unwrap()
        .replace(r"\", "/");
    let root_dir = std::path::Path::new(&package_dir)
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    let target_dir = format!("{root_dir}/{TARGET_DIR_NAME}/{profile}");

    // find latest rs_container_ffi lib
    let (lib_dir, lib_name) = get_lib_rs_container_ffi_path(&target_dir);
    // set link library search paths
    println!("cargo:rustc-link-search={lib_dir}");
    // link libraries
    println!("cargo:rustc-link-lib=static={lib_name}");

    // set runtime library search paths
    if !cfg!(target_os = "windows") {
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{MACOS_RPATH_EXE_DIR}");
        } else {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{LINUX_RPATH_EXE_DIR}");
        }
    }
}

fn get_lib_rs_container_ffi_path(target_dir: &str) -> (String, String) {
    // find cargo-install* dir path in user temp dir
    let mut dir_paths = vec![];
    if let Ok(entries) = std::fs::read_dir(std::env::temp_dir()) {
        for entry in entries.filter_map(Result::ok) {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.starts_with(CARGO_INSTALL_DIR_NAME_PREFIEX) {
                    dir_paths.push(format!(
                        "{}/{ENV_VALUE_PROFILE_RELEASE}/{DEPS_DIR_NAME}",
                        entry.path().to_str().unwrap().replace(r"\", "/")
                    ));
                }
            }
        }
    }

    // find rs_container_ffi lib path
    let lib_file_ext = get_lib_file_ext();
    let mut file_paths = vec![format!("{target_dir}/{LIB_RS_CONTAINER_FFI}{lib_file_ext}")];
    for path in &dir_paths {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with(LIB_RS_CONTAINER_FFI) {
                        if file_name.ends_with(lib_file_ext) {
                            file_paths.push(entry.path().to_str().unwrap().replace(r"\", "/"));
                        }
                    }
                }
            }
        }
    }

    // find latest rs_container_ffi lib path
    let mut last_updated_lib_path = String::new();
    let mut last_updated_ts = 0u128;
    for path in file_paths {
        let ts = get_last_modified_time(&path);
        if ts > last_updated_ts {
            last_updated_ts = ts;
            last_updated_lib_path = path;
        }
    }

    // return rs_conatiner_ffi located dir path and file name
    if last_updated_lib_path.is_empty() {
        panic!("can not find lib rs_conatiner_ffi in both target dir ({target_dir}) and cargo-install* temp dir ({:#?})", dir_paths);
    }

    let path = std::path::Path::new(&last_updated_lib_path);
    let dir_path = path.parent().unwrap().to_str().unwrap().to_string();
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    return (dir_path, file_name.replace(lib_file_ext, ""));
}

fn get_lib_file_ext() -> &'static str {
    if cfg!(target_os = "windows") {
        return ".lib";
    } else {
        return ".a";
    }
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
