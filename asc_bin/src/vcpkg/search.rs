use super::{index::VcpkgSearchIndex, VcpkgManager};

use crate::{
    cli::commands::VcpkgArgs,
    config::{self, vcpkg::versions_baseline::VcpkgPortVersion},
    git::log::GitCommitInfo,
    util,
};

pub fn get_port_version_commit_info(
    port_name: &str,
    version: &str,
) -> Option<(String, GitCommitInfo)> {
    let vcpkg_manager = VcpkgManager::new(VcpkgArgs::load_or_default());

    for (registry, _url, _branch, vcpkg_root_dir) in vcpkg_manager.args.flatten_registry() {
        for (v, c, d) in vcpkg_manager.get_port_versions(&vcpkg_root_dir, &registry, port_name) {
            if v == version {
                return Some((
                    registry,
                    GitCommitInfo {
                        hash: c,
                        date_time: d,
                        user_email: String::new(),
                        path: String::new(),
                    },
                ));
            }
        }
    }
    None
}

pub fn from_index_file(port_name: &str, list_all: bool) -> Vec<String> {
    let mut results = vec![];

    let vcpkg_manager = VcpkgManager::new(VcpkgArgs::load_or_default());

    for (registry, _url, _branch, vcpkg_root_dir) in vcpkg_manager.args.flatten_registry() {
        match VcpkgSearchIndex::load(
            &config::system_paths::DataPath::vcpkg_search_index_json(
                vcpkg_manager.args.index_directory.as_ref().unwrap(),
                &registry,
            ),
            false,
        ) {
            None => return results,
            Some(index) => {
                if port_name.starts_with("*") && port_name.ends_with("*") {
                    // contains
                    let mut query = port_name.split_at(1).1;
                    query = query.split_at(query.len() - 1).0;
                    for (name, version) in &index.baseline.default {
                        if name.contains(query) {
                            results.push(format_port_version(&registry, name, version));
                        }
                    }
                } else if port_name.ends_with("*") {
                    // prefix
                    let query = port_name.split_at(port_name.len() - 1).0;
                    if let Some(mut data) = index.prefix_trie.get_data(&query, true) {
                        data.sort();
                        for name in data {
                            if let Some(version) = index.baseline.default.get(name) {
                                results.push(format_port_version(&registry, name, version));
                            }
                        }
                    }
                } else if port_name.starts_with("*") {
                    // postfix
                    let query = util::str::reverse_string(port_name.split_at(1).1);
                    if let Some(mut data) = index.postfix_trie.get_data(&query, true) {
                        data.sort();
                        for name in data {
                            if let Some(version) = index.baseline.default.get(name) {
                                results.push(format_port_version(&registry, name, version));
                            }
                        }
                    }
                } else {
                    // extract match
                    if index.baseline.default.contains_key(port_name) {
                        if let Some(version) = index.baseline.default.get(port_name) {
                            if !list_all {
                                results.push(format_port_version(&registry, port_name, version));
                            } else {
                                for (v, c, d) in vcpkg_manager.get_port_versions(
                                    &vcpkg_root_dir,
                                    &registry,
                                    &port_name,
                                ) {
                                    results.push(format!("[{registry}]  {}  {}  {}", v, c, d));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return results;
}

fn format_port_version(registry: &str, name: &str, version: &VcpkgPortVersion) -> String {
    format!("[{registry}]  {}  {}", name, version.format_version_text())
}
