use crate::{cli, config, util};

pub fn exec(options: &cli::commands::scan::ScanOptions, prefix: &str) {
    let args = vec![
        "--install",
        &options.target_dir,
        "--config",
        options.cmake_config.as_ref(),
        "--prefix",
        prefix,
    ];
    let output = util::shell::run("cmake", &args, true, false).unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    println!("{}", &stdout);

    let mut data = config::project::InstalledFiles::default();
    data.path = config::project::path::INSTALL_FILES_PATH.to_string();
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
