use super::init;
use crate::cli::{config, template};

use clap::Args;

use handlebars::Handlebars;

use serde_json;

#[derive(Args, Debug, Clone, Default)]
pub struct NewArgs {
    pub name: Option<String>,
    #[clap(long, default_value_t = false)]
    pub lib: bool,
    #[clap(long, default_value_t = false)]
    pub workspace: bool,
    pub member: Option<Vec<String>>,
}

impl NewArgs {
    pub fn exec(&self) -> bool {
        if self.name.is_some() {
            if self.workspace && self.member.is_some() {
                return self.new_workspace();
            } else if !self.lib {
                return self.new_bin(self.name.as_ref().unwrap());
            } else {
                return self.new_lib(self.name.as_ref().unwrap());
            }
        }
        return false;
    }

    fn new_bin(&self, name: &str) -> bool {
        // write asc.toml
        if !self.new_package(name) {
            return false;
        }

        // write main.cpp
        return std::fs::write(
            format!(
                "{}/{}/{}",
                name,
                config::PROJECT_SRC_DIR,
                config::PROJECT_BIN_SRC
            ),
            template::NEW_BIN_HBS.as_bytes(),
        )
        .is_ok();
    }

    fn new_lib(&self, name: &str) -> bool {
        // write asc.toml
        if !self.new_package(name) {
            return false;
        }

        {
            // write export.h
            let reg = Handlebars::new();
            match reg.render_template(
                template::NEW_LIB_EXPORT_HBS,
                &serde_json::json!({"project_upper": name.to_uppercase()}),
            ) {
                Err(e) => {
                    tracing::error!(
                        message = "Handlebars::render_template",
                        template = template::NEW_LIB_EXPORT_HBS,
                        error = e.to_string()
                    );

                    return false;
                }
                Ok(text) => {
                    let path = format!(
                        "{}/{}/{}",
                        name,
                        config::PROJECT_SRC_DIR,
                        config::PROJECT_EXPORT_SRC
                    );
                    if let Err(e) = std::fs::write(&path, text.as_bytes()) {
                        tracing::error!(
                            message = "std::fs::write",
                            path = path,
                            text = text,
                            error = e.to_string()
                        );
                        return false;
                    }
                }
            }
        }

        {
            // write main.cpp
            let reg = Handlebars::new();
            match reg.render_template(
                template::NEW_LIB_MAIN_HBS,
                &serde_json::json!({"project_upper": name.to_uppercase()}),
            ) {
                Err(e) => {
                    tracing::error!(
                        message = "Handlebars::render_template",
                        template = template::NEW_LIB_EXPORT_HBS,
                        error = e.to_string()
                    );

                    return false;
                }
                Ok(text) => {
                    let path = format!(
                        "{}/{}/{}",
                        name,
                        config::PROJECT_SRC_DIR,
                        config::PROJECT_LIB_SRC
                    );
                    if let Err(e) = std::fs::write(&path, text.as_bytes()) {
                        tracing::error!(
                            message = "std::fs::write",
                            path = path,
                            text = text,
                            error = e.to_string()
                        );
                        return false;
                    }
                }
            }
        }

        return true;
    }

    fn new_package(&self, name: &str) -> bool {
        // validate args
        if name.is_empty() {
            return false;
        }

        // skip is exists
        if std::fs::metadata(name).is_ok() {
            tracing::error!(message = "std::fs::metadata", path = name, error = "exits");
            return false;
        }

        // create src dir
        let src_dir = format!("{name}/{}", config::PROJECT_SRC_DIR);
        if let Err(e) = std::fs::create_dir_all(src_dir) {
            tracing::error!(message = "std::fs::create_dir_all", error = e.to_string());
            return false;
        }

        let cwd = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // init
        std::env::set_current_dir(name).unwrap();
        let mut args = init::InitArgs::default();
        args.lib = self.lib;
        args.workspace = self.workspace;
        args.member = self.member.clone();
        return args.init_package(name) && std::env::set_current_dir(cwd).is_ok();
    }

    fn new_workspace(&self) -> bool {
        // validate args
        let name = self.name.as_ref().unwrap();
        let members = self.member.as_ref().unwrap();
        if name.is_empty() || members.is_empty() {
            return false;
        }

        // skip is exists
        if std::fs::metadata(name).is_ok() {
            tracing::error!(message = "std::fs::metadata", path = name, error = "exits");
            return false;
        }

        let cwd = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        if let Err(e) = std::fs::create_dir(name) {
            tracing::info!(
                message = "std::fs::create_dir",
                path = name,
                error = e.to_string()
            );
            return false;
        }

        // create members
        std::env::set_current_dir(name).unwrap();
        let mut has_error = false;
        let mut workspace = config::WorkSpaceConfig::default();
        for m in members {
            if workspace.members.insert(m.clone()) {
                if self.lib {
                    if !self.new_lib(m) {
                        has_error = true;
                    }
                } else {
                    if !self.new_bin(m) {
                        has_error = true;
                    }
                }
            }
        }
        let mut project = config::ProjectConfig::default();
        project.workspace = Some(workspace);

        std::env::set_current_dir(cwd).unwrap();

        // skip if exists
        let path = format!("{name}/{}", config::PROJECT_TOML);
        if std::fs::metadata(&path).is_ok() {
            tracing::error!(
                message = "std::fs::metadata",
                path = config::PROJECT_TOML,
                error = "exits"
            );
            return false;
        }

        // write asc.toml
        return !has_error && project.validate() && project.dump(&path);
    }
}
