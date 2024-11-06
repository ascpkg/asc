use crate::cli::commands::VcpkgArgs;

pub fn run(
    command: &str,
    args: &Vec<&str>,
    capture_stdout: bool,
    capture_stderr: bool,
    silent: bool,
) -> std::io::Result<std::process::Output> {
    let vcpkg_conf = VcpkgArgs::load_or_default();
    let envs = vcpkg_conf.get_envs();

    if !silent {
        tracing::info!(
            "command: {}, args: {}, envs: {}",
            command,
            args.join(" "),
            serde_json::to_string(&envs).unwrap()
        );
    }

    return std::process::Command::new(command)
        .args(args)
        .envs(envs)
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
