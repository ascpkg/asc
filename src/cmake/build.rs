use crate::util;

pub fn compile(build_dir: &String, config: &util::cli::CMakeConfigType) {
    std::process::Command::new("cmake")
        .arg("--build")
        .arg(build_dir)
        .arg("--config")
        .arg(if config == &util::cli::CMakeConfigType::Debug {
            "Debug"
        } else {
            "Release"
        })
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .unwrap();
}
