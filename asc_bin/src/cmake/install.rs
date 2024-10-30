use crate::{cli, config, config::relative_paths, util};

pub fn exec(options: &cli::commands::scan::ScanOptions, prefix: &str) {
    let args = vec![
        "--install",
        &options.target_dir,
        "--config",
        options.cmake_config.as_ref(),
        "--prefix",
        prefix,
    ];
    let output = util::shell::run("cmake", &args, true, false, false).unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    println!("{}", &stdout);

    let mut data = config::project::InstalledFiles::default();
    data.path = relative_paths::ASC_TARGET_INSTALLED_FILES_TOML_PATH.to_string();
    data.prefix = prefix.to_string();
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
    data.dump(false);
}
