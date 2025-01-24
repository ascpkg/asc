use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    config::relative_paths::{VCPKG_CONTROL_FILE_NAME, VCPKG_JSON_FILE_NAME, VCPKG_PORTS_DIR_NAME},
    errors::ErrorTag,
    util,
};

pub static GIT_LOG_FORMAT_COMMIT_HASH_DATE: &str =
    r#"--pretty=format:{"hash": "%H", "date_time": "%ad", "user_email": "%ae"}"#;
pub static GIT_LOG_FORMAT_ABBR_COMMIT_HASH_DATE: &str =
    r#"--pretty=format:{"hash": "%h", "date_time": "%ad", "user_email": "%ae"}"#;
pub static GIT_LOG_FORMAT_VERSION_STAT: &str =
    r#"--pretty=format:commit %H%nDate:   %ad%n%n    %s%n"#;

// from vcpkg (git log)
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GitCommitInfo {
    #[serde(skip)]
    pub path: String,

    pub hash: String,
    pub date_time: String,
    pub user_email: String,
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

pub fn get_changed_commits(
    repo_root_dir: &str,
    sub_path: &str,
) -> Vec<(GitCommitInfo, HashSet<String>)> {
    let mut commits = vec![];

    let output = util::shell::run(
        "git",
        &vec![
            "log",
            "--reverse",
            "--date=iso",
            "--name-only",
            GIT_LOG_FORMAT_COMMIT_HASH_DATE,
            sub_path,
        ],
        repo_root_dir,
        true,
        false,
        false,
    )
    .unwrap();

    let control_file_suffix = format!("/{VCPKG_CONTROL_FILE_NAME}");
    let vcpkg_json_file_suffix = format!("/{VCPKG_JSON_FILE_NAME}");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    for lines in stdout.split("\n\n") {
        let mut commit = GitCommitInfo::default();
        let mut changed_files = HashSet::new();
        let mut port_name = String::new();
        let mut manifest_file_found = false;
        for line in lines.lines() {
            if line.starts_with("{") {
                // commit info
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
                        commit = info;
                    }
                }
            } else {
                // changed files
                if line.starts_with(sub_path) {
                    changed_files.insert(line.trim().to_string());

                    let name = line
                        .split_at(VCPKG_PORTS_DIR_NAME.len())
                        .1
                        .split_once("/")
                        .unwrap()
                        .0
                        .to_string();

                    if !port_name.is_empty() && name != port_name {
                        // another port
                        append_manifest_file_if_not_changed(
                            repo_root_dir,
                            &port_name,
                            manifest_file_found,
                            &commit.hash,
                            &mut changed_files,
                        );
                        manifest_file_found = false;
                    }

                    port_name = name;

                    if line.ends_with(&control_file_suffix)
                        || line.ends_with(&vcpkg_json_file_suffix)
                    {
                        manifest_file_found = true;
                    }
                }
            }
        }

        append_manifest_file_if_not_changed(
            repo_root_dir,
            &port_name,
            manifest_file_found,
            &commit.hash,
            &mut changed_files,
        );

        commits.push((commit, changed_files));
    }

    return commits;
}

fn append_manifest_file_if_not_changed(
    repo_root_dir: &str,
    port_name: &str,
    manifest_file_found: bool,
    git_commit_hash: &str,
    changed_files: &mut HashSet<String>,
) {
    if port_name.is_empty() {
        tracing::warn!(
            message = "========== no files changed",
            port_name = port_name,
            git_commit_hash = git_commit_hash,
        );
    } else if !manifest_file_found {
        let control_path = format!("{VCPKG_PORTS_DIR_NAME}{port_name}/{VCPKG_CONTROL_FILE_NAME}");
        let mut tree_hash =
            super::rev_parse::get_tree_hash(repo_root_dir, &git_commit_hash, &control_path, true);
        if !tree_hash.is_empty() {
            tracing::warn!(
                message = "========== append CONTROL",
                port_name = port_name,
                git_commit_hash = git_commit_hash,
            );
            changed_files.insert(control_path);
        } else {
            let vcpkg_json_path =
                format!("{VCPKG_PORTS_DIR_NAME}{port_name}/{VCPKG_JSON_FILE_NAME}");
            tree_hash = super::rev_parse::get_tree_hash(
                repo_root_dir,
                &git_commit_hash,
                &vcpkg_json_path,
                true,
            );
            if !tree_hash.is_empty() {
                tracing::warn!(
                    message = "========== append vcpkg.json",
                    port_name = port_name,
                    git_commit_hash = git_commit_hash,
                );
                changed_files.insert(vcpkg_json_path);
            }
        }
        if tree_hash.is_empty() {
            tracing::warn!(
                message = "========== CONTROL or vcpkg.json not found",
                port_name = port_name,
                git_commit_hash = git_commit_hash,
            );
        }
    }
}
