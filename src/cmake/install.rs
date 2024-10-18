use crate::util;

pub fn run(options: &util::cli::CommandLines) {
    let args = vec![
        "--install",
        &options.build_dir,
        "--config",
        if options.cmake_config == util::cli::CMakeConfigType::Debug {
            "Debug"
        } else {
            "Release"
        },
    ];

    tracing::info!(command = "cmake", args = args.join(" "));

    std::process::Command::new("cmake")
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .unwrap();
}
