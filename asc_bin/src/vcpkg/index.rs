use std::collections::HashMap;

use basic_trie::DataTrie;
use config_file_derives::ConfigFile;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{config, errors::ErrorTag, util};

use super::VcpkgManager;

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("toml")]
pub struct CommitInfo {
    #[serde(skip)]
    path: String,
    pub hash: String,
    pub datetime: String,
    pub user_email: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgBaseline {
    #[serde(skip)]
    path: String,
    pub default: HashMap<String, VcpkgPortVersion>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct VcpkgPortVersion {
    pub baseline: String,
    pub port_version: u32,
}

impl VcpkgManager {
    pub fn index(&mut self) -> bool {
        // compare check point
        let commits = self.get_commits();
        let latest_commit = &commits[commits.len() - 1];
        if let Some(commit) = &self.load_check_point() {
            if commit.hash == latest_commit.hash {
                return true;
            }
        }

        // build search index files
        if self.build_search_index() {
            return false;
        }

        // build version history
        if self.build_version_history() {
            return false;
        }

        // save check point
        return self.save_check_point(&commits[commits.len() - 1]);
    }

    fn build_search_index(&mut self) -> bool {
        self.config_get(true);
        let baseline_json_path = format!(
            "{}/{}",
            self.args.directory.as_ref().unwrap(),
            super::path::BASELINE_JSON
        );
        match VcpkgBaseline::load(&baseline_json_path, false) {
            None => return false,
            Some(baselie_data) => {
                std::fs::copy(
                    &baseline_json_path,
                    config::dir::DataDir::vcpkg_search_baseline_file(),
                )
                .unwrap();

                let data_path = [
                    config::dir::DataDir::vcpkg_search_prefix_index_file(),
                    config::dir::DataDir::vcpkg_search_postfix_index_file(),
                ];
                let mut data_trie = [DataTrie::<String>::new(), DataTrie::<String>::new()];
                for port_name in baselie_data.default.keys() {
                    data_trie[0].insert(&port_name, port_name.clone());
                    data_trie[1].insert(&util::str::reverse_string(port_name), port_name.clone());
                }

                let mut has_error = false;
                for i in 0..data_path.len() {
                    match serde_json::to_string(&data_trie[i]) {
                        Err(e) => {
                            tracing::error!(
                                call = "serde_json::to_string",
                                error_tag = ErrorTag::JsonSerializeError.as_ref(),
                                error_str = e.to_string()
                            );
                            has_error = true;
                        }
                        Ok(text) => {
                            has_error &= std::fs::write(&data_path[i], text.as_bytes()).is_ok();
                        }
                    }
                }

                return has_error;
            }
        }
    }

    pub fn load_search_index(&self) -> Vec<DataTrie<String>> {
        let data_path = [
            config::dir::DataDir::vcpkg_search_prefix_index_file(),
            config::dir::DataDir::vcpkg_search_postfix_index_file(),
        ];
        let mut data_tire = vec![];
        for i in 0..data_path.len() {
            match std::fs::read_to_string(&data_path[i]) {
                Err(e) => {
                    tracing::error!(
                        call = "std::fs::read_to_string",
                        path = data_path[i],
                        error_tag = ErrorTag::ReadFileError.as_ref(),
                        error_str = e.to_string()
                    );
                }
                Ok(text) => match serde_json::from_str(&text) {
                    Err(e) => {
                        tracing::error!(
                            call = "serde_json::from_str",
                            error_tag = ErrorTag::JsonDeserializeError.as_ref(),
                            error_str = e.to_string()
                        );
                    }
                    Ok(tire) => {
                        data_tire.push(tire);
                    }
                },
            }
        }

        return data_tire;
    }

    fn build_version_history(&self) -> bool {
        true
    }

    fn build_dependencies(&self) -> bool {
        true
    }

    fn parse_baseline(&self, commit_info: &CommitInfo, index: i32) {
        let cwd = util::fs::get_cwd();
        let dir = self.args.directory.as_ref().unwrap();
        util::fs::set_cwd(&dir);

        util::shell::run(
            "git",
            &vec!["reset", "--hard", &commit_info.hash],
            false,
            false,
        )
        .unwrap();

        if util::fs::is_file_exists("versions/baseline.json") {
            self.parse_baseline_from_versions(&commit_info, index);
        } else if util::fs::is_file_exists("port_versions/baseline.json") {
            self.parse_baseline_from_port_versions(&commit_info, index);
        } else {
            self.parse_baseline_from_control(&commit_info, index);
        }

        util::fs::set_cwd(&cwd);
    }

    // ports/ffmpeg/CONTROL
    fn parse_baseline_from_control(&self, commit_info: &CommitInfo, index: i32) {
        let mut data = VcpkgBaseline::default();

        let ports = "ports";
        let file_name = "FILENAME";
        let version_prefix = "Version:";
        for dir_entry in std::fs::read_dir(ports).unwrap() {
            let entry = dir_entry.unwrap();
            let entry_path = entry.path();
            if entry_path.is_dir() {
                let entry_str = entry_path.to_str().unwrap();
                if entry_str == ports {
                    continue;
                }
                let control_file_path = format!("{}/CONTROL", entry_str);
                if !util::fs::is_file_exists(&control_file_path) {
                    // tracing::error!(
                    //     call = "!util::fs::is_file_exists",
                    //     path = control_file_path,
                    //     error_tag = ErrorTag::FileNotFoundError.as_ref()
                    // );

                    let port_file = format!("{}/portfile.cmake", entry_str);
                    let text = std::fs::read_to_string(port_file).unwrap();
                    for line in text.split("\n") {
                        let mut s = line.trim_start();
                        if s.starts_with(file_name) {
                            let port = entry_path.file_name().unwrap().to_str().unwrap();
                            // s = s
                            //     .split_at(file_name.len())
                            //     .1
                            //     .trim()
                            //     .strip_prefix(r#"""#)
                            //     .unwrap()
                            //     .strip_suffix(r#"""#)
                            //     .unwrap();
                            // s = s.split_at(port.len()).1;
                            // let ext = std::path::Path::new(s).extension().unwrap();
                            // s = s.split_at(s.len() - ext.len() - 1).0;
                            // if let Some(ext) = std::path::Path::new(s).extension() {
                            //     s = s.split_at(s.len() - ext.len() - 1).0;
                            // }
                            tracing::error!(port = port, version = s);
                            break;
                        }
                    }
                } else {
                    let text = std::fs::read_to_string(&control_file_path).unwrap();
                    for line in text.split("\n") {
                        if line.starts_with(&version_prefix) {
                            let version = line.split_at(version_prefix.len()).1.trim();
                            let port = entry_path.file_name().unwrap().to_str().unwrap();
                            // tracing::info!(port = port, version = version);
                            data.default.insert(
                                port.to_string(),
                                VcpkgPortVersion {
                                    baseline: version.to_string(),
                                    port_version: 0,
                                },
                            );
                        }
                    }
                }
            }
        }

        let s = serde_json::to_string_pretty(&data).unwrap();
        std::fs::write(
            format!(
                "../baselines/{}-{}.json",
                commit_info.datetime.split_at(10).0,
                index
            ),
            s.as_bytes(),
        )
        .unwrap()
    }

    // port_versions/baseline.json
    fn parse_baseline_from_port_versions(&self, commit_info: &CommitInfo, index: i32) {
        // let s = std::fs::read_to_string("port_versions/baseline.json").unwrap();
        // let x: VcpkgBaseline = serde_json::from_str(&s).unwrap();
        // tracing::info!("{:#?}", x);
        std::fs::copy(
            "port_versions/baseline.json",
            format!(
                "../baselines/{}-{}.json",
                commit_info.datetime.split_at(10).0,
                index
            ),
        );
    }

    // versions/baseline.json
    fn parse_baseline_from_versions(&self, commit_info: &CommitInfo, index: i32) {
        // let s = std::fs::read_to_string("port_versions/baseline.json").unwrap();
        // let x: VcpkgBaseline = serde_json::from_str(&s).unwrap();
        // tracing::info!("{:#?}", x);
        std::fs::copy(
            "versions/baseline.json",
            format!(
                "../baselines/{}-{}.json",
                commit_info.datetime.split_at(10).0,
                index
            ),
        );
    }

    fn save_check_point(&self, commit_info: &CommitInfo) -> bool {
        let mut c = commit_info.clone();
        c.path = config::dir::DataDir::vcpkg_check_point_file();
        c.dump(false)
    }

    fn load_check_point(&self) -> Option<CommitInfo> {
        CommitInfo::load(&config::dir::DataDir::vcpkg_check_point_file(), false)
    }

    fn get_commits(&mut self) -> Vec<CommitInfo> {
        let mut commits = vec![];

        self.config_get(true);
        match &self.args.directory {
            None => {
                tracing::info!(
                    call = "self.args.directory.is_none",
                    error_tag = ErrorTag::InvalidConfigError.as_ref()
                );
                return commits;
            }
            Some(dir) => {
                let cwd = util::fs::get_cwd();

                util::fs::set_cwd(dir);
                let output = util::shell::run(
                    "git",
                    &vec![
                        "log",
                        "--reverse",
                        "--date=iso",
                        r#"--pretty=format:{"hash": "%H", "datetime": "%ad", "user_email": "%ae"}"#,
                    ],
                    true,
                    false,
                )
                .unwrap();
                util::fs::set_cwd(&cwd);

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
            }
        }

        return commits;
    }
}
