use std::collections::BTreeSet;

use crate::{cli::commands::add::AddArgs, config, errors::ErrorTag, util};

pub fn dependency_to_config_file(args: &AddArgs) -> bool {
    match config::project::ProjectConfig::read_project_conf() {
        None => false,
        Some(mut project_conf) => match project_conf.workspace {
            None => add_for_pakcage(args, &mut project_conf),
            Some(workspace) => add_for_workspace(args, workspace),
        },
    }
}

fn add_for_workspace(args: &AddArgs, workspace: config::project::WorkSpaceConfig) -> bool {
    match &args.package {
        None => {
            tracing::error!(
                call = "args.package.is_none",
                error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
            );
            return false;
        }
        Some(member) => {
            if !workspace.members.contains(member) {
                tracing::error!(
                    error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
                    packages = workspace.get_members()
                );
                return false;
            } else {
                let cwd = util::fs::get_cwd();
                util::fs::set_cwd(member);
                let result = match config::project::ProjectConfig::read_project_conf() {
                    None => false,
                    Some(mut project_conf) => add_for_pakcage(args, &mut project_conf),
                };
                util::fs::set_cwd(&cwd);
                return result;
            }
        }
    }
}

fn add_for_pakcage(args: &AddArgs, project_conf: &mut config::project::ProjectConfig) -> bool {
    if args.dependency.is_empty() {
        tracing::error!(
            call = "args.dependency.is_empty",
            error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
        );
        return false;
    } else {
        project_conf.dependencies.insert(
            args.dependency.clone(),
            config::project::DependencyConfig {
                version: args.version.clone(),
                find_packages: args
                    .find_package
                    .iter()
                    .map(|s| s.clone())
                    .collect::<BTreeSet<String>>(),
                link_libraries: args
                    .link_library
                    .iter()
                    .map(|s| s.clone())
                    .collect::<BTreeSet<String>>(),
                features: args
                    .feature
                    .iter()
                    .map(|s| s.clone())
                    .collect::<BTreeSet<String>>(),
            },
        );
        return project_conf.write_project_conf();
    }
}
