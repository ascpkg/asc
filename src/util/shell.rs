pub fn run(
    command: &str,
    args: &Vec<&str>,
    stdout: Option<std::process::Stdio>,
    stderr: Option<std::process::Stdio>,
) -> std::io::Result<std::process::Output> {
    tracing::info!("command: {}, args: {}", command, args.join(" "));

    return std::process::Command::new("git")
        .args(args)
        .stdout(stdout.unwrap_or(std::process::Stdio::inherit()))
        .stderr(stderr.unwrap_or(std::process::Stdio::inherit()))
        .output();
}
