use crate::cli;

pub fn exec(options: &cli::commands::scan::ScanOptions) {
    let args = vec![
        "--build",
        &options.target_dir,
        "--config",
        options.cmake_config.as_ref(),
    ];

    tracing::info!(command = "cmake", args = args.join(" "));

    std::process::Command::new("cmake")
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .unwrap();
}
