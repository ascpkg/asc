use std::collections::BTreeMap;

use clap::Args;

use crate::clang;
use crate::cmake;
use crate::config;
use crate::config::project::DependencyConfig;
use crate::config::project::ProjectConfig;
use crate::config::relative_paths;
use crate::errors::ErrorTag;
use crate::graph;
use crate::util;
use crate::vcpkg;

#[derive(Clone, Debug, Default)]
pub struct ScanOptions {
    pub project: String,
    pub project_dir: String,
    pub target_dir: String,
    pub source_dir: String,
    pub entry_point_source: String,
    pub shared_lib: bool,
    pub static_lib: bool,
    pub std_c: String,
    pub std_cxx: String,
    pub cmake_config: String,
    pub cmake_minimum_version: String,
}

#[derive(Args, Debug, Clone)]
/// scan necessary sources, generate cmake and vcpkg configurations 
pub struct ScanArgs {
    /// for cmake cmake_minimum_required
    #[clap(long, default_value = "3.20")]
    pub cmake_minimum_version: String,
}

impl ScanArgs {
    pub fn exec(&self) -> bool {
        if !config::project::ProjectConfig::is_project_inited(false) {
            return false;
        }

        match config::project::ProjectConfig::read_project_conf() {
            None => false,
            Some(project_conf) => {
                if project_conf.workspace.is_some() {
                    return self.scan_workspace(&project_conf);
                }

                if project_conf.bins.is_empty() && project_conf.libs.is_empty() {
                    tracing::error!(
                        error_tag = ErrorTag::InvalidProjectPackageError.as_ref(),
                        message = "bins, libs were not found"
                    );
                    return false;
                }

                // cd .asc
                if !util::fs::is_dir_exists(relative_paths::ASC_PROJECT_DIR_NAME) {
                    util::fs::create_dir(relative_paths::ASC_PROJECT_DIR_NAME);
                }
                let cwd = util::fs::get_cwd();
                util::fs::set_cwd(relative_paths::ASC_PROJECT_DIR_NAME);

                let mut members = vec![];
                for bin_entry in &project_conf.bins {
                    members.push(bin_entry.name.clone());

                    if !util::fs::is_dir_exists(&bin_entry.name) {
                        util::fs::create_dir(&bin_entry.name);
                    }
                    let c = util::fs::get_cwd();
                    // cd bin_entry.name
                    util::fs::set_cwd(&bin_entry.name);

                    self.scan_package(
                        &bin_entry.name,
                        &cwd,
                        &format!("{cwd}/{}", bin_entry.source_dir),
                        &format!("{cwd}/{}/{}", bin_entry.source_dir, bin_entry.source_file),
                        &format!(
                            "{cwd}/{}/{}",
                            relative_paths::ASC_TARGET_DIR_NAME,
                            bin_entry.name
                        ),
                        true,
                        &project_conf.dependencies,
                        false,
                        false,
                        &bin_entry.std_c,
                        &bin_entry.std_cxx,
                    );

                    // cd .asc
                    util::fs::set_cwd(&c);
                }

                for lib_entry in &project_conf.libs {
                    members.push(lib_entry.name.clone());

                    if !util::fs::is_dir_exists(&lib_entry.name) {
                        util::fs::create_dir(&lib_entry.name);
                    }
                    let c = util::fs::get_cwd();
                    // cd lib_entry.name
                    util::fs::set_cwd(&lib_entry.name);

                    let is_shared_lib = lib_entry.shared.unwrap();
                    self.scan_package(
                        &lib_entry.name,
                        &cwd,
                        &format!("{cwd}/{}", lib_entry.source_dir),
                        &format!("{cwd}/{}/{}", lib_entry.source_dir, lib_entry.source_file),
                        &format!(
                            "{cwd}/{}/{}",
                            relative_paths::ASC_TARGET_DIR_NAME,
                            lib_entry.name
                        ),
                        true,
                        &project_conf.dependencies,
                        is_shared_lib,
                        !is_shared_lib,
                        &lib_entry.std_c,
                        &lib_entry.std_cxx,
                    );

                    // cd .asc
                    util::fs::set_cwd(&c);
                }

                cmake::lists::gen_workspace(
                    &self.cmake_minimum_version,
                    &project_conf.package.unwrap().name,
                    &members,
                );

                tracing::warn!("generate vcpkg manifest");
                vcpkg::json::gen_vcpkg_configurations(&project_conf.dependencies);

                tracing::warn!("generate a build system with cmake");
                let options = ScanOptions {
                    project_dir: format!("{cwd}/{}", relative_paths::ASC_PROJECT_DIR_NAME),
                    target_dir: format!("{cwd}/{}", relative_paths::ASC_TARGET_DIR_NAME),
                    shared_lib: false,
                    ..Default::default()
                };
                cmake::project::gen(&options);

                return true;
            }
        }
    }

