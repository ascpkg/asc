use std::collections::BTreeSet;

use crate::{cli::commands::add::AddArgs, config, errors::ErrorTag, util, vcpkg};

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
        let mut version = args.version.clone();
        if version.is_empty() {
            let results = vcpkg::search::from_index_file(&args.dependency, true);
            if results.is_empty() {
                tracing::error!(
                    call = "vcpkg::search::from_index_file",
                    port = args.dependency,
                    error_tag = ErrorTag::VcpkgPortNotFound.as_ref(),
                    message = "try to run asc vcpkg update, asc vcpkg index"
                );
                return false;
            }
            let v = results[0].split_once(']').unwrap().1.trim();
            version = v.split_once("  ").unwrap().0.to_string();
        } else {
            let mut found = false;
            let results = vcpkg::search::from_index_file(&args.dependency, true);
            for v in &results {
                let v = v.split_once(']').unwrap().1.trim();
                if v.starts_with(&format!("{}  ", &version)) {
                    found = true;
                }
            }
            if !found {
                tracing::error!(
                    call = "vcpkg::search::from_index_file",
                    port = args.dependency,
                    error_tag = ErrorTag::VcpkgPortVersionNotFound.as_ref(),
                    message = format!(
                        "try to run asc vcpkg update, asc vcpkg index\n{}",
                        results.join("\n")
                    )
                );
                return false;
            }
        }

        project_conf.dependencies.insert(
            args.dependency.clone(),
            config::project::DependencyConfig {
                version: version,
                find_packages: args
                    .find_package
                    .iter()
                    .map(|s| s.clone())
                    .collect::<BTreeSet<String>>(),
                include_directories: args
                    .include_directory
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
