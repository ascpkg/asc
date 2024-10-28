use super::index::{VcpkgBaseline, VcpkgPortVersion, VcpkgSearchIndex};

use crate::{cli::commands::search::SearchArgs, config, util};

pub fn from_index_file(args: &SearchArgs) -> Vec<String> {
    let mut results = vec![];

    match VcpkgSearchIndex::load(&config::dir::DataDir::vcpkg_search_index_json(), false) {
        None => return results,
        Some(index) => {
            let baseline_path = config::dir::DataDir::vcpkg_search_baseline_json();
            if !util::fs::is_file_exists(&baseline_path) {
                return results;
            }

            match VcpkgBaseline::load(&baseline_path, false) {
                None => return results,
                Some(baseline) => {
                    if args.name.starts_with("*") && args.name.ends_with("*") {
                        // contains
                        let mut query = args.name.split_at(1).1;
                        query = query.split_at(query.len() - 1).0;
                        for (name, version) in &baseline.default {
                            if name.contains(query) {
                                results.push(format_port_version(name, version));
                            }
                        }
                    } else if args.name.ends_with("*") {
                        // prefix
                        let query = args.name.split_at(args.name.len() - 1).0;
                        if let Some(mut data) = index.prefix.get_data(&query, true) {
                            data.sort();
                            for name in data {
                                if let Some(version) = baseline.default.get(name) {
                                    results.push(format_port_version(name, version));
                                }
                            }
                        }
                    } else if args.name.starts_with("*") {
                        // postfix
                        let query = util::str::reverse_string(&args.name.split_at(1).1);
                        if let Some(mut data) = index.postfix.get_data(&query, true) {
                            data.sort();
                            for name in data {
                                if let Some(version) = baseline.default.get(name) {
                                    results.push(format_port_version(name, version));
                                }
                            }
                        }
                    } else {
                        // extract match
                        if baseline.default.contains_key(&args.name) {
                            if let Some(version) = baseline.default.get(&args.name) {
                                results.push(format_port_version(&args.name, version));
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