    pub fn scan_package(
        &self,
        name: &str,
        root_dir: &str,
        src_dir: &str,
        src_path: &str,
        taget_dir: &str,
        is_workspace: bool,
        dependencies: &BTreeMap<String, DependencyConfig>,
        is_shared_lib: bool,
        is_static_lib: bool,
        std_c: &str,
        std_cxx: &str,
    ) -> bool {
        tracing::info!(message = "scan package", name = name);

        let options = ScanOptions {
            project: name.to_string(),
            project_dir: root_dir.to_string(),
            target_dir: taget_dir.to_string(),
            source_dir: src_dir.to_string(),
            entry_point_source: src_path.to_string(),
            shared_lib: is_shared_lib,
            static_lib: is_static_lib,
            std_c: std_c.to_string(),
            std_cxx: std_cxx.to_string(),
            cmake_minimum_version: self.cmake_minimum_version.clone(),
            ..Default::default()
        };

        tracing::info!("{:#?}", options);

        // write empty files
        util::fs::create_dirs(&&options.target_dir);
        std::fs::write(
            format!(
                "{}/{}",
                &options.target_dir,
                relative_paths::CONFIG_H_FILE_NAME
            ),
            b"",
        )
        .unwrap_or(());
        std::fs::write(
            format!(
                "{}/{}",
                &options.target_dir,
                relative_paths::VERSION_H_FILE_NAME
            ),
            b"",
        )
        .unwrap_or(());

        tracing::warn!("scan source dependencies with clang ir");
        let source_mappings = clang::parser::SourceMappings::scan(&options);

        tracing::warn!(
            "output flow chart {}",
            relative_paths::FLOW_CHART_MD_FILE_NAME
        );
        let mermaid_flowchart = graph::flowchart::gen(&options, &source_mappings);
        tracing::info!("\n{mermaid_flowchart}");

        tracing::warn!("output {}", relative_paths::CMAKE_LISTS_TXT_FILE_NAME);
        cmake::lists::gen(&options, &source_mappings, is_workspace, dependencies);

        return true;
    }

    pub fn scan_workspace(&self, project_conf: &ProjectConfig) -> bool {
        tracing::info!(message = "scan workspace", name = util::fs::get_cwd_name());

        // cd .asc
        if !util::fs::is_dir_exists(relative_paths::ASC_PROJECT_DIR_NAME) {
            util::fs::create_dir(relative_paths::ASC_PROJECT_DIR_NAME);
        }
        let cwd = util::fs::get_cwd();
        util::fs::set_cwd(relative_paths::ASC_PROJECT_DIR_NAME);

        let mut has_error = false;
        let mut members = vec![];
        let mut dependencies = BTreeMap::new();
        let is_shared_lib = false;
        for member in &project_conf.workspace.as_ref().unwrap().members {
            match config::project::ProjectConfig::load(
                &format!("{cwd}/{member}/{}", relative_paths::ASC_TOML_FILE_NAME),
                false,
            ) {
                None => {
                    has_error = true;
                }
                Some(project_conf) => {
                    for bin_entry in &project_conf.bins {
                        members.push(bin_entry.name.clone());

                        if !util::fs::is_dir_exists(&bin_entry.name) {
                            util::fs::create_dir(&bin_entry.name);
                        }
                        let c = util::fs::get_cwd();
                        util::fs::set_cwd(&bin_entry.name);

                        self.scan_package(
                            &bin_entry.name,
                            &cwd,
                            &format!("{cwd}/{member}/{}", bin_entry.source_dir),
                            &format!(
                                "{cwd}/{member}/{}/{}",
                                bin_entry.source_dir, bin_entry.source_file
                            ),
                            &format!(
                                "{cwd}/{}/{}",
                                relative_paths::ASC_TARGET_DIR_NAME,
                                bin_entry.name
                            ),
                            true,
                            &project_conf.dependencies,
                            false,
                            false,
                            &bin_entry.std_c,
                            &bin_entry.std_cxx,
                        );

                        util::fs::set_cwd(&c);
                    }

                    for lib_entry in &project_conf.libs {
                        members.push(lib_entry.name.clone());

                        if !util::fs::is_dir_exists(&lib_entry.name) {
                            util::fs::create_dir(&lib_entry.name);
                        }
                        let c = util::fs::get_cwd();
                        util::fs::set_cwd(&lib_entry.name);

                        let is_shared_lib = lib_entry.shared.unwrap();
                        self.scan_package(
                            &lib_entry.name,
                            &cwd,
                            &format!("{cwd}/{member}/{}", lib_entry.source_dir),
                            &format!(
                                "{cwd}/{member}/{}/{}",
                                lib_entry.source_dir, lib_entry.source_file
                            ),
                            &format!(
                                "{cwd}/{}/{}",
                                relative_paths::ASC_TARGET_DIR_NAME,
                                lib_entry.name
                            ),
                            true,
                            &project_conf.dependencies,
                            is_shared_lib,
                            !is_shared_lib,
                            &lib_entry.std_c,
                            &lib_entry.std_cxx,
                        );

                        util::fs::set_cwd(&c);
                    }

                    dependencies.extend(project_conf.dependencies);
                }
            }
        }

        cmake::lists::gen_workspace(
            &self.cmake_minimum_version,
            &util::fs::get_file_name(&cwd),
            &members,
        );

        tracing::warn!("generate vcpkg manifest");
        vcpkg::json::gen_vcpkg_configurations(&dependencies);

        tracing::warn!("generate a build system with cmake");
        let options = ScanOptions {
            project_dir: format!("{cwd}/{}", relative_paths::ASC_PROJECT_DIR_NAME),
            target_dir: format!("{cwd}/{}", relative_paths::ASC_TARGET_DIR_NAME),
            shared_lib: is_shared_lib,
            ..Default::default()
        };
        cmake::project::gen(&options);

        util::fs::set_cwd(&cwd);

        return has_error;
    }
}
