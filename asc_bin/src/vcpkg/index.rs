use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use basic_trie::DataTrie;
use config_file_derives::ConfigFile;
use config_file_types;

use super::VcpkgManager;

use crate::{
    cli::commands::VcpkgArgs,
    config::{
        self,
        relative_paths::asc::ASC_REGISTRY_DIR_NAME,
        vcpkg::{port_manifest::VcpkgPortManifest, versions_baseline::VcpkgBaseline},
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

    pub versions: BTreeMap<String, BTreeSet<String>>,

    check_point: GitCommitInfo,
}

impl VcpkgManager {
    pub fn index(&mut self) -> bool {
        self.config_get(true);

        self.clean_deprecated_files();

        if !self.build_search_index() {
            return false;
        }

        return true;
    }

    pub fn clean_deprecated_files(&self) {
        for (name, _registry_root_dir) in Self::get_vcpkg_root_dir() {
            let tree_index_path = config::system_paths::DataPath::vcpkg_tree_index_json(
                self.args.index_directory.as_ref().unwrap(),
                &name,
            );
            util::fs::remove_file(&tree_index_path);
        }
    }

    pub fn get_vcpkg_root_dir() -> Vec<(String, String)> {
        let mut results = vec![];
        let vcpkg_conf = VcpkgArgs::load_or_default();
        for (name, _url, _branch, directory) in vcpkg_conf.flatten_registry() {
            results.push((name, directory));
        }
        return results;
    }

    fn build_search_index(&mut self) -> bool {
        let (registry_name, _url, _branch, registry_root_dir) =
            self.args.get_private_registry(ASC_REGISTRY_DIR_NAME);
        let latest_commit = Self::get_latest_commit(&registry_root_dir);

        let versions_baseline_json_path =
            config::system_paths::DataPath::vcpkg_versions_baseline_json_path(&registry_root_dir);
        match VcpkgBaseline::load(&versions_baseline_json_path, false) {
            None => return false,
            Some(baseline_data) => {
                let mut search_index = VcpkgSearchIndex::load(
                    &config::system_paths::DataPath::vcpkg_search_index_json(
                        self.args.index_directory.as_ref().unwrap(),
                        &registry_name,
                    ),
                    true,
                )
                .unwrap();
                if latest_commit.hash == search_index.check_point.hash {
                    return true;
                }

                let mut versions = BTreeMap::new();
                for (port_name, version_info) in &baseline_data.default {
                    let (n, version) =
                        VcpkgPortManifest::remove_verson_suffix(port_name, version_info);

                    search_index.prefix_trie.insert(&n, n.clone());
                    search_index
                        .postfix_trie
                        .insert(&util::str::reverse_string(&n), n.clone());

                    versions
                        .entry(n)
                        .or_insert_with(BTreeSet::new)
                        .insert(version);
                }

                search_index.versions = versions;
                search_index.check_point = latest_commit.clone();
                search_index.dump(false, false);
            }
        }

        return true;
    }

    pub fn get_latest_commit(vcpkg_root_dir: &str) -> GitCommitInfo {
        return git::log::get_latest_commit(
            vcpkg_root_dir,
            git::log::GIT_LOG_FORMAT_COMMIT_HASH_DATE,
        );
    }
}
