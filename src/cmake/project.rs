pub fn gen(build_dir: &String, source_dir: &str) {
    std::process::Command::new("cmake")
        .arg("-S")
        .arg(source_dir)
        .arg("-B")
        .arg(build_dir)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .unwrap();
}
