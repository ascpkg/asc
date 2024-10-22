use clap::Args;

use crate::clang;
use crate::cli::config;
use crate::cmake;
use crate::errors::ErrorTag;
use crate::graph;
use crate::util;

#[derive(Clone, Debug, Default)]
pub struct ScanOptions {
    pub project: String,
    pub project_dir: String,
    pub build_dir: String,
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
    pub name: Option<String>,
    #[clap(long, default_value_t = false)]
    pub shared_lib: bool,
    #[clap(long, default_value_t = false)]
    pub static_lib: bool,
    #[clap(long, default_value = "3.20")]
    pub cmake_minimum_version: String,
}

impl ScanArgs {
    pub fn exec(&self) -> bool {
        if !config::ProjectConfig::is_conf_exists() {
            tracing::error!(
                call = "!config::ProjectConfig::is_conf_exists",
                path = config::PROJECT_TOML,
                error_tag = ErrorTag::FileNotFoundError.as_ref(),
                message = "please run asc init first"
            );
            return false;
        }

        match config::ProjectConfig::read_project_conf() {
            None => false,
            Some(project_conf) => {
                if project_conf.package.is_none()
                    && project_conf.bins.is_none()
                    && project_conf.libs.is_none()
                {
                    tracing::error!(
                        error_tag = ErrorTag::InvalidProjectPackageError.as_ref(),
                        message = "package, bins, libs were not found"
                    );
                    return false;
                }

                if project_conf.workspace.is_some() {
                    return self.scan_workspace();
                }

                let (target_name, target_src) =
                    project_conf.get_target_name_src(&self.name, self.shared_lib, self.static_lib);
                return self.scan_package(&target_name, &target_src);
            }
        }
    }

    pub fn scan_package(&self, name: &str, path: &str) -> bool {
        let cwd = util::fs::get_cwd();
        let options = ScanOptions {
            project: name.to_string(),
            project_dir: cwd.clone(),
            build_dir: format!("{cwd}/{}", config::PROJECT_TARGET_DIR),
            source_dir: format!("{cwd}/{}", config::PROJECT_SRC_DIR),
            entry_point_source: format!("{cwd}/{}", path),
            include_dirs: vec![],
            shared_lib: self.shared_lib,
            static_lib: self.static_lib,
            cmake_minimum_version: self.cmake_minimum_version.clone(),
            ..Default::default()
        };

        tracing::info!("{:#?}", options);

        // write empty files
        std::fs::create_dir(&options.build_dir).unwrap_or(());
        std::fs::write(format!("{}/config.h", &options.build_dir), b"").unwrap_or(());
        std::fs::write(format!("{}/version.h", &options.build_dir), b"").unwrap_or(());

        tracing::warn!("scan source dependencies with clang ir");
        let source_mappings = clang::parser::SourceMappings::scan(&options);

        tracing::warn!("output flow chart {}", graph::flowchart::path(&options));
        let mermaid_flowchart = graph::flowchart::gen(&options, &source_mappings);
        tracing::info!("\n{mermaid_flowchart}");

        tracing::warn!("output {}", cmake::path::cmake_lists_path(&options));
        cmake::lists::gen(&options, &source_mappings);

        tracing::warn!("generate a build system with cmake");
        cmake::project::gen(&options);
        return true;
    }

    pub fn scan_workspace(&self) -> bool {
        false
    }
}
