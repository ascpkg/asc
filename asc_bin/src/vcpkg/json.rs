use std::collections::BTreeMap;

use super::{index::VcpkgBaseline, search::get_port_version_commit_info, VcpkgManager};

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

static VCPKG_PORT_NAME_KEY: &str = "name";
static VCPKG_PORT_VERSION_KEY: &str = "version";
static VCPKG_REGISTRY_KIND_GIT: &str = "git";
static VCPKG_REGISTRY_DEFAULT_KIND: &str = "artifact";
static VCPKG_REGISTRY_DEFAULT_NAME: &str = "microsoft";
static VCPKG_REGISTRY_DEFAULT_LOCATION: &str =
    "https://github.com/microsoft/vcpkg-ce-catalog/archive/refs/heads/main.zip";

pub fn gen(dependencies: &BTreeMap<String, DependencyConfig>) {
    // ascending date time commits
    let mut sorted_commits = BTreeMap::new();

    let mut vcpkg_data = VcpkgDependency::load(relative_paths::VCPKG_JSON_FILE_NAME, true).unwrap();
    vcpkg_data.dependencies.clear();

    for (port_name, desc) in dependencies {
        vcpkg_data.dependencies.push(port_name.clone());
        vcpkg_data.overrides.push(BTreeMap::from([
            (String::from(VCPKG_PORT_NAME_KEY), port_name.clone()),
            (String::from(VCPKG_PORT_VERSION_KEY), desc.version.clone()),
        ]));

        if let Some(c) = get_port_version_commit_info(port_name, &desc.version) {
            sorted_commits.insert(c.date_time, c.hash);
        }
    }

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

        if let Some(baseline_data) = VcpkgBaseline::loads(&stdout, false) {
            // overwrite versions
            for desc in vcpkg_data.overrides.iter_mut() {
                if let Some(v) = baseline_data
                    .default
                    .get(desc.get(VCPKG_PORT_NAME_KEY).unwrap())
                {
                    desc.insert(
                        String::from(VCPKG_PORT_VERSION_KEY),
                        v.format_version_text(),
                    );
                }
            }

            // search baseline
            let mut found = true;
            for name in &vcpkg_data.dependencies {
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

    // write vcpkg.json
    vcpkg_data.dump(false);

    if baseline.is_empty() {
        tracing::error!("can't found all dependencies in same baseline");
    } else {
        let mut vcpkg_conf_data =
            VcpkgConfiguration::load(relative_paths::VCPKG_CONFIGURATION_JSON_FILE_NAME, true)
                .unwrap();
        vcpkg_conf_data.registries = vec![VcpkgRegistry {
            kind: String::from(VCPKG_REGISTRY_DEFAULT_KIND),
            name: String::from(VCPKG_REGISTRY_DEFAULT_NAME),
            location: String::from(VCPKG_REGISTRY_DEFAULT_LOCATION),
        }];
        vcpkg_conf_data.default_registry = VcpkgDefaultRegistry {
            kind: String::from(VCPKG_REGISTRY_KIND_GIT),
            repository: vcpkg_args.repo.unwrap(),
            baseline: VcpkgManager::get_latest_commit().hash,
        };
        // write vcpkg-configuration.json
        vcpkg_conf_data.dump(false);
    }
}
