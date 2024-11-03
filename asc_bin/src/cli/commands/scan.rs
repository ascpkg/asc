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
    pub include_dirs: Vec<String>,
    pub shared_lib: bool,
    pub static_lib: bool,
    pub cmake_minimum_version: String,
    pub cmake_config: String,
}

#[derive(Args, Debug, Clone)]
pub struct ScanArgs {
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

                if project_conf.bins.is_none() && project_conf.libs.is_none() {
                    tracing::error!(
                        error_tag = ErrorTag::InvalidProjectPackageError.as_ref(),
                        message = "bins, libs were not found"
                    );
                    return false;
                }

                let (target_name, target_src, is_shared_lib, is_static_lib) =
                    project_conf.get_target_name_src();
                if target_name.is_empty() || target_src.is_empty() {
                    tracing::error!(
                        func = "target_name.is_empty || target_src.is_empty",
                        error_tag = ErrorTag::InvalidCliArgsError.as_ref()
                    );
                }
                return self.scan_package(
                    &target_name,
                    &target_src,
                    false,
                    &project_conf.dependencies,
                    is_shared_lib,
                    is_static_lib,
                );
            }
        }
    }

    pub fn scan_package(
        &self,
        name: &str,
        path: &str,
        is_workspace: bool,
        dependencies: &BTreeMap<String, DependencyConfig>,
        is_shared_lib: bool,
        is_static_lib: bool,
    ) -> bool {
        tracing::info!(message = "scan package", name = name);

        let cwd = util::fs::get_cwd();
        let options = ScanOptions {
            project: name.to_string(),
            project_dir: cwd.clone(),
            target_dir: if !is_workspace {
                format!("{cwd}/{}", relative_paths::ASC_TARGET_DIR_NAME)
            } else {
                format!(
                    "{}/{}/{}",
                    util::fs::get_cwd_parent(),
                    relative_paths::ASC_TARGET_DIR_NAME,
                    name
                )
            },
            source_dir: format!("{cwd}/{}", relative_paths::SRC_DIR_NAME),
            entry_point_source: format!("{cwd}/{}", path),
            include_dirs: vec![],
            shared_lib: is_shared_lib,
            static_lib: is_static_lib,
            cmake_minimum_version: self.cmake_minimum_version.clone(),
            ..Default::default()
        };

        tracing::info!("{:#?}", options);

        // write empty files
        std::fs::create_dir_all(&options.target_dir).unwrap_or(());
        std::fs::write(format!("{}/config.h", &options.target_dir), b"").unwrap_or(());
        std::fs::write(format!("{}/version.h", &options.target_dir), b"").unwrap_or(());

        tracing::warn!("scan source dependencies with clang ir");
        let source_mappings = clang::parser::SourceMappings::scan(&options);

        tracing::warn!("output flow chart {}", graph::flowchart::path(&options));
        let mermaid_flowchart = graph::flowchart::gen(&options, &source_mappings);
        tracing::info!("\n{mermaid_flowchart}");

        tracing::warn!("output {}", relative_paths::CMAKE_LISTS_TXT_FILE_NAME);
        cmake::lists::gen(&options, &source_mappings, is_workspace, dependencies);

        if !is_workspace {
            tracing::warn!("generate vcpkg manifest");
            vcpkg::json::gen(&dependencies);

            tracing::warn!("generate a build system with cmake");
            cmake::project::gen(&options);
        }
        return true;
    }

    pub fn scan_workspace(&self, project_conf: &ProjectConfig) -> bool {
        tracing::info!(message = "scan workspace", name = util::fs::get_cwd_name());

        let cwd = util::fs::get_cwd();
        let mut has_error = false;
        let mut members = vec![];
        let mut dependencies = BTreeMap::new();
        let is_shared_lib = false;
        for member in &project_conf.workspace.as_ref().unwrap().members {
            util::fs::set_cwd(member);

            match config::project::ProjectConfig::read_project_conf() {
                None => {
                    has_error = true;
                }
                Some(project_conf) => {
                    let (target_name, target_src, is_shared_lib, is_static_lib) =
                        project_conf.get_target_name_src();
                    if !target_name.is_empty() && !target_src.is_empty() {
                        if self.scan_package(
                            &target_name,
                            &target_src,
                            true,
                            &project_conf.dependencies,
                            is_shared_lib,
                            is_static_lib,
                        ) {
                            members.push(member.clone());
                            dependencies.extend(project_conf.dependencies);
                        } else {
                            has_error = true;
                        }
                    } else {
                        has_error = true;
                        tracing::error!(
                            func = "target_name.is_empty || target_src.is_empty",
                            error_tag = ErrorTag::InvalidCliArgsError.as_ref()
                        );
                    }
                }
            }

            util::fs::set_cwd(&cwd);
        }

        cmake::lists::gen_workspace(
            &self.cmake_minimum_version,
            &util::fs::get_cwd_name(),
            &members,
        );

        tracing::warn!("generate vcpkg manifest");
        vcpkg::json::gen(&dependencies);

        tracing::warn!("generate a build system with cmake");
        let options = ScanOptions {
            project_dir: cwd.clone(),
            target_dir: format!("{cwd}/{}", relative_paths::ASC_TARGET_DIR_NAME),
            shared_lib: is_shared_lib,
            ..Default::default()
        };
        cmake::project::gen(&options);

        return has_error;
    }
}
