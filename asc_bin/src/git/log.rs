use serde::{Deserialize, Serialize};

use crate::{errors::ErrorTag, util};

pub static GIT_LOG_FORMAT_COMMIT_HASH_DATE: &str =
    r#"--pretty=format:{"hash": "%H", "date_time": "%ad"}"#;
pub static GIT_LOG_FORMAT_ABBR_COMMIT_HASH_DATE: &str =
    r#"--pretty=format:{"hash": "%h", "date_time": "%ad"}"#;
pub static GIT_LOG_FORMAT_VERSION_STAT: &str =
    r#"--pretty=format:commit %H%nDate:   %ad%n%n    %s%n"#;

// from vcpkg (git log)
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GitCommitInfo {
    #[serde(skip)]
    pub path: String,

    pub hash: String,
    pub date_time: String,
}

pub fn get_latest_commit_stat(repo_root_dir: &str) -> String {
    let output = util::shell::run(
        "git",
        &vec![
            "log",
            "-n 1",
            "--date=iso",
            "--stat",
            GIT_LOG_FORMAT_VERSION_STAT,
        ],
        repo_root_dir,
        true,
        false,
        false,
    )
    .unwrap();

    return String::from_utf8_lossy(&output.stdout).trim().to_string();
}

pub fn get_latest_commit(repo_root_dir: &str, pretty_format: &str) -> GitCommitInfo {
    let output = util::shell::run(
        "git",
        &vec!["log", "-n 1", "--date=iso", pretty_format],
        repo_root_dir,
        true,
        false,
        false,
    )
    .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    for line in stdout.split("\n") {
        match serde_json::from_str(line) {
            Err(e) => {
                tracing::error!(
                    call = "serde_json::from_str",
                    line = line,
                    error_tag = ErrorTag::JsonDeserializeError.as_ref(),
                    message = e.to_string()
                );
            }
            Ok(info) => {
                return info;
            }
        }
    }

    return GitCommitInfo::default();
}

pub fn get_commits(repo_root_dir: &str, pretty_format: &str) -> Vec<GitCommitInfo> {
    let mut commits = vec![];

    let output = util::shell::run(
        "git",
        &vec!["log", "--reverse", "--date=iso", pretty_format],
        repo_root_dir,
        true,
        false,
        false,
    )
    .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    for line in stdout.split("\n") {
        match serde_json::from_str(line) {
            Err(e) => {
                tracing::error!(
                    call = "serde_json::from_str",
                    line = line,
                    error_tag = ErrorTag::JsonDeserializeError.as_ref(),
                    message = e.to_string()
                );
            }
            Ok(info) => {
                commits.push(info);
            }
        }
    }

    return commits;
}
