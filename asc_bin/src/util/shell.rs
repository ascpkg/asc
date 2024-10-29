pub fn run(
    command: &str,
    args: &Vec<&str>,
    capture_stdout: bool,
    capture_stderr: bool,
    silent: bool,
) -> std::io::Result<std::process::Output> {
    if !silent {
        tracing::info!("command: {}, args: {}", command, args.join(" "));
    }

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
