use crate::cli;

pub fn run(options: &cli::commands::scan::ScanOptions) {
    let args = vec![
        "--install",
        &options.build_dir,
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
