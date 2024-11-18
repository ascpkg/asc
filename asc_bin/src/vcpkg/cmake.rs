use std::collections::BTreeSet;

use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::{
    config::{self, project::PackageConfig},
    git::log::GitCommitInfo,
    templates, util,
};

#[derive(Default, Debug, Deserialize, Serialize)]
struct PortFileData {
    repo: String,
    commit: String,
    version: String,
    datetime: String,
    branch: String,
    patches: BTreeSet<String>,
}

pub fn gen_port_file_cmake(
    repo_root_dir: &String,
    package_conf: &PackageConfig,
    commit: &GitCommitInfo,
) -> bool {
    let port_file_path = config::system_paths::DataPath::vcpkg_ports_port_file_cmake_path(
        &repo_root_dir,
        &package_conf.name,
    );
    let port_file_text = std::fs::read_to_string(&port_file_path).unwrap_or_default();
    if port_file_text.contains(&commit.hash) {
        tracing::warn!(
            message = "the version and commit hash were not changed",
            commit_hash = commit.hash,
            commit_time = commit.date_time
        );
        return false;
    }

    let mut data = PortFileData {
        repo: package_conf.repository.clone(),
        commit: commit.hash.clone(),
        version: package_conf.version.clone(),
        datetime: commit.date_time.clone(),
        branch: package_conf.branch.clone(),
        ..Default::default()
    };

    let ports_dir_path =
        config::system_paths::DataPath::vcpkg_ports_dir_path(&repo_root_dir, &package_conf.name);
    for p in util::fs::find_patch_files(&ports_dir_path) {
        data.patches.insert(util::fs::remove_prefix(
            &p,
            &ports_dir_path,
            &ports_dir_path,
        ));
    }

    let reg = Handlebars::new();
    let text = reg
        .render_template(templates::PORT_FILE_CMAKE_HBS, &data)
        .unwrap();
    std::fs::write(&port_file_path, text.as_bytes()).unwrap();

    return true;
}
