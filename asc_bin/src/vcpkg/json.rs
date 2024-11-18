use std::collections::BTreeMap;

use super::{search::get_port_version_commit_info, VcpkgManager};

use crate::{
    cli::commands::VcpkgArgs,
    config::{
        self,
        project::{DependencyConfig, PackageConfig},
        relative_paths::{self, ASC_TOML_FILE_NAME, VCPKG_JSON_FILE_NAME},
        vcpkg::{
            manifest::{VcpkgConfiguration, VcpkgDefaultRegistry, VcpkgRegistry},
            port::{VcpkgDependency, VcpkgDependencyDesc, VcpkgJsonDependency, VcpkgPortJson},
            versions_baseline::{VcpkgBaseline, VcpkgPortVersion},
            versions_port::{VcpkgPortTreeVersion, VcpkgPortVersions},
        },
    },
    git::{self, log::GitCommitInfo},
    util,
};

static VCPKG_PORT_NAME_KEY: &str = "name";
static VCPKG_PORT_VERSION_KEY: &str = "version";
static VCPKG_PORT_PLATFORM_KEY: &str = "platform";
static VCPKG_FEATURE_PLATFORM_DELIMITER: &str = "@";
static VCPKG_REGISTRY_KIND_GIT: &str = "git";
static VCPKG_REGISTRY_DEFAULT_KIND: &str = "artifact";
static VCPKG_REGISTRY_DEFAULT_NAME: &str = "microsoft";
static VCPKG_REGISTRY_DEFAULT_LOCATION: &str =
    "https://github.com/microsoft/vcpkg-ce-catalog/archive/refs/heads/main.zip";

