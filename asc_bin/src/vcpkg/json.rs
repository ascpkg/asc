use std::collections::{BTreeMap, BTreeSet};

use super::search::get_port_version_commit_info;

use crate::{
    cli::commands::VcpkgArgs,
    config::{
        self,
        project::{DependencyConfig, PackageConfig},
        relative_paths::{self, ASC_TOML_FILE_NAME, VCPKG_JSON_FILE_NAME},
        vcpkg::{
            port::{VcpkgDependency, VcpkgDependencyDesc, VcpkgJsonDependency, VcpkgPortJson},
            registry_manifest::{VcpkgConfiguration, VcpkgDefaultRegistry, VcpkgRegistry},
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

pub fn gen_vcpkg_configurations(dependencies: &BTreeMap<String, DependencyConfig>) {
    let mut grouped_names = BTreeMap::new();
    // ascending date time commits group by registry
    let mut grouped_commits = BTreeMap::new();

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

        if let Some((registry, commit)) = get_port_version_commit_info(port_name, &desc.version) {
            grouped_names
                .entry(registry.clone())
                .or_insert_with(BTreeSet::new)
                .insert(port_name.clone());
            grouped_commits
                .entry(registry)
                .or_insert_with(BTreeSet::new)
                .insert((
                    commit.date_time,
                    commit.hash,
                    port_name,
                    desc.version.clone(),
                ));
        }
    }

    tracing::info!("baselines: {:#?}", grouped_commits);

    let vcpkg_versions_baseline_json = relative_paths::vcpkg_versions_baseline_json();
    let vcpkg_args = VcpkgArgs::load_or_default();
    let cwd = util::fs::get_cwd();
    let mut registry_baseline = BTreeMap::new();
    for (registry, commits) in &grouped_commits {
        for (date_time, hash, _, _) in commits {
            let (_registry, url, branch, vcpkg_root_dir) = vcpkg_args.get_registry(&registry);
            util::fs::set_cwd(&vcpkg_root_dir);

            let stdout = git::show::commit_file_content(
                &vcpkg_root_dir,
                &hash,
                &vcpkg_versions_baseline_json,
            );

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
                let mut versions = vec![];
                for name in grouped_names.get(registry).unwrap_or(&BTreeSet::new()) {
                    if let Some(version) = baseline_data.default.get(name) {
                        versions.push(format!("{name}={}", version.format_version_text()));
                    } else {
                        found = false;
                        tracing::warn!("can't found {} in {hash} @ {date_time}", name);
                        break;
                    }
                }
                if found {
                    tracing::info!(
                        "set baseline to {hash} @ {date_time} ({})",
                        versions.join(", ")
                    );
                    registry_baseline.insert(registry, (url, branch, hash));
                }
            }
        }
    }
    util::fs::set_cwd(&cwd);

    if vcpkg_data.dependencies.is_empty() {
        tracing::error!("can't found any dependencies in {}", ASC_TOML_FILE_NAME);
    } else {
        // write vcpkg.json
        vcpkg_data.dump(true, false);

        // auto bootstrap
        super::bootstrap::run();
    }

    if registry_baseline.is_empty() {
        if !vcpkg_data.dependencies.is_empty() {
            tracing::error!("can't found all dependencies in same baseline");
        }
    } else {
        let mut vcpkg_conf_data =
            VcpkgConfiguration::load(relative_paths::VCPKG_CONFIGURATION_JSON_FILE_NAME, true)
                .unwrap();
        vcpkg_conf_data.registries.clear();
        for (registry, (url, branch, hash)) in registry_baseline {
            if registry == config::relative_paths::VCPKG_DIR_NAME {
                vcpkg_conf_data.default_registry = VcpkgDefaultRegistry {
                    kind: String::from(VCPKG_REGISTRY_KIND_GIT),
                    repository: url,
                    baseline: hash.clone(),
                };
            } else {
                vcpkg_conf_data.registries.push(VcpkgRegistry {
                    kind: String::from(VCPKG_REGISTRY_KIND_GIT),
                    reference: branch,
                    repository: url,
                    baseline: hash.clone(),
                    packages: grouped_names
                        .get(registry)
                        .unwrap()
                        .iter()
                        .map(|s| s.clone())
                        .collect::<Vec<String>>(),
                    ..Default::default()
                });
            }
        }
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
    name: &str,
    version: &Option<String>,
    version_date: &Option<String>,
    version_semver: &Option<String>,
    version_string: &Option<String>,
    port_version: u32,
) -> bool {
    let mut versions_data = VcpkgPortVersions::load(
        &config::system_paths::DataPath::vcpkg_versions_port_json_path(repo_root_dir, name),
        true,
    )
    .unwrap();

    let mut baseline_version = String::new();
    for vv in [version, version_date, version_semver, version_string] {
        if let Some(v) = vv {
            baseline_version = v.clone();
            if !versions_data.versions.is_empty() {
                if versions_data.versions[0].version.as_ref().unwrap() == v
                    && versions_data.versions[0].port_version == port_version
                {
                    tracing::warn!(
                        message = "the port version was not changed",
                        name = name,
                        version = v,
                        port_version = port_version,
                    );
                    versions_data.versions.clear();
                }
            }
        }
    }

    versions_data.versions.insert(
        0,
        VcpkgPortTreeVersion {
            git_tree: git::rev_parse::run("HEAD", name, repo_root_dir),
            version: version.clone(),
            version_date: version_date.clone(),
            version_semver: version_semver.clone(),
            version_string: version_string.clone(),
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
    if let Some(v) = baseline_data.default.get(name) {
        if v.baseline == baseline_version && v.port_version == port_version {
            tracing::warn!(
                message = "the baseline version was not changed",
                name = name,
                version = baseline_version,
                port_version = port_version,
            );
            return result;
        }
    }

    baseline_data.default.insert(
        name.to_string(),
        VcpkgPortVersion {
            baseline: baseline_version,
            port_version: port_version,
        },
    );

    result &= baseline_data.dump(true, false);

    return result;
}
