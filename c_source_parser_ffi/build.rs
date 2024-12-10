static SRC_DIR: &str = "src";
static CLANG_H: &str = "src/clang.h";
static DYLIB_H: &str = "src/dylib.h";
static DYLIB_C: &str = "src/dylib.c";
static LIB_H: &str = "src/lib.h";
static LIB_C: &str = "src/lib.c";

static C11_FLAG_MSVC: &str = "/std:c11";
static C99_FLAG_GCC_CLANG: &str = "-std=c99";

static ENV_TARGET_KEY: &str = "TARGET";
static ENV_TARGET_VALUE_WINDOWS_MSVC: &str = "-windows-msvc";

static ENV_KEY_CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

static CARGO_TOML_FILE_NAME: &str = "Cargo.toml";

fn main() {
    let manifest_path = std::env::var(ENV_KEY_CARGO_MANIFEST_DIR).unwrap();
    let cargo_toml_path = std::path::Path::new(&manifest_path).join(CARGO_TOML_FILE_NAME);

    let toml_text = std::fs::read_to_string(cargo_toml_path).unwrap();
    let toml_value: toml::Value = toml::de::from_str(&toml_text).unwrap();
    let package_name = toml_value
        .get("package")
        .unwrap()
        .get("name")
        .unwrap()
        .as_str()
        .unwrap();

    println!("cargo:rerun-if-changed={LIB_H}");
    println!("cargo:rerun-if-changed={LIB_C}");
    println!("cargo:rerun-if-changed={CLANG_H}");
    println!("cargo:rerun-if-changed={DYLIB_H}");
    println!("cargo:rerun-if-changed={DYLIB_C}");

    let compiler_flag = if std::env::var(ENV_TARGET_KEY)
        .unwrap_or(String::from(ENV_TARGET_VALUE_WINDOWS_MSVC))
        .contains(ENV_TARGET_VALUE_WINDOWS_MSVC)
    {
        C11_FLAG_MSVC
    } else {
        C99_FLAG_GCC_CLANG
    };

    cc::Build::new()
        .file(LIB_C)
        .file(DYLIB_C)
        .include(SRC_DIR)
        .flag(compiler_flag)
        .compile(package_name);
}
