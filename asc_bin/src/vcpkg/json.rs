use std::collections::BTreeMap;

use super::{index::VcpkgBaseline, search::get_port_version_commit_info};

use crate::{
    cli::commands::VcpkgArgs,
    config::{
        project::{
            DependencyConfig, VcpkgConfiguration, VcpkgDefaultRegistry, VcpkgDependency,
            VcpkgRegistry,
        },
        relative_paths, system_paths,
    },
    util,
};

pub fn gen(dependencies: &BTreeMap<String, DependencyConfig>) {
    // ascending date time commits
    let mut sorted_commits = BTreeMap::new();

    let mut vcpkg_data = VcpkgDependency::load(relative_paths::VCPKG_JSON_FILE_NAME, true).unwrap();
    vcpkg_data.port_names.clear();

    for (port_name, desc) in dependencies {
        vcpkg_data.port_names.push(port_name.clone());

        if let Some(c) = get_port_version_commit_info(port_name, &desc.version) {
            sorted_commits.insert(c.date_time, c.hash);
        }
    }

    // write vcpkg.json
    vcpkg_data.dump(false);

    let vcpkg_args = VcpkgArgs::load(&system_paths::ConfigPath::vcpkg_toml(), true).unwrap();
    let vcpkg_clone_dir = vcpkg_args
        .directory
        .unwrap_or(system_paths::DataPath::vcpkg_clone_dir());

    let cwd = util::fs::get_cwd();
    util::fs::set_cwd(&vcpkg_clone_dir);
    let mut baseline = String::new();
    for (date_time, hash) in sorted_commits {
        let output = util::shell::run(
            "git",
            &vec![
                "show",
                &format!(
                    "{}:{}",
                    hash,
                    relative_paths::vcpkg_versions_baseline_json()
                ),
            ],
            true,
            false,
            false,
        )
        .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        let mut found = true;
        if let Some(baseline_data) = VcpkgBaseline::loads(&stdout, false) {
            for name in &vcpkg_data.port_names {
                if !baseline_data.default.contains_key(name) {
                    found = false;
                    tracing::warn!("can't found {name} in {hash} @ {date_time}");
                    break;
                }
            }
            if found {
                tracing::info!("set baseline to {hash} @ {date_time}");
                baseline = hash;
                break;
            }
        }
    }
    util::fs::set_cwd(&cwd);

    if baseline.is_empty() {
        tracing::error!("can't found all dependencies in same baseline");
    } else {
        let mut vcpkg_conf_data =
            VcpkgConfiguration::load(relative_paths::VCPKG_CONFIGURATION_JSON_FILE_NAME, true)
                .unwrap();
        vcpkg_conf_data.registries = vec![VcpkgRegistry {
            kind: String::from("artifact"),
            name: String::from("microsoft"),
            location: String::from(
                "https://github.com/microsoft/vcpkg-ce-catalog/archive/refs/heads/main.zip",
            ),
        }];
        vcpkg_conf_data.default_registry = VcpkgDefaultRegistry {
            kind: String::from("git"),
            repository: vcpkg_args.repo.unwrap(),
            baseline: baseline,
        };
        // write vcpkg-configuration.json
        vcpkg_conf_data.dump(false);
    }
}
