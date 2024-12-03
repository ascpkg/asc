static RPATH_EXE_DIR_LINUX: &str = "$ORIGIN";
static RPATH_EXE_DIR_MACOS: &str = "@executable_path";

fn main() {
    // set runtime library search paths
    if !cfg!(target_os = "windows") {
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{RPATH_EXE_DIR_MACOS}");
        } else {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{RPATH_EXE_DIR_LINUX}");
        }
    }
}
