use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use basic_trie::DataTrie;
use config_file_derives::ConfigFile;
use config_file_types;

use super::VcpkgManager;

use crate::{
    cli::commands::VcpkgArgs,
    config::{
        self,
        vcpkg::{
            versions_baseline::VcpkgBaseline,
            versions_port::{VcpkgGitTreeInfo, VcpkgPortVersions},
        },
    },
    git::{self, log::GitCommitInfo},
    util,
};

// asc
#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgSearchIndex {
    #[serde(skip)]
    path: String,

    pub prefix_trie: DataTrie<String>,
    pub postfix_trie: DataTrie<String>,

    pub baseline: VcpkgBaseline,

    check_point: GitCommitInfo,
}

// asc
#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct VcpkgGitTreeIndex {
    #[serde(skip)]
    path: String,

    index: HashMap<String, VcpkgGitTreeInfo>,

    check_point: GitCommitInfo,
}

impl VcpkgManager {
    pub fn index(&mut self) -> bool {
        let commits = self.get_commits();
        let latest_commit = &commits[commits.len() - 1];

        self.config_get(true);

        self.build_git_tree_index(&commits);

        if !self.build_search_index(latest_commit) {
            return false;
        }

        return true;
    }

    fn get_vcpkg_root_dir() -> String {
        let vcpkg_conf = VcpkgArgs::load_or_default();
        vcpkg_conf.directory.as_ref().unwrap().clone()
    }

    pub fn get_port_versions(&self, port: &str) -> Vec<(String, String, String)> {
        let mut results = vec![];

        let versions_port_json_path = config::system_paths::DataPath::vcpkg_versions_port_json_path(
            &Self::get_vcpkg_root_dir(),
            port,
        );
        if let Some(versions) = VcpkgPortVersions::load(&versions_port_json_path, false) {
            if let Some(git_tree_index) = VcpkgGitTreeIndex::load(
                &config::system_paths::DataPath::vcpkg_tree_index_json(
                    self.args.index_directory.as_ref().unwrap(),
                ),
                false,
            ) {
                for v in versions.versions {
                    if let Some(info) = git_tree_index.index.get(&v.git_tree) {
                        results.push((
                            v.format_version_text(),
                            info.commit_hash.clone(),
                            info.commit_date_time.clone(),
                        ));
                    } else {
                        tracing::error!("{:#?}", v)
                    }
                }
            }
        }

        return results;
    }

    fn build_search_index(&mut self, latest_commit: &GitCommitInfo) -> bool {
        let versions_baseline_json_path =
            config::system_paths::DataPath::vcpkg_versions_baseline_json_path(
                &Self::get_vcpkg_root_dir(),
            );
        match VcpkgBaseline::load(&versions_baseline_json_path, false) {
            None => return false,
            Some(baseline_data) => {
                let mut search_index = VcpkgSearchIndex::load(
                    &config::system_paths::DataPath::vcpkg_search_index_json(
                        self.args.index_directory.as_ref().unwrap(),
                    ),
                    true,
                )
                .unwrap();
                if latest_commit.hash <= search_index.check_point.hash {
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
                return search_index.dump(false, false);
            }
        }
    }

    fn build_git_tree_index(&self, commits: &Vec<GitCommitInfo>) -> VcpkgGitTreeIndex {
        let mut results = VcpkgGitTreeIndex::load(
            &config::system_paths::DataPath::vcpkg_tree_index_json(
                self.args.index_directory.as_ref().unwrap(),
            ),
            true,
        )
        .unwrap();

        let mut next_index = 0;
        if let Some(index) = commits
            .iter()
            .position(|c| c.hash == results.check_point.hash)
        {
            next_index = index + 1;
        }

        for (index, c) in commits[next_index..].iter().enumerate() {
            if index <= next_index {
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
                results.dump(false, false);
                tracing::info!("[{index}] #{}# {:#?}", results.index.len(), c.date_time);
            }
        }

        results.check_point = commits[commits.len() - 1].clone();
        results.dump(false, false);

        return results;
    }

    fn get_git_trees(&self, git_commit_hash: &str) -> Vec<(String, String)> {
        return git::ls_tree::run(git_commit_hash, &Self::get_vcpkg_root_dir());
    }

    pub fn get_latest_commit() -> GitCommitInfo {
        return git::log::get_latest_commit(
            &Self::get_vcpkg_root_dir(),
            git::log::GIT_LOG_FORMAT_COMMIT_HASH_DATE,
        );
    }

    fn get_commits(&mut self) -> Vec<GitCommitInfo> {
        return git::log::get_commits(
            &Self::get_vcpkg_root_dir(),
            git::log::GIT_LOG_FORMAT_COMMIT_HASH_DATE,
        );
    }
}
