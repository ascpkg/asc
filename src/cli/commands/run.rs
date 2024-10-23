use crate::{config, errors::ErrorTag};

use super::ConfigType;

use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct RunArgs {
    name: Option<String>,

    args: Option<Vec<String>>,

    #[clap(long, default_value = "debug")]
    config: ConfigType,
}

impl RunArgs {
    pub fn exec(&self) -> bool {
        tracing::info!(message = "run");

        if let Some(project_conf) = config::ProjectConfig::read_project_conf() {
            if let Some(workspace) = project_conf.workspace {
                if let Some(name) = &self.name {
                    return self.run(
                        format!(
                            "{}/{}/{}/{}",
                            config::path::PROJECT_TARGET_DIR,
                            name,
                            self.config.as_ref(),
                            name
                        ),
                        self.args.as_ref().unwrap_or(&Vec::<String>::new()),
                    );
                } else {
                    tracing::error!(
                        error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
                        members = workspace
                            .members
                            .into_iter()
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                }
            }
            if let Some(package) = project_conf.package {
                return self.run(
                    format!(
                        "{}/{}/{}",
                        config::path::PROJECT_TARGET_DIR,
                        self.config.as_ref(),
                        package.name
                    ),
                    self.args.as_ref().unwrap_or(&Vec::<String>::new()),
                );
            }
        }

        return false;
    }

    fn run(&self, command: String, args: &Vec<String>) -> bool {
        tracing::info!(run = command, args = args.join(" "));
        return std::process::Command::new(command)
            .args(args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .output()
            .is_ok();
    }
}
