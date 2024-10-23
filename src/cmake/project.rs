use tracing;

use crate::cli;

pub fn gen(options: &cli::commands::scan::ScanOptions) {
    let mut args = vec!["-S", &options.project_dir, "-B", &options.target_dir];

    if options.shared_lib {
        args.push("-D BUILD_SHARED_LIBS=1");
    }

    tracing::info!(command = "cmake", args = args.join(" "));

    std::process::Command::new("cmake")
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .unwrap();
}
