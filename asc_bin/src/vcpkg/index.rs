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

    pub prefix_trie: DataTrie<String>,
    pub postfix_trie: DataTrie<String>,

    pub baseline: VcpkgBaseline,

    check_point: CommitInfo,
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

    pub port_name_index: HashMap<String, u32>,
    #[serde(skip)]
    pub port_names: Vec<String>,

    pub commit_index: HashMap<String, u32>,
    #[serde(skip)]
    pub commits: Vec<String>,

    pub date_time_index: HashMap<String, Vec<u32>>,
    #[serde(skip)]
    pub date_times: Vec<String>,

    pub port_version_index: HashMap<String, HashMap<String, Vec<u32>>>,
    #[serde(skip)]
    pub port_versions: HashMap<String, Vec<String>>,

    // port name index -> (port version index, commit hash index, commit date time index)
    pub versions: HashMap<u32, Vec<(u32, u32, u32)>>,

    check_point: CommitInfo,
}

impl VcpkgVersionIndex {
    pub fn read() -> Self {
        if let Some(mut index) = Self::load(
            &config::dir::DataDir::vcpkg_search_version_index_json(),
            false,
        ) {
            index.build_reverse_index();
            return index;
        }
        return VcpkgVersionIndex::default();
    }

    pub fn build_reverse_index(&mut self) {
        if self.port_names.is_empty() {
            self.port_names = vec![String::new(); self.port_name_index.len()];
            for (s, i) in &self.port_name_index {
                self.port_names[i.clone() as usize] = s.clone();
            }
        }

        if self.commits.is_empty() {
            self.commits = vec![String::new(); self.commit_index.len()];
            for (s, i) in &self.commit_index {
                self.commits[i.clone() as usize] = s.clone();
            }
        }

        if self.date_times.is_empty() {
            let mut size = 0;
            for (_, indexes) in &self.date_time_index {
                size += indexes.len();
            }

            self.date_times = vec![String::new(); size];
            for (s, i) in &self.date_time_index {
                for x in i {
                    self.date_times[x.clone() as usize] = s.clone();
                }
            }
        }

        for (name, v) in &self.port_version_index {
            let mut size = 0;
            for (_, indexes) in v {
                size += indexes.len();
            }

            self.port_versions
                .insert(name.clone(), vec![String::new(); size]);
            for (s, i) in v {
                for x in i {
                    self.port_versions.get_mut(name).unwrap()[x.clone() as usize] = s.clone();
                }
            }
        }
    }

