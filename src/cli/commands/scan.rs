use std::collections::BTreeSet;

use clap::Args;

use crate::clang;
use crate::cli::config;
use crate::cmake;
use crate::graph;

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
        if std::fs::metadata(config::PROJECT_TOML).is_err() {
            tracing::error!("{} not found, please run init first", config::PROJECT_TOML);
            return false;
        }

        match config::ProjectConfig::load(config::PROJECT_TOML) {
            None => false,
            Some(project_conf) => {
                if project_conf.package.is_none()
                    && project_conf.bins.is_none()
                    && project_conf.libs.is_none()
                {
                    tracing::error!("package, bins, libs were not found");
                    return false;
                }

                if project_conf.workspace.is_some() {
                    return self.scan_workspace();
                }

                let mut package_name = String::new();
                if project_conf.package.is_some() {
                    package_name = project_conf.package.as_ref().unwrap().name.clone();
                }

                if !self.shared_lib && !self.static_lib {
                    // bin
                    let (n, p) = self.get_name_path(
                        &project_conf.bins,
                        &package_name,
                        &format!("{}/{}", config::PROJECT_SRC_DIR, config::PROJECT_BIN_SRC),
                    );
                    if n.is_empty() || p.is_empty() {
                        return false;
                    }
                    return self.scan_package(&n, &p);
                } else {
                    // lib
                    let (n, p) = self.get_name_path(
                        &project_conf.libs,
                        &package_name,
                        &format!("{}/{}", config::PROJECT_SRC_DIR, config::PROJECT_LIB_SRC),
                    );
                    if n.is_empty() || p.is_empty() {
                        return false;
                    }
                    return self.scan_package(&n, &p);
                }
            }
        }
    }

    fn get_name_path(
        &self,
        entries: &Option<BTreeSet<config::EntryConfig>>,
        package_name: &str,
        default_path: &str,
    ) -> (String, String) {
        // no bins and libs, use package
        if entries.is_none() || entries.as_ref().unwrap().is_empty() {
            if package_name.is_empty() {
                return (String::new(), String::new());
            }
            return (package_name.to_string(), default_path.to_string());
        } else {
            // try to use bins/libs
            if self.name.is_none() {
                return (String::new(), String::new());
            }
            let name = self.name.as_ref().unwrap();
            // validate name
            if name.is_empty() {
                return (String::new(), String::new());
            }
            // validate bins/libs
            let mut path = String::new();
            for entry in entries.as_ref().unwrap() {
                if &entry.name == name {
                    path = entry.path.clone();
                    break;
                }
            }
            if path.is_empty() {
                return (String::new(), String::new());
            }

            return (name.clone(), path);
        }
    }

    pub fn scan_package(&self, name: &str, path: &str) -> bool {
        let cwd = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace(r"\", "/");
        let options = ScanOptions {
            project: name.to_string(),
            project_dir: cwd.clone(),
            build_dir: format!("{cwd}/{}", config::PROJECT_TARGET_DIR.to_string()),
            source_dir: format!("{cwd}/{}", config::PROJECT_SRC_DIR.to_string()),
            entry_point_source: format!("{cwd}/{}", path),
            include_dirs: vec![],
            shared_lib: self.shared_lib,
            static_lib: self.static_lib,
            cmake_minimum_version: self.cmake_minimum_version.clone(),
            cmake_config: String::new(),
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

        return true;
    }

    pub fn scan_workspace(&self) -> bool {
        false
    }
}
