use clap::Args;

use directories;

use config_file_derives::ConfigFile;

use serde::{Deserialize, Serialize};

use crate::{config, errors::ErrorTag, util};

use super::VcpkgAction;

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("toml")]
pub struct VcpkgArgs {
    #[serde(skip)]
    action: VcpkgAction,

    args: Vec<String>,

    #[clap(long)]
    repo: Option<String>,

    #[clap(long)]
    branch: Option<String>,

    #[clap(long)]
    directory: Option<String>,

    #[clap(long, default_value = "")]
    path: String,
}

impl VcpkgArgs {
    pub fn exec(&mut self) -> bool {
        tracing::info!(
            message = "vcpkg",
            repo = self.repo,
            branch = self.branch,
            directory = self.directory
        );

        match self.action {
            VcpkgAction::Update => self.update(),
            VcpkgAction::Set => self.set(),
            VcpkgAction::Get => {
                self.get();
                return true;
            }
            VcpkgAction::Index => self.index(),
        }
    }

    fn update(&mut self) -> bool {
        self.get();
        if !self.validate() {
            return false;
        }

        // clone if not exists
        if !util::fs::is_dir_exists(self.directory.as_ref().unwrap()) {
            let mut args = vec![
                "clone",
                "-b",
                self.branch.as_ref().unwrap(),
                self.repo.as_ref().unwrap(),
                self.directory.as_ref().unwrap(),
            ];
            for a in &self.args {
                args.push(&a);
            }

            return util::shell::run("git", &args, None, None).is_ok();
        } else {
            // fetch and reset
            let cwd = util::fs::get_cwd();
            util::fs::set_cwd(self.directory.as_ref().unwrap());

            let result = util::shell::run("git", &vec!["fetch"], None, None).is_ok()
                && util::shell::run(
                    "git",
                    &vec![
                        "reset",
                        "--hard",
                        &format!("origin/{}", self.branch.as_ref().unwrap()),
                    ],
                    None,
                    None,
                )
                .is_ok();

            util::fs::set_cwd(&cwd);
            return result;
        }
    }

    fn set(&self) -> bool {
        let config_path = self.config_path();
        if config_path.is_empty() {
            return false;
        }

        // write conf to file
        let mut conf = Self::load(&config_path, true).unwrap_or_default();
        if let Some(repo) = &self.repo {
            conf.repo = Some(repo.clone());
        }
        if let Some(branch) = &self.branch {
            conf.branch = Some(branch.clone());
        }
        if let Some(directory) = &self.directory {
            conf.directory = Some(directory.clone());
        }

        return conf.dump(false);
    }

    fn get(&mut self) {
        let config_path = self.config_path();
        if config_path.is_empty() {
            return;
        }

        match Self::load(&config_path, false) {
            None => {}
            Some(conf) => {
                // read conf from file
                if self.repo.is_none() {
                    if let Some(repo) = conf.repo {
                        self.repo = Some(repo);
                    }
                }
                if self.branch.is_none() {
                    if let Some(branch) = conf.branch {
                        self.branch = Some(branch);
                    }
                }
                if self.directory.is_none() {
                    if let Some(directory) = conf.directory {
                        self.directory = Some(directory);
                    }
                }
                // default directory
                if self.directory.is_none() {
                    self.directory = Some(format!("{}/vcpkg", self.data_directory()))
                }
            }
        }

        tracing::info!("{:#?}", self);
    }

    fn index(&self) -> bool {
        false
    }

    fn validate(&self) -> bool {
        if self.repo.is_none() && self.branch.is_none() && self.directory.is_none() {
            tracing::error!(
                call = "self.repo.is_none && self.branch.is_none && self.directory.is_none",
                error_tag = ErrorTag::InvalidCliArgsError.as_ref()
            );
            return false;
        }
        return true;
    }

    fn config_path(&self) -> String {
        if let Some(dir) = directories::ProjectDirs::from("", "", "asc") {
            let config_dir = dir.config_dir().to_str().unwrap().replace(r"\", "/");
            if !util::fs::is_dir_exists(&config_dir) {
                if !util::fs::create_dirs(&config_dir) {
                    return String::new();
                }
            }
            return format!("{config_dir}/vcpkg.toml");
        }
        return String::new();
    }

    fn data_directory(&self) -> String {
        if let Some(dir) = directories::ProjectDirs::from("", "", "asc") {
            let data_dir = dir.data_dir().to_str().unwrap().replace(r"\", "/");
            if !util::fs::is_dir_exists(&data_dir) {
                if !util::fs::create_dirs(&data_dir) {
                    return String::new();
                }
            }
            return data_dir;
        }
        return String::new();
    }
}
