use crate::{cli::commands::RemoveArgs, config, errors::ErrorTag, util};

pub fn dependency_from_config_file(args: &RemoveArgs) -> bool {
    tracing::info!(message = "remove", dependency = args.dependency);

    match config::project::ProjectConfig::read_project_conf() {
        None => false,
        Some(mut project_conf) => match project_conf.workspace {
            None => remove_for_pakcage(args, &mut project_conf),
            Some(workspace) => remove_for_workspace(args, workspace),
        },
    }
}

fn remove_for_workspace(args: &RemoveArgs, workspace: config::project::WorkSpaceConfig) -> bool {
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
                    Some(mut project_conf) => remove_for_pakcage(args, &mut project_conf),
                };
                util::fs::set_cwd(&cwd);
                return result;
            }
        }
    }
}

fn remove_for_pakcage(
    args: &RemoveArgs,
    project_conf: &mut config::project::ProjectConfig,
) -> bool {
    if args.dependency.is_empty() {
        tracing::error!(
            call = "args.dependency.is_empty",
            error_tag = ErrorTag::InvalidCliArgsError.as_ref(),
        );
        return false;
    } else {
        return project_conf.dependencies.remove(&args.dependency).is_some()
            && project_conf.dump(false);
    }
}
