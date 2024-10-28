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
    pub date_time: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgBaseline {
    #[serde(skip)]
    path: String,
    pub default: HashMap<String, VcpkgPortVersion>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgSearchIndex {
    #[serde(skip)]
    path: String,
    pub prefix: DataTrie<String>,
    pub postfix: DataTrie<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct VcpkgPortVersion {
    pub baseline: String,
    pub port_version: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgVersionIndex {
    #[serde(skip)]
    path: String,

    pub port_name_si: HashMap<String, u32>,
    #[serde(skip)]
    pub port_name_is: Vec<String>,

    pub commit_si: HashMap<String, u32>,
    #[serde(skip)]
    pub commit_is: Vec<String>,

    pub date_time_si: HashMap<String, u32>,
    #[serde(skip)]
    pub date_time_is: Vec<String>,

    pub port_version_si: HashMap<String, HashMap<String, u32>>,
    #[serde(skip)]
    pub port_version_is: Vec<String>,

    // port name index -> (port version index, commit hash index, commit date time index)
    pub port_versions: HashMap<u32, Vec<(u32, u32, u32)>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgPortVersions {
    #[serde(skip)]
    path: String,

    versions: Vec<VcpkgPortTreeVersion>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct VcpkgPortTreeVersion {
    git_tree: String,
    version: Option<String>,
    version_date: Option<String>,
    version_semver: Option<String>,
    version_string: Option<String>,
    port_version: u32,
}

impl VcpkgPortTreeVersion {
    fn build_version_text(&self) -> String {
        let mut s = String::new();
        if let Some(v) = &self.version {
            s = v.clone();
        } else if let Some(v) = &self.version_date {
            s = v.clone();
        } else if let Some(v) = &self.version_string {
            s = v.clone();
        } else if let Some(v) = &self.version_semver {
            s = v.clone();
        }

        if self.port_version == 0 {
            s
        } else {
            format!("{}#{}", s, self.port_version)
        }
    }
}

impl VcpkgManager {
    pub fn index(&mut self) -> bool {
        // compare check point
        let commits = self.get_commits();
        let latest_commit = &commits[commits.len() - 1];
        let indexed_commit = &self.load_check_point();
        if let Some(commit) = indexed_commit {
            if commit.hash == latest_commit.hash {
                return true;
            }
        }

        // build search index files
        if !self.build_search_index() {
            return false;
        }

        // build version history
        if !self.build_version_history(&commits, indexed_commit) {
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
            Some(baseline_data) => {
                std::fs::copy(
                    &baseline_json_path,
                    config::dir::DataDir::vcpkg_search_baseline_json(),
                )
                .unwrap();

                let mut search_index = VcpkgSearchIndex::default();
                search_index.path = config::dir::DataDir::vcpkg_search_index_json();
                for port_name in baseline_data.default.keys() {
                    search_index.prefix.insert(&port_name, port_name.clone());
                    search_index
                        .postfix
                        .insert(&util::str::reverse_string(port_name), port_name.clone());
                }
                return search_index.dump(false);
            }
        }
    }

    fn build_version_history(
        &self,
        commits: &Vec<CommitInfo>,
        indexed_commit: &Option<CommitInfo>,
    ) -> bool {
        let config_path = config::dir::DataDir::vcpkg_search_version_index_json();
        let mut version_index = VcpkgVersionIndex::load(&config_path, true).unwrap();

        // update commit hash and date index
        let mut updated = false;
        for commit_index in 0..commits.len() {
            let commit = &commits[commit_index];
            if indexed_commit.is_none()
                || commit.date_time > indexed_commit.as_ref().unwrap().date_time
            {
                updated = true;
                version_index
                    .commit_si
                    .insert(commit.hash.clone(), commit_index as u32);
                version_index
                    .date_time_si
                    .insert(commit.date_time.clone(), commit_index as u32);
            }
        }

        let baseline_json_path = format!(
            "{}/{}",
            self.args.directory.as_ref().unwrap(),
            super::path::BASELINE_JSON
        );
        match VcpkgBaseline::load(&baseline_json_path, false) {
            None => {
                return false;
            }
            Some(baseline) => {
                // update port name index
                let mut ports = vec![];
                for k in baseline.default.keys() {
                    if !version_index.port_name_si.contains_key(k) {
                        ports.push(k);
                    }
                }
                ports.sort();

                let port_offset = version_index.port_name_si.len();
                for port_index in 0..ports.len() {
                    updated = true;
                    version_index
                        .port_name_si
                        .insert(ports[port_index].clone(), (port_offset + port_index) as u32);
                }

                for name in ports {
                    let path = format!(
                        "{}/{}/{}-/{}.json",
                        self.args.directory.as_ref().unwrap(),
                        super::path::VERSION_DIR,
                        name.chars().nth(0).unwrap(),
                        name
                    );
                    match VcpkgPortVersions::load(&path, false) {
                        None => {}
                        Some(port_versions) => {
                            // update port version index
                            let mut versions = vec![];
                            let mut tree_hash = HashMap::new();
                            for v in &port_versions.versions {
                                let s = v.build_version_text();
                                if !version_index.port_version_si.contains_key(&s) {
                                    versions.push(s.clone());
                                    tree_hash.insert(s, v.git_tree.clone());
                                }
                            }
                            versions.reverse();

                            let ver_offset = version_index
                                .port_version_si
                                .get(name)
                                .unwrap_or(&HashMap::new())
                                .len();
                            for v_index in 0..versions.len() {
                                updated = true;
                                let ver_index = (ver_offset + v_index) as u32;
                                version_index
                                    .port_version_si
                                    .entry(name.clone())
                                    .or_insert_with(HashMap::new)
                                    .insert(versions[v_index].clone(), ver_index);

                                // update versions
                                let (commit_hash, commit_date_time) = self
                                    .get_commit_hash(&tree_hash.get(&versions[v_index]).unwrap());
                                tracing::info!(
                                    port = name,
                                    version = versions[v_index],
                                    commit_date_time = commit_date_time
                                );
                                let name_index = version_index.port_name_si.get(name).unwrap();
                                let commit_index =
                                    version_index.commit_si.get(&commit_hash).unwrap();
                                let date_time_index =
                                    version_index.date_time_si.get(&commit_date_time).unwrap();

                                version_index
                                    .port_versions
                                    .entry(name_index.clone())
                                    .or_insert_with(Vec::new)
                                    .push((
                                        ver_index,
                                        commit_index.clone(),
                                        date_time_index.clone(),
                                    ));
                            }
                        }
                    }
                }
            }
        }

        if updated {
            return version_index.dump(false);
        }

        return true;
    }

    // fn build_dependencies(&self) -> bool {
    //     true
    // }

    fn save_check_point(&self, commit_info: &CommitInfo) -> bool {
        let mut c = commit_info.clone();
        c.path = config::dir::DataDir::vcpkg_check_point_file();
        c.dump(false)
    }

    fn load_check_point(&self) -> Option<CommitInfo> {
        CommitInfo::load(&config::dir::DataDir::vcpkg_check_point_file(), true)
    }

    fn get_commit_hash(&self, git_tree_hash: &str) -> (String, String) {
        let cwd = util::fs::get_cwd();
        util::fs::set_cwd(self.args.directory.as_ref().unwrap());
        let output = util::shell::run(
            "git",
            &vec![
                "log",
                "--all",
                "--date=iso",
                "--pretty=%H  %ad",
                &format!("--find-object={git_tree_hash}"),
            ],
            true,
            false,
        )
        .unwrap();
        util::fs::set_cwd(&cwd);

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        for line in stdout.split("\n") {
            let s = line.trim();
            if !s.is_empty() {
                let parts = s.split("  ").collect::<Vec<&str>>();
                if parts.len() == 2 {
                    return (parts[0].to_string(), parts[1].to_string());
                }
            }
        }
        return (String::new(), String::new());
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
                        r#"--pretty=format:{"hash": "%H", "date_time": "%ad"}"#,
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
