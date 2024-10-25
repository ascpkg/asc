pub fn run(
    command: &str,
    args: &Vec<&str>,
    capture_stdout: bool,
    capture_stderr: bool,
) -> std::io::Result<std::process::Output> {
    tracing::info!("command: {}, args: {}", command, args.join(" "));

    return std::process::Command::new(command)
        .args(args)
        .stdout(if capture_stdout {
            std::process::Stdio::piped()
        } else {
            std::process::Stdio::inherit()
        })
        .stderr(if capture_stderr {
            std::process::Stdio::piped()
        } else {
            std::process::Stdio::inherit()
        })
        .output();
}