    pub fn get_versions(&self, port: &str) -> Vec<(String, String, String)> {
        let mut results = vec![];
        if let Some(name_index) = self.port_name_index.get(port) {
            if let Some(versions) = self.versions.get(name_index) {
                results = vec![(String::new(), String::new(), String::new()); versions.len()];
                for (version_index, commit_index, date_time_index) in versions {
                    let index = version_index.clone() as usize;
                    results[index] = (
                        self.port_versions.get(port).unwrap()[index].clone(),
                        self.commits[commit_index.clone() as usize].clone(),
                        self.date_times[date_time_index.clone() as usize].clone(),
                    )
                }
            }
        }
        return results;
    }
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgGitTreePort {
    #[serde(skip)]
    path: String,

    index: HashMap<String, VcpkgGitTreeInfo>,

    check_point: CommitInfo,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VcpkgGitTreeInfo {
    pub port_name: String,
    pub commit_hash: String,
    pub commit_date_time: String,
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
        let commits = self.get_commits();
        let latest_commit = &commits[commits.len() - 1];

        // map git-tree to commit hash and date time
        let git_tree_to_port = self.index_git_tree(&commits);

        // build search index files
        if !self.build_search_index(latest_commit) {
            return false;
        }

        // build version history
        if !self.build_version_history(&commits, &git_tree_to_port) {
            return false;
        }

        return true;
    }

    fn build_search_index(&mut self, latest_commit: &CommitInfo) -> bool {
        self.config_get(true);
        let baseline_json_path = format!(
            "{}/{}",
            self.args.directory.as_ref().unwrap(),
            super::path::BASELINE_JSON
        );
        match VcpkgBaseline::load(&baseline_json_path, false) {
            None => return false,
            Some(baseline_data) => {
                let mut search_index =
                    VcpkgSearchIndex::load(&config::dir::DataDir::vcpkg_search_index_json(), true)
                        .unwrap();
                if latest_commit.date_time <= search_index.check_point.date_time {
                    return true;
                }

                for port_name in baseline_data.default.keys() {
                    search_index
                        .prefix_trie
                        .insert(&port_name, port_name.clone());
                    search_index
                        .postfix_trie
                        .insert(&util::str::reverse_string(port_name), port_name.clone());
                }
                search_index.baseline = baseline_data;
                search_index.check_point = latest_commit.clone();
                return search_index.dump(false);
            }
        }
    }

    fn build_version_history(
        &self,
        commits: &Vec<CommitInfo>,
        git_tree_to_port: &VcpkgGitTreePort,
    ) -> bool {
        let config_path = config::dir::DataDir::vcpkg_search_version_index_json();
        let mut version_index = VcpkgVersionIndex::load(&config_path, true).unwrap();

        // update commit hash and date index
        for commit_index in 0..commits.len() {
            let commit = &commits[commit_index];
            if commit.date_time <= version_index.check_point.date_time {
                continue;
            }
            version_index
                .commit_index
                .insert(commit.hash.clone(), commit_index as u32);
            version_index
                .date_time_index
                .entry(commit.date_time.clone().clone())
                .or_insert_with(Vec::new)
                .push(commit_index as u32);
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
                    if !version_index.port_name_index.contains_key(k) {
                        ports.push(k);
                    }
                }
                ports.sort();

                let port_offset = version_index.port_name_index.len();
                for port_index in 0..ports.len() {
                    version_index
                        .port_name_index
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
                            let mut text_versions = vec![];
                            let mut text_verstion_to_git_tree = HashMap::new();
                            for v in &port_versions.versions {
                                let s = v.build_version_text();
                                if !version_index.port_version_index.contains_key(&s) {
                                    text_versions.push(s.clone());
                                    text_verstion_to_git_tree.insert(s, v.git_tree.clone());
                                }
                            }
                            text_versions.reverse();

                            let version_offset = version_index
                                .port_version_index
                                .get(name)
                                .unwrap_or(&HashMap::new())
                                .len();
                            for i in 0..text_versions.len() {
                                let ver_index = (version_offset + i) as u32;
                                version_index
                                    .port_version_index
                                    .entry(name.clone())
                                    .or_insert_with(HashMap::new)
                                    .entry(text_versions[i].clone())
                                    .or_insert_with(Vec::new)
                                    .push(ver_index);

                                match git_tree_to_port
                                    .index
                                    .get(text_verstion_to_git_tree.get(&text_versions[i]).unwrap())
                                {
                                    None => {
                                        tracing::error!(
                                            port = name,
                                            version = text_versions[i],
                                            git_tree = text_verstion_to_git_tree
                                                .get(&text_versions[i])
                                                .unwrap()
                                        );
                                    }
                                    Some(info) => {
                                        // tracing::info!(
                                        //     port = name,
                                        //     version = text_versions[i],
                                        //     commit_date_time = info.commit_date_time
                                        // );
                                        let name_index =
                                            version_index.port_name_index.get(name).unwrap();
                                        let commit_index = version_index
                                            .commit_index
                                            .get(&info.commit_hash)
                                            .unwrap();
                                        let date_time_index = version_index
                                            .date_time_index
                                            .get(&info.commit_date_time)
                                            .unwrap()[0];

                                        version_index
                                            .versions
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
            }
        }

        version_index.check_point = commits[commits.len() - 1].clone();
        version_index.dump(false);

        return true;
    }

    fn index_git_tree(&self, commits: &Vec<CommitInfo>) -> VcpkgGitTreePort {
        let mut results =
            VcpkgGitTreePort::load(&config::dir::DataDir::vcpkg_git_tree_json(), true).unwrap();

        let mut index = 0;
        for c in commits {
            index += 1;

            if c.date_time <= results.check_point.date_time {
                continue;
            }

            let trees = self.get_git_trees(&c.hash);
            for (git_tree, port_name) in &trees {
                if !results.index.contains_key(git_tree) {
                    results.index.insert(
                        git_tree.clone(),
                        VcpkgGitTreeInfo {
                            port_name: port_name.clone(),
                            commit_hash: c.hash.clone(),
                            commit_date_time: c.date_time.clone(),
                        },
                    );
                }
            }

            if index % 200 == 0 || commits.len() < 1000 {
                results.check_point = c.clone();
                results.dump(false);
                tracing::info!("[{index}] #{}# {:#?}", results.index.len(), c.date_time);
            }
        }

        results.check_point = commits[commits.len()-1].clone();
        results.dump(false);

        return results;
    }

    fn get_git_trees(&self, git_commit_hash: &str) -> Vec<(String, String)> {
        let mut results = vec![];

        let cwd = util::fs::get_cwd();
        util::fs::set_cwd(self.args.directory.as_ref().unwrap());
        let output = util::shell::run(
            "git",
            &vec![
                "ls-tree",
                "-d",
                "-r",
                "--full-tree",
                git_commit_hash,
                "ports",
            ],
            true,
            false,
            true,
        )
        .unwrap();
        util::fs::set_cwd(&cwd);

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        for line in stdout.split("\n") {
            let s = line.trim();
            if !s.is_empty() {
                let right = s.split_once(" tree ").unwrap().1;
                let parts: Vec<&str> = right.split("ports/").map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    results.push((parts[0].to_string(), parts[1].to_string()));
                }
            }
        }

        return results;
    }

    fn get_commits(&mut self) -> Vec<CommitInfo> {
        let mut commits = vec![];

        self.config_get(true);
        match &self.args.directory {
            None => {
                tracing::error!(
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
