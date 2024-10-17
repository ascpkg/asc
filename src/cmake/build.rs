pub fn compile(build_dir: &String) {
    std::process::Command::new("cmake")
        .arg("--build")
        .arg(build_dir)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .unwrap();
}
