use super::{
    index::{VcpkgPortVersion, VcpkgSearchIndex},
    VcpkgManager,
};

use crate::{config, util};

pub fn from_index_file(port_name: &str, list_all: bool) -> Vec<String> {
    let mut results = vec![];

    match VcpkgSearchIndex::load(&config::dir::DataDir::vcpkg_search_index_json(), false) {
        None => return results,
        Some(index) => {
            if port_name.starts_with("*") && port_name.ends_with("*") {
                // contains
                let mut query = port_name.split_at(1).1;
                query = query.split_at(query.len() - 1).0;
                for (name, version) in &index.baseline.default {
                    if name.contains(query) {
                        results.push(format_port_version(name, version));
                    }
                }
            } else if port_name.ends_with("*") {
                // prefix
                let query = port_name.split_at(port_name.len() - 1).0;
                if let Some(mut data) = index.prefix_trie.get_data(&query, true) {
                    data.sort();
                    for name in data {
                        if let Some(version) = index.baseline.default.get(name) {
                            results.push(format_port_version(name, version));
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
                            results.push(format_port_version(name, version));
                        }
                    }
                }
            } else {
                // extract match
                if index.baseline.default.contains_key(port_name) {
                    if let Some(version) = index.baseline.default.get(port_name) {
                        if !list_all {
                            results.push(format_port_version(port_name, version));
                        } else {
                            for (v, c, d) in VcpkgManager::get_port_versions(port_name) {
                                results.push(format!("{}  {}  {}", v, c, d));
                            }
                        }
                    }
                }
            }
        }
    }

    return results;
}

fn format_port_version(name: &str, version: &VcpkgPortVersion) -> String {
    format!(
        "{}  {}",
        name,
        if version.port_version == 0 {
            version.baseline.clone()
        } else {
            format!("{}#{}", version.baseline, version.port_version)
        }
    )
}