pub fn gen_vcpkg_configurations(dependencies: &BTreeMap<String, DependencyConfig>) {
    // ascending date time commits
    let mut sorted_commits = BTreeMap::new();

    let mut vcpkg_data = VcpkgDependency::load(relative_paths::VCPKG_JSON_FILE_NAME, true).unwrap();
    vcpkg_data.dependencies.clear();
    vcpkg_data.overrides.clear();

    for (port_name, desc) in dependencies {
        let mut dep = VcpkgDependencyDesc::default();
        dep.name = port_name.clone();
        if !desc.features.is_empty() {
            dep.default_features = Some(false);
        }
        for f in &desc.features {
            match f.split_once(VCPKG_FEATURE_PLATFORM_DELIMITER) {
                None => {
                    dep.features.push(BTreeMap::from([(
                        String::from(VCPKG_PORT_NAME_KEY),
                        f.clone(),
                    )]));
                }
                Some((n, p)) => {
                    dep.features.push(BTreeMap::from([
                        (String::from(VCPKG_PORT_NAME_KEY), n.to_string()),
                        (String::from(VCPKG_PORT_PLATFORM_KEY), p.to_string()),
                    ]));
                }
            };
        }
        vcpkg_data.dependencies.push(dep);
        vcpkg_data.overrides.push(BTreeMap::from([
            (String::from(VCPKG_PORT_NAME_KEY), port_name.clone()),
            (String::from(VCPKG_PORT_VERSION_KEY), desc.version.clone()),
        ]));

        if let Some(c) = get_port_version_commit_info(port_name, &desc.version) {
            sorted_commits.insert(c.date_time, c.hash);
        }
    }

    let vcpkg_args = VcpkgArgs::load_or_default();
    let vcpkg_clone_dir = vcpkg_args.directory.unwrap();

    let cwd = util::fs::get_cwd();
    util::fs::set_cwd(&vcpkg_clone_dir);
    let mut baseline = String::new();
    for (date_time, hash) in sorted_commits {
        let stdout = git::show::run(&vcpkg_clone_dir, &hash);

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
            for desc in &vcpkg_data.dependencies {
                if !baseline_data.default.contains_key(&desc.name) {
                    found = false;
                    tracing::warn!("can't found {} in {hash} @ {date_time}", desc.name);
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

    if vcpkg_data.dependencies.is_empty() {
        tracing::error!("can't found any dependencies in {}", ASC_TOML_FILE_NAME);
    } else {
        // write vcpkg.json
        vcpkg_data.dump(true, false);
    }

    if baseline.is_empty() {
        if !vcpkg_data.dependencies.is_empty() {
            tracing::error!("can't found all dependencies in same baseline");
        }
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
        vcpkg_conf_data.dump(true, false);
    }
}

pub fn gen_port_json(
    repo_root_dir: &String,
    package_conf: &PackageConfig,
    dependencies: &BTreeMap<String, DependencyConfig>,
    latest_commit: &GitCommitInfo,
) -> (bool, u32) {
    let mut data = VcpkgPortJson::load(
        &config::system_paths::DataPath::vcpkg_ports_vcpkg_json_path(
            repo_root_dir,
            &package_conf.name,
        ),
        true,
    )
    .unwrap();

    let port_file_cmake = std::fs::read_to_string(
        &config::system_paths::DataPath::vcpkg_ports_port_file_cmake_path(
            repo_root_dir,
            &package_conf.name,
        ),
    )
    .unwrap_or_default();

    data.name = package_conf.name.clone();
    if data.version >= package_conf.version {
        tracing::error!(
            message = format!("version in {VCPKG_JSON_FILE_NAME} was large than package version in {ASC_TOML_FILE_NAME}"),
            version_in_vcpkg_json = data.version,
            version_in_asc_toml = data.version,
        );
        return (false, data.port_version);
    } else if data.version == package_conf.version {
        if port_file_cmake.contains(&latest_commit.hash) {
            tracing::warn!(
                message = "the version and commit hash were not changed",
                version = data.version,
                commit_hash = latest_commit.hash,
                commit_time = latest_commit.date_time
            );

            println!("Do you want to update port version, yes or no? ");
            let mut choose = String::new();
            std::io::stdin().read_line(&mut choose).unwrap();
            if [String::from("y"), String::from("yes")].contains(&choose.to_lowercase()) {
                data.port_version += 1;
                update_vcpkg_json_fields(&mut data, package_conf, dependencies);
                data.dump(true, false);
                return (true, data.port_version);
            }

            return (false, data.port_version);
        } else {
            tracing::warn!(
                message = "the version was not changed, but commit hash was changed",
                version = data.version,
                commit_hash = latest_commit.hash,
                commit_time = latest_commit.date_time
            );

            println!("Do you want to update port version, yes or no? ");
            let mut choose = String::new();
            std::io::stdin().read_line(&mut choose).unwrap();
            if [String::from("y"), String::from("yes")].contains(&choose.to_lowercase()) {
                data.port_version += 1;
                update_vcpkg_json_fields(&mut data, package_conf, dependencies);
                data.dump(true, false);
                return (true, data.port_version);
            }

            tracing::error!(
                message = format!("update package version in {ASC_TOML_FILE_NAME} first"),
                version = data.version,
                commit_hash = latest_commit.hash,
                commit_time = latest_commit.date_time
            );
            return (false, data.port_version);
        }
    } else {
        data.version = package_conf.version.clone();
        data.port_version = 0;
        update_vcpkg_json_fields(&mut data, package_conf, dependencies);
        data.dump(true, false);
        return (true, data.port_version);
    }
}

fn update_vcpkg_json_fields(
    data: &mut VcpkgPortJson,
    package_conf: &PackageConfig,
    dependencies: &BTreeMap<String, DependencyConfig>,
) {
    data.description = package_conf.description.clone();
    data.homepage = package_conf.repository.clone();
    data.license = package_conf.license.clone();
    data.supports = package_conf.supports.clone();

    data.dependencies.clear();

    for (name, desc) in dependencies {
        let mut dep = VcpkgJsonDependency::default();
        dep.name = name.clone();
        let mut platforms = vec![];
        if !desc.features.is_empty() {
            dep.default_features = Some(false);
            for feat in &desc.features {
                match feat.split_once(VCPKG_FEATURE_PLATFORM_DELIMITER) {
                    None => {
                        dep.features.push(feat.clone());
                    }
                    Some((n, p)) => {
                        let ns = n.to_string();
                        if !dep.features.contains(&ns) {
                            dep.features.push(ns);
                        }
                        let ps = p.to_string();
                        if !platforms.contains(&ps) {
                            platforms.push(ps);
                        }
                    }
                };
            }
        }
        if !platforms.is_empty() {
            dep.platform = Some(
                platforms
                    .iter()
                    .map(|p| {
                        if p.contains("|") || p.contains("&") {
                            format!("({p})")
                        } else {
                            p.clone()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" | "),
            );
        }
        data.dependencies.push(dep);

        data.dependencies.push(VcpkgJsonDependency {
            name: String::from("vcpkg-cmake"),
            host: Some(true),
            ..Default::default()
        });
        data.dependencies.push(VcpkgJsonDependency {
            name: String::from("vcpkg-cmake-config"),
            host: Some(true),
            ..Default::default()
        });
    }
}

pub fn gen_port_versions(
    repo_root_dir: &String,
    package_conf: &PackageConfig,
    port_version: u32,
) -> bool {
    let mut versions_data = VcpkgPortVersions::load(
        &config::system_paths::DataPath::vcpkg_versions_port_json_path(
            repo_root_dir,
            &package_conf.name,
        ),
        true,
    )
    .unwrap();

    if !versions_data.versions.is_empty() {
        if versions_data.versions[0].version.as_ref().unwrap() == &package_conf.version
            && versions_data.versions[0].port_version == port_version
        {
            tracing::warn!(
                message = "the version was not changed",
                version = package_conf.version,
                port_version = port_version,
            );
            return false;
        }
    }

    versions_data.versions.insert(
        0,
        VcpkgPortTreeVersion {
            git_tree: git::rev_parse::run(&package_conf.name, repo_root_dir),
            version: Some(package_conf.version.clone()),
            port_version: port_version,
            ..Default::default()
        },
    );

    let mut result = versions_data.dump(true, false);

    let mut baseline_data = VcpkgBaseline::load(
        &&config::system_paths::DataPath::vcpkg_versions_baseline_json_path(repo_root_dir),
        true,
    )
    .unwrap();
    if let Some(v) = baseline_data.default.get(&package_conf.name) {
        if v.baseline == package_conf.version && v.port_version == port_version {
            tracing::warn!(
                message = "the version was not changed",
                version = package_conf.version,
                port_version = port_version,
            );
            return result;
        }
    }

    baseline_data.default.insert(
        package_conf.name.clone(),
        VcpkgPortVersion {
            baseline: package_conf.version.clone(),
            port_version: port_version,
        },
    );

    result &= baseline_data.dump(true, false);

    return result;
}
