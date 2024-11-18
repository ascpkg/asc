use crate::{cli, util};

pub fn exec(options: &cli::commands::scan::ScanOptions) {
    let mut args = vec![
        "--build",
        &options.target_dir,
        "--config",
        options.cmake_config.as_ref(),
    ];
    if !options.project.is_empty() {
        args.extend(vec!["--target", &options.project]);
    }
    util::shell::run("cmake", &args, ".", false, false, false).unwrap();
}
