use super::{index::VcpkgSearchIndex, VcpkgManager};

use crate::{cli::commands::VcpkgArgs, config, util};

pub fn from_index_file(port_name: &str, list_all: bool) -> Vec<String> {
    let mut results = vec![];

    let vcpkg_manager = VcpkgManager::new(VcpkgArgs::load_or_default());

    for (registry, _url, _branch, _vcpkg_root_dir) in vcpkg_manager.args.flatten_registry() {
        match VcpkgSearchIndex::load(
            &config::system_paths::DataPath::vcpkg_search_index_json(
                vcpkg_manager.args.index_directory.as_ref().unwrap(),
                &registry,
            ),
            true,
        ) {
            None => return results,
            Some(index) => {
                if port_name.starts_with("*") && port_name.ends_with("*") {
                    // contains
                    let mut query = port_name.split_at(1).1;
                    query = query.split_at(query.len() - 1).0;
                    for (name, versions) in &index.versions {
                        if name.contains(query) {
                            results.push(format_port_version(
                                &registry,
                                name,
                                versions.last().unwrap(),
                            ));
                        }
                    }
                } else if port_name.ends_with("*") {
                    // prefix
                    let query = port_name.split_at(port_name.len() - 1).0;
                    if let Some(mut data) = index.prefix_trie.get_data(&query, true) {
                        data.sort();
                        for name in data {
                            if let Some(versions) = index.versions.get(name) {
                                results.push(format_port_version(
                                    &registry,
                                    name,
                                    versions.last().unwrap(),
                                ));
                            }
                        }
                    }
                } else if port_name.starts_with("*") {
                    // postfix
                    let query = util::str::reverse_string(port_name.split_at(1).1);
                    if let Some(mut data) = index.postfix_trie.get_data(&query, true) {
                        data.sort();
                        for name in data {
                            if let Some(versions) = index.versions.get(name) {
                                results.push(format_port_version(
                                    &registry,
                                    name,
                                    versions.last().unwrap(),
                                ));
                            }
                        }
                    }
                } else {
                    // extract match
                    if index.versions.contains_key(port_name) {
                        if let Some(versions) = index.versions.get(port_name) {
                            if !list_all {
                                results.push(format_port_version(
                                    &registry,
                                    port_name,
                                    versions.last().unwrap(),
                                ));
                            } else {
                                for v in versions {
                                    results.push(format_port_version(&registry, &port_name, v));
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

fn format_port_version(registry: &str, name: &str, version: &str) -> String {
    format!("[{registry}]  {}  {}", name, version)
}
