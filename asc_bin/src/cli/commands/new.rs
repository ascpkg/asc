use clap::Args;

use handlebars::Handlebars;

use serde_json;

use uuid::Uuid;

use super::init;
use crate::cmake::project::default_vcpkg_triplet;
use crate::errors::ErrorTag;
use crate::{config, config::relative_paths, templates, util};

#[derive(Args, Debug, Clone, Default)]
/// new package/workspace of binary/static library/shared library
pub struct NewArgs {
    // Examples:
    //     asc new test_bin
    //     asc new --lib test_static_lib
    //     asc new --lib --shared test_shared_lib
    //     asc new --workspace test_workspace --lib --shared --member=a --member=b --member=c
    /// new package/workspace name
    pub name: Option<String>,

    /// new library (default bin)
    #[clap(long, default_value_t = false)]
    pub lib: bool,

    /// new shared library (default static library)
    #[clap(long, default_value_t = false)]
    pub shared: bool,

    /// new workspace (default package)
    #[clap(long, default_value_t = false)]
    pub workspace: bool,

    /// new workspace members (--member=a --member=b --member=c)
    #[clap(long)]
    pub member: Vec<String>,
}

impl NewArgs {
    pub fn exec(&self) -> bool {
        if self.name.is_some() {
            if self.workspace {
                return self.new_workspace();
            } else if !self.lib {
                let name = self.name.as_ref().unwrap();
                let result = self.new_bin(name);

                // new setup.iss
                let cwd = util::fs::get_cwd();
                util::fs::set_cwd(name);
                self.new_setup_iss(name);
                util::fs::set_cwd(&cwd);

                return result;
            } else {
                let name = self.name.as_ref().unwrap();
                let result = self.new_lib(name);

                // new setup.iss
                let cwd = util::fs::get_cwd();
                util::fs::set_cwd(name);
                self.new_setup_iss(name);
                util::fs::set_cwd(&cwd);

                return result;
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

        // new setup.iss
        self.new_setup_iss(name);

        // write asc.toml
        let result = !has_error && project.validate() && project.write_project_conf();
        util::fs::set_cwd(&cwd);
        return result;
    }

    fn new_setup_iss(&self, name: &str) {
        let reg = Handlebars::new();
        let text = reg
            .render_template(
                templates::SETUP_ISS_HUBS,
                &serde_json::json!({
                    "name": name,
                    "uuid": Uuid::new_v4().to_string(),
                    "version": config::project::ProjectConfig::version_date(),
                    "target": config::relative_paths::ASC_TARGET_DIR_NAME,
                    "installed": config::relative_paths::ASC_INSTALLED_DIR_NAME,
                    "triplet": default_vcpkg_triplet(),
                    "bin": config::relative_paths::CMAKE_INSTALL_BIN_DIR_NAME,
                }),
            )
            .unwrap();
        std::fs::write(
            config::relative_paths::INNO_SETUP_ISS_FILE_NAME,
            text.as_bytes(),
        )
        .unwrap();
    }
}
