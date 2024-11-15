use std::collections::BTreeSet;

use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::{
    cli::commands::VcpkgArgs,
    config::{self, project::PackageConfig},
    git, templates,
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

pub fn gen_port_file_cmake(package_conf: &PackageConfig) -> bool {
    let vcpkg_conf = VcpkgArgs::load_or_default();
    let repo_root_dir = vcpkg_conf.directory.as_ref().unwrap();

    let commit = git::log::get_latest_commit(".", git::log::GIT_LOG_FORMAT);

    let path = config::system_paths::DataPath::vcpkg_port_file_cmake_path(
        &repo_root_dir,
        &package_conf.name,
    );
    let exists_text = std::fs::read_to_string(&path).unwrap_or_default();
    if exists_text.contains(&commit.hash) {}

    let data = PortFileData {
        repo: package_conf.repository.clone(),
        commit: commit.hash,
        version: package_conf.version.clone(),
        datetime: commit.date_time,
        branch: package_conf.branch.clone(),
        ..Default::default()
    };
    let reg = Handlebars::new();
    let text = reg
        .render_template(templates::PORT_FILE_CMAKE_HBS, &data)
        .unwrap();
    std::fs::write(&path, text.as_bytes()).unwrap();

    return true;
}
