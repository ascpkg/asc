use clap::Args;

use handlebars::Handlebars;

use serde_json;

use super::init;
use crate::errors::ErrorTag;
use crate::{config, config::relative_paths, templates, util};

#[derive(Args, Debug, Clone, Default)]
pub struct NewArgs {
    pub name: Option<String>,

    #[clap(long, default_value_t = false)]
    pub lib: bool,

    #[clap(long, default_value_t = false)]
    pub shared: bool,

    #[clap(long, default_value_t = false)]
    pub workspace: bool,

    #[clap(long)]
    pub member: Vec<String>,
}

impl NewArgs {
    pub fn exec(&self) -> bool {
        if self.name.is_some() {
            if self.workspace {
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
        tracing::info!(message = "new bin", name = name);

        // write asc.toml
        if !self.new_package(name) {
            return false;
        }

        // write main.cpp
        return std::fs::write(
            format!(
                "{}/{}/{}",
                name,
                relative_paths::SRC_DIR_NAME,
                relative_paths::MAIN_CPP_FILE_NAME
            ),
            templates::MAIN_CPP_HBS.as_bytes(),
        )
        .is_ok();
    }

    fn new_lib(&self, name: &str) -> bool {
        tracing::info!(message = "new lib", name = name);

        // write asc.toml
        if !self.new_package(name) {
            return false;
        }

        {
            // write export.h
            let reg = Handlebars::new();
            match reg.render_template(
                templates::EXPORT_H_HBS,
                &serde_json::json!({"project_upper": name.to_uppercase()}),
            ) {
                Err(e) => {
                    tracing::error!(
                        func = "Handlebars::render_template",
                        template = templates::EXPORT_H_HBS,
                        error_tag = ErrorTag::RenderHandlebarsError.as_ref(),
                        error_str = e.to_string()
                    );

                    return false;
                }
                Ok(text) => {
                    let path = format!(
                        "{}/{}/{}",
                        name,
                        relative_paths::SRC_DIR_NAME,
                        relative_paths::EXPORT_H_FILE_NAME
                    );
                    if let Err(e) = std::fs::write(&path, text.as_bytes()) {
                        tracing::error!(
                            func = "std::fs::write",
                            path = path,
                            error_tag = ErrorTag::WriteFileError.as_ref(),
                            error_str = e.to_string(),
                            message = text,
                        );
                        return false;
                    }
                }
            }
        }

        {
            // write lib.hpp
            let reg = Handlebars::new();
            match reg.render_template(
                templates::LIB_HPP_HBS,
                &serde_json::json!({"project_upper": name.to_uppercase()}),
            ) {
                Err(e) => {
                    tracing::error!(
                        func = "Handlebars::render_template",
                        template = templates::LIB_HPP_HBS,
                        error_tag = ErrorTag::RenderHandlebarsError.as_ref(),
                        error_str = e.to_string()
                    );

                    return false;
                }
                Ok(text) => {
                    let path = format!(
                        "{}/{}/{}",
                        name,
                        relative_paths::SRC_DIR_NAME,
                        relative_paths::LIB_HPP_FILE_NAME
                    );
                    if let Err(e) = std::fs::write(&path, text.as_bytes()) {
                        tracing::error!(
                            func = "std::fs::write",
                            path = path,
                            error_tag = ErrorTag::WriteFileError.as_ref(),
                            error_str = e.to_string(),
                            message = text,
                        );
                        return false;
                    }
                }
            }
        }

        {
            // write lib.cpp
            let reg = Handlebars::new();
            match reg.render_template(
                templates::LIB_CPP_HBS,
                &serde_json::json!({"project_upper": name.to_uppercase()}),
            ) {
                Err(e) => {
                    tracing::error!(
                        func = "Handlebars::render_template",
                        template = templates::LIB_CPP_HBS,
                        error_tag = ErrorTag::RenderHandlebarsError.as_ref(),
                        error_str = e.to_string()
                    );

                    return false;
                }
                Ok(text) => {
                    let path = format!(
                        "{}/{}/{}",
                        name,
                        relative_paths::SRC_DIR_NAME,
                        relative_paths::LIB_CPP_FILE_NAME
                    );
                    if let Err(e) = std::fs::write(&path, text.as_bytes()) {
                        tracing::error!(
                            func = "std::fs::write",
                            path = path,
                            error_tag = ErrorTag::WriteFileError.as_ref(),
                            error_str = e.to_string(),
                            message = text,
                        );
                        return false;
                    }
                }
            }
        }

        return true;
    }

    fn new_package(&self, name: &str) -> bool {
        tracing::info!(message = "new package", name = name);

        // validate args
        if name.is_empty() {
            tracing::error!(
                func = "name.is_empty",
                error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
            );
            return false;
        }

        // skip is exists
        if util::fs::is_file_exists(name) {
            tracing::error!(
                func = "util::fs::is_file_exists",
                path = name,
                error_tag = ErrorTag::FileExistsError.as_ref()
            );
            return false;
        }

        // create src dir
        let src_dir = format!("{name}/{}", relative_paths::SRC_DIR_NAME);
        util::fs::create_dirs(&src_dir);

        let cwd = util::fs::get_cwd();

        // init
        util::fs::set_cwd(name);
        let args = init::InitArgs {
            lib: self.lib,
            shared: self.shared,
            workspace: self.workspace,
            member: self.member.clone(),
        };
        return args.init_package(name) && util::fs::set_cwd(&cwd);
    }

    fn new_workspace(&self) -> bool {
        // validate args
        let name = self.name.as_ref().unwrap();
        if name.is_empty() || self.member.is_empty() {
            tracing::error!(
                func = "self.member.is_empty",
                error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
            );
            return false;
        }

        tracing::info!(message = "new workspace", name = self.name);

        // skip is exists
        if util::fs::is_file_exists(name) {
            tracing::error!(
                func = "util::fs::is_file_exists",
                path = name,
                error_tag = ErrorTag::FileExistsError.as_ref()
            );
            return false;
        }

        let cwd = util::fs::get_cwd();

        if let Err(e) = std::fs::create_dir(name) {
            tracing::info!(
                func = "std::fs::create_dir",
                path = name,
                error_tag = e.to_string()
            );
            return false;
        }

        // create members
        util::fs::set_cwd(name);
        let mut has_error = false;
        let mut workspace = config::project::WorkSpaceConfig::default();
        for m in &self.member {
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
        let mut project = config::project::ProjectConfig::default();
        project.workspace = Some(workspace);

        // skip if exists
        if config::project::ProjectConfig::is_project_inited(true) {
            return false;
        }

        // write asc.toml
        let result = !has_error && project.validate() && project.write_project_conf();
        util::fs::set_cwd(&cwd);
        return result;
    }
}
