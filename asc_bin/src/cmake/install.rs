use crate::{
    cli,
    cmake::project::default_vcpkg_triplet,
    config::{self, relative_paths},
    util,
};

pub fn exec(options: &cli::commands::scan::ScanOptions, prefix: &str) {
    let install_prefix = format!("{}/{}", prefix, default_vcpkg_triplet());
    let args = vec![
        "--install",
        &options.target_dir,
        "--config",
        options.cmake_config.as_ref(),
        "--prefix",
        &install_prefix,
    ];
    let output = util::shell::run("cmake", &args, true, false, false).unwrap();

    let stdout: String = String::from_utf8_lossy(&output.stdout).to_string();
    println!("{}", &stdout);

    let mut data = config::project::InstalledFiles::default();
    data.path = relative_paths::ASC_PROJECT_INSTALLED_FILES_TOML_PATH.to_string();
    data.prefix = install_prefix;
    for line in stdout.split("\n") {
        let path = line
            .replace("-- Installing: ", "")
            .replace("-- Up-to-date:", "")
            .trim()
            .to_string();
        if !path.is_empty() {
            data.files.push(path);
        }
    }
    data.dump(true, false);
}
