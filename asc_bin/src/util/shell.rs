use std::collections::HashMap;

use crate::{cli::commands::VcpkgArgs, config::system_paths};

pub fn run(
    command: &str,
    args: &Vec<&str>,
    capture_stdout: bool,
    capture_stderr: bool,
    silent: bool,
) -> std::io::Result<std::process::Output> {
    let mut envs = HashMap::new();
    let vcpkg_conf = VcpkgArgs::load(&system_paths::ConfigPath::vcpkg_toml(), true).unwrap();
    if let Some(env) = vcpkg_conf.env_download_dir {
        envs.insert("VCPKG_DOWNLOADS", env);
    }
    if let Some(env) = vcpkg_conf.env_binary_cache_dir {
        envs.insert("VCPKG_DEFAULT_BINARY_CACHE", env);
    }

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
