use crate::{cli, util};

pub fn exec(options: &cli::commands::scan::ScanOptions) {
    let args = vec![
        "--build",
        &options.target_dir,
        "--config",
        options.cmake_config.as_ref(),
    ];
    util::shell::run("cmake", &args, false, false).unwrap();
}
