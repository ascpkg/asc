use std::collections::{BTreeMap, BTreeSet, HashMap};

use chrono::Datelike;

use handlebars::Handlebars;

use serde::{Deserialize, Serialize};

use crate::clang;
use crate::cli;
use crate::config::project::DependencyConfig;
use crate::config::relative_paths;
use crate::templates;
use crate::util;

#[derive(Default, Debug, Deserialize, Serialize)]
struct SourcesGroup {
    dir: String,
    original_dir: String,
    variable: String,
    files: Vec<String>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
struct InstallHeader {
    src: String,
    dst: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
struct CMakeListsData {
    cmake_version: String,
    is_workspace: bool,
    project: String,
    project_upper: String,
    build_year: i32,
    build_month: u32,
    build_day: u32,
    user_cmake_txt: String,
    install_bin_dir: String,
    install_lib_dir: String,
    install_include_dir: String,
    install_share_dir: String,
    executable: bool,
    library: bool,
    shared_library: bool,
    sources_group_by_dir: Vec<SourcesGroup>,
    include_dirs: Vec<String>,
    find_packages: Vec<String>,
    link_libraries: bool,
    link_public_libraries: bool,
    public_libraries: Vec<String>,
    link_private_libraries: bool,
    private_libraries: Vec<String>,
    install_headers: Vec<InstallHeader>,
}

pub fn gen(
    options: &cli::commands::scan::ScanOptions,
    source_mappings: &clang::parser::SourceMappings,
    is_workspace: bool,
    dependencies: &BTreeMap<String, DependencyConfig>,
) {
    // output default config.in.cm if not exists
    if !util::fs::is_file_exists(relative_paths::CONFIG_H_CM_FILE_NAME) {
        std::fs::write(
            relative_paths::CONFIG_H_CM_FILE_NAME,
            templates::CONFIG_H_CM_HBS.as_bytes(),
        )
        .unwrap();
    }

    // output default user.cmake if not exists
    if !util::fs::is_file_exists(relative_paths::USER_CMAKE_FILE_NAME) {
        std::fs::write(
            relative_paths::USER_CMAKE_FILE_NAME,
            templates::USER_CMAKE_HBS.as_bytes(),
        )
        .unwrap()
    }

    // group data
    let (group_sources, classify_to_dir, install_headers) = group_sources(options, source_mappings);

    // init data
    let local_date_time = chrono::prelude::Local::now();
    let mut data = CMakeListsData::default();
    data.is_workspace = is_workspace;
    data.cmake_version = options.cmake_minimum_version.clone();
    data.project = options.project.clone();
    data.project_upper = options.project.to_uppercase();
    data.build_year = local_date_time.year();
    data.build_month = local_date_time.month();
    data.build_day = local_date_time.day();
    data.user_cmake_txt =
        std::fs::read_to_string(relative_paths::USER_CMAKE_FILE_NAME).unwrap_or(String::new());
    data.install_bin_dir = relative_paths::CMAKE_INSTALL_BIN_DIR_NAME.to_string();
    data.install_lib_dir = relative_paths::CMAKE_INSTALL_LIB_DIR_NAME.to_string();
    data.install_include_dir = relative_paths::CMAKE_INSTALL_INCLUDE_DIR_NAME.to_string();
    data.install_share_dir = relative_paths::CMAKE_INSTALL_SHARE_DIR_NAME.to_string();
    data.executable = !options.static_lib && !options.shared_lib;
    data.library = options.static_lib || options.shared_lib;
    data.shared_library = data.library && options.shared_lib;
    data.include_dirs = options.include_dirs.clone();
    data.link_libraries = !dependencies.is_empty();
    data.link_public_libraries = false;
    data.link_private_libraries = !dependencies.is_empty();
    for (_, dep) in dependencies {
        if !dep.find_packages.is_empty() {
            data.find_packages.push(
                dep.find_packages
                    .iter()
                    .map(|s| s.clone())
                    .collect::<Vec<String>>()
                    .join(" "),
            );
        }
        if !dep.link_libraries.is_empty() {
            data.private_libraries.push(
                dep.link_libraries
                    .iter()
                    .map(|s| s.clone())
                    .collect::<Vec<String>>()
                    .join(" "),
            );
        }
    }

    for (dir, sources) in &group_sources {
        let mut group = SourcesGroup::default();
        group.dir = dir.clone();
        group.original_dir = classify_to_dir.get(dir).unwrap().clone();
        group.variable = format!("${}{}{}", "{", dir, "}");
        for src in sources {
            group.files.push(src.clone());
        }
        data.sources_group_by_dir.push(group);
    }

    for (src, dst) in install_headers {
        data.install_headers
            .push(InstallHeader { src: src, dst: dst });
    }

    // render template
    {
        // write project-config.cmake.in
        let reg = Handlebars::new();
        let text = reg
            .render_template(templates::PROJECT_CONFIG_CMAKE_IN_HBS, &data)
            .unwrap();
        std::fs::write(
            relative_paths::get_config_cmake_in_file_name(&options.project),
            text.as_bytes(),
        )
        .unwrap();
    }

    {
        // write version.h.in
        let reg = Handlebars::new();
        let text = reg
            .render_template(templates::VERSION_H_IN_HBS, &data)
            .unwrap();
        std::fs::write(relative_paths::VERSION_H_IN_FILE_NAME, text.as_bytes()).unwrap();
    }

    {
        // write CMakeLists.txt
        let reg = Handlebars::new();
        let text = reg
            .render_template(templates::PROJECT_CMAKE_LISTS_TXT_HBS, &data)
            .unwrap();
        std::fs::write(relative_paths::CMAKE_LISTS_TXT_FILE_NAME, text.as_bytes()).unwrap();
    }
}

pub fn gen_workspace(cmake_minimum_version: &str, project: &str, members: &Vec<String>) {
    let data = serde_json::json!({
        "cmake_version": cmake_minimum_version,
        "project": project,
        "members": members,
    });

    // write CMakeLists.txt
    let reg = Handlebars::new();
    let text = reg
        .render_template(templates::WORKSPACE_CMAKE_LISTS_TXT_HBS, &data)
        .unwrap();
    std::fs::write(relative_paths::CMAKE_LISTS_TXT_FILE_NAME, text.as_bytes()).unwrap();
}

fn group_sources(
    options: &cli::commands::scan::ScanOptions,
    source_mappings: &clang::parser::SourceMappings,
) -> (
    BTreeMap<String, BTreeSet<String>>,
    HashMap<String, String>,
    BTreeMap<String, String>,
) {
    // group sources by dir name
    let mut group_sources = BTreeMap::<String, BTreeSet<String>>::new();
    let mut classify_to_dir = HashMap::<String, String>::new();
    let mut install_headers = BTreeMap::<String, String>::new();
    for (header, sources) in &source_mappings.header_include_by_sources {
        {
            let header_locate_dir = util::fs::get_parent_dir(header);

            // prepare install headers's src and dst
            let src = if header.starts_with(&options.source_dir) {
                header.clone()
            } else {
                format!(
                    "${{CMAKE_CURRENT_BINARY_DIR}}/{}",
                    util::fs::remove_prefix(header, &options.source_dir, &options.target_dir)
                )
            };

            let dst = if header_locate_dir.starts_with(&options.source_dir) {
                util::fs::remove_prefix(
                    &header_locate_dir,
                    &options.source_dir,
                    &options.target_dir,
                )
            } else {
                String::new()
            };
            install_headers.insert(src, dst);
        }

        {
            // group header
            let relative_path: String =
                util::fs::remove_prefix(header, &options.project_dir, &options.target_dir);
            let dir = util::fs::get_parent_dir(&relative_path);
            let classify = dir.replace("/", "_");
            classify_to_dir.insert(classify.clone(), dir.to_string());
            let header_path = util::fs::replace_common_prefix(
                &header,
                &options.source_dir,
                &options.target_dir,
                "../../",
            );
            group_sources
                .entry(classify.to_string())
                .or_default()
                .insert(header_path);
        }

        {
            for src in sources {
                // group source
                let relative_path: String =
                    util::fs::remove_prefix(src, &options.project_dir, &options.target_dir);
                let dir = util::fs::get_parent_dir(&relative_path);
                let classify = dir.replace("/", "_");
                classify_to_dir.insert(classify.clone(), dir.to_string());
                let src_path = util::fs::replace_common_prefix(
                    &src,
                    &options.source_dir,
                    &options.target_dir,
                    "../../",
                );
                group_sources
                    .entry(classify.to_string())
                    .or_default()
                    .insert(src_path);
            }
        }
    }

    return (group_sources, classify_to_dir, install_headers);
}
