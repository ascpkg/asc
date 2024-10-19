use tracing;

use crate::util;

pub fn gen(options: &util::cli::Options) {
    let mut args = vec!["-S", &options.project_dir, "-B", &options.build_dir];

    if options.cmake_target_type == util::cli::types::CMakeTargetType::Library
        && options.cmake_lib_type == util::cli::types::CMakeLibraryType::Shared
    {
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
