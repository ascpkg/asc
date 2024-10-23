use crate::{cli, config};

pub fn exec(options: &cli::commands::scan::ScanOptions, prefix: &str) {
    let args = vec![
        "--install",
        &options.target_dir,
        "--config",
        options.cmake_config.as_ref(),
        "--prefix",
        prefix,
    ];

    tracing::info!(command = "cmake", args = args.join(" "));

    let stdout_pipe = std::process::Stdio::piped();
    let output = std::process::Command::new("cmake")
        .args(args)
        .stdout(stdout_pipe)
        .stderr(std::process::Stdio::inherit())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    println!("{}", &stdout);

    let mut installed_files = vec![];
    for line in stdout.split("\n") {
        installed_files.push(
            line.replace("-- Installing: ", "")
                .replace("-- Up-to-date:", "")
                .trim()
                .to_string(),
        );
    }
    std::fs::write(
        config::path::INSTALL_FILES_PATH,
        installed_files.join("\n").as_bytes(),
    )
    .unwrap()
}
