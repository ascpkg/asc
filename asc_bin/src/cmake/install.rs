use crate::{
    cli,
    cmake::project::default_vcpkg_triplet,
    config::{self, relative_paths},
    dependency, pack, util,
};

pub fn exec(options: &cli::commands::scan::ScanOptions, prefix: &str, pack_cli: &str) {
    // run cmake --install
    let triplet = default_vcpkg_triplet();
    let install_prefix = format!("{prefix}/{triplet}");
    let args = vec![
        "--install",
        &options.target_dir,
        "--config",
        &options.cmake_config,
        "--prefix",
        &install_prefix,
    ];
    let output = util::shell::run("cmake", &args, ".", true, false, false).unwrap();

    let stdout: String = String::from_utf8_lossy(&output.stdout).to_string();
    println!("{}", &stdout);

    // save installed files
    let mut executable_and_dynamic_library_files = vec![];
    let mut data = config::project::InstalledFiles::default();
    data.path = relative_paths::ASC_PROJECT_INSTALLED_FILES_TOML_PATH.to_string();
    data.prefix = install_prefix.clone();
    for line in stdout.split("\n") {
        let path = line
            .replace("-- Installing: ", "")
            .replace("-- Up-to-date:", "")
            .replace(r"\", "/")
            .trim()
            .to_string();
        if !path.is_empty() {
            if util::fs::is_executable_or_dynamic_library(&path) {
                executable_and_dynamic_library_files.push(path.clone());
            }
            data.files.push(path);
        }
    }
    data.dump(true, false);

    // copy dependent libraries
    dependency::copy::copy_dependent_libraries(
        &install_prefix,
        options.cmake_config.to_lowercase(),
        &triplet,
        executable_and_dynamic_library_files,
    );

    // package files
    if !pack_cli.is_empty() {
        let name = if let Some(prj) = config::project::ProjectConfig::read_project_conf() {
            if let Some(pkg) = prj.package {
                pkg.name.clone()
            } else {
                util::fs::get_cwd_name()
            }
        } else {
            util::fs::get_cwd_name()
        };
        pack::make_package(
            &name,
            format!(
                "{install_prefix}/{}",
                config::relative_paths::VCPKG_BIN_DIR_NAME
            ),
            pack_cli,
        );
    }
}
