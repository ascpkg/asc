use std::collections::{BTreeMap, BTreeSet};

use fs_extra;

use crate::{
    config::{
        relative_paths::{
            ASC_REGISTRY_DIR_NAME, VCPKG_CONTROL_FILE_NAME, VCPKG_DIR_NAME, VCPKG_JSON_FILE_NAME,
            VCPKG_PORTS_DIR_NAME, VCPKG_VERSIONS_DIR_NAME,
        },
        vcpkg::port_manifest::VcpkgPortManifest,
    },
    git,
    util::{self, shell},
    vcpkg,
};

use super::VcpkgManager;

const FLATTEN_PREFIX: &str = "flatten https://github.com/microsoft/vcpkg/commit/";

impl VcpkgManager {
    pub fn flatten(&mut self) -> bool {
        // update registries
        if self.args.sync {
            self.update();
        }

        // get registry dirs
        let (vcpkg_registry_dir, asc_registry_dir) = self.get_registry_dirs();

        // load asc registry check point
        let check_point_hash = self.load_check_point(&asc_registry_dir);

        // prepare dirs
        let ports = VCPKG_PORTS_DIR_NAME.replace("/", "");
        let asc_registry_ports_dir = format!("{asc_registry_dir}/{ports}");
        let tar_name = format!("{ports}.tar",);
        let tmp_dir = format!("{asc_registry_dir}/tmp");
        let tmp_tar_path = format!("{tmp_dir}/{tar_name}");
        let tmp_ports_path = format!("{tmp_dir}/{ports}");
        util::fs::remove_dirs(&tmp_dir);
        util::fs::create_dirs(&tmp_dir);

        // get vcpkg registry commits
        let vcpkg_ports_changed_commits =
            git::log::get_changed_commits(&vcpkg_registry_dir, VCPKG_PORTS_DIR_NAME);
        let mut next_index = 0;
        if let Some(index) = vcpkg_ports_changed_commits
            .iter()
            .position(|c| c.0.hash.starts_with(&check_point_hash))
        {
            if !check_point_hash.is_empty() {
                next_index = index; // redo last commit because versions dir may not be added and committed
            }
        }
        let total = vcpkg_ports_changed_commits.len() as f32;
        for (index, (commit, changed_files)) in
            vcpkg_ports_changed_commits[next_index..].iter().enumerate()
        {
            tracing::warn!(
                "========== {} / {total} = {}% ==========",
                next_index + index,
                (index + next_index) as f32 * 100.0 / total
            );

            // get all ports
            let (all_port_versions, all_ports_manifest) =
                self.get_all_port_versions(&vcpkg_registry_dir, &commit.hash);

            // get port git trees
            let changed_ports = self.get_changed_ports(changed_files);

            // output git archive
            self.output_git_archive(
                &vcpkg_registry_dir,
                &commit.hash,
                &tmp_tar_path,
                &tmp_ports_path,
            );

            // append version to port name in CONTROL/vcpkg.json
            for port_name in &changed_ports {
                self.append_version_to_port_manifest(
                    format!("{tmp_ports_path}/{port_name}"),
                    &all_port_versions,
                );
            }

            // move versioned ports
            self.move_versioned_ports(
                &asc_registry_ports_dir,
                &tmp_ports_path,
                &changed_ports,
                &all_port_versions,
            );

            // git add ports
            git::add::run(&vec![VCPKG_PORTS_DIR_NAME.to_string()], &asc_registry_dir);
            git::commit::run(
                format!(
                    "{FLATTEN_PREFIX}{} from {} at {}",
                    commit.hash.split_at(7).0,
                    commit.user_email,
                    commit.date_time
                ),
                &asc_registry_dir,
            );

            // generate manifests
            self.generate_port_manifests(
                &asc_registry_dir,
                &changed_ports,
                &all_port_versions,
                &all_ports_manifest,
            );

            // git add versions
            git::add::run(
                &vec![VCPKG_VERSIONS_DIR_NAME.to_string()],
                &asc_registry_dir,
            );
            git::commit_amend::run(&asc_registry_dir);

            // git push
            if self.args.push && (index > 0 && index % 10 == 0) {
                git::push::run(&asc_registry_dir, true);
            }
        }

        // remove tmp dir
        util::fs::remove_dirs(&tmp_dir);

        return true;
    }

    pub fn get_registry_dirs(&mut self) -> (String, String) {
        let mut vcpkg_registry_dir = String::new();
        let mut asc_registry_dir = String::new();
        for (name, path) in Self::get_vcpkg_root_dir() {
            if name == VCPKG_DIR_NAME {
                vcpkg_registry_dir = path;
            } else if name == ASC_REGISTRY_DIR_NAME {
                asc_registry_dir = path;
            }
        }
        return (vcpkg_registry_dir, asc_registry_dir);
    }

    pub fn load_check_point(&self, asc_registry_dir: &str) -> String {
        let stat_text = git::log::get_latest_commit_stat(&asc_registry_dir);
        for line in stat_text.lines() {
            if line.contains(FLATTEN_PREFIX) {
                return line
                    .split_once(FLATTEN_PREFIX)
                    .unwrap()
                    .1
                    .split_whitespace()
                    .collect::<Vec<&str>>()[0]
                    .to_string();
            }
        }
        return String::new();
    }

    pub fn get_all_port_versions(
        &self,
        vcpkg_registry_dir: &str,
        commit_hash: &str,
    ) -> (BTreeMap<String, String>, BTreeMap<String, (String, String)>) {
        let mut all_port_versions = BTreeMap::new();
        let all_ports_manifest = git::ls_tree::list_ports(commit_hash, vcpkg_registry_dir, true);
        for (port, (control_file_text, vcpkg_json_file_text)) in &all_ports_manifest {
            if !control_file_text.is_empty() {
                all_port_versions.insert(
                    port.clone(),
                    VcpkgPortManifest::get_version_from_control_file(control_file_text),
                );
            } else if !vcpkg_json_file_text.is_empty() {
                all_port_versions.insert(
                    port.clone(),
                    VcpkgPortManifest::get_version_from_vcpkg_json_file(vcpkg_json_file_text),
                );
            }
        }
        return (all_port_versions, all_ports_manifest);
    }

    pub fn get_changed_ports(&self, changed_files: &Vec<String>) -> BTreeSet<String> {
        let mut changed_ports = BTreeSet::new();
        for file in changed_files {
            changed_ports.insert(
                file.split_at(VCPKG_PORTS_DIR_NAME.len())
                    .1
                    .split_once("/")
                    .unwrap()
                    .0
                    .to_string(),
            );
        }
        return changed_ports;
    }

    pub fn output_git_archive(
        &self,
        vcpkg_registry_dir: &str,
        commit_hash: &str,
        tmp_tar_path: &str,
        tmp_ports_path: &str,
    ) {
        // export archive
        git::archive::run(
            vcpkg_registry_dir,
            "tar",
            tmp_tar_path,
            commit_hash,
            VCPKG_PORTS_DIR_NAME,
        );
        // extract archive
        util::fs::create_dirs(tmp_ports_path);
        shell::run(
            "tar",
            &vec!["-xf", tmp_tar_path],
            tmp_ports_path,
            false,
            false,
            true,
        )
        .unwrap();
        // remove archive
        util::fs::remove_file(tmp_tar_path);
    }

    pub fn append_version_to_port_manifest(
        &mut self,
        port_manifest_dir: String,
        all_port_versions: &BTreeMap<String, String>,
    ) {
        let control_file = format!("{port_manifest_dir}/{VCPKG_CONTROL_FILE_NAME}");
        let vcpkg_json_file = format!("{port_manifest_dir}/{VCPKG_JSON_FILE_NAME}");
        if util::fs::is_file_exists(&control_file) {
            let version = VcpkgPortManifest::update_control_file(&control_file, all_port_versions);
            std::fs::rename(&port_manifest_dir, format!("{port_manifest_dir}-{version}")).unwrap();
        } else if util::fs::is_file_exists(&vcpkg_json_file) {
            let version =
                VcpkgPortManifest::update_vcpkg_json_file(&vcpkg_json_file, all_port_versions);
            std::fs::rename(&port_manifest_dir, format!("{port_manifest_dir}-{version}")).unwrap();
        };
    }

    pub fn move_versioned_ports(
        &self,
        asc_registry_ports_dir: &str,
        tmp_ports_path: &str,
        changed_ports: &BTreeSet<String>,
        all_port_versions: &BTreeMap<String, String>,
    ) {
        let mut versioned_ports = Vec::new();
        for port_name in changed_ports {
            if let Some(version) = all_port_versions.get(port_name) {
                let path = format!("{tmp_ports_path}/{port_name}-{version}");
                if util::fs::is_dir_exists(&path) {
                    versioned_ports.push(path);
                } else {
                    tracing::warn!("{port_name} {version} was not found");
                }
            } else {
                tracing::warn!("{port_name} version was not found");
            }
        }
        if !versioned_ports.is_empty() {
            let mut options = fs_extra::dir::CopyOptions::new();
            options.overwrite = true;
            fs_extra::move_items(&versioned_ports, &asc_registry_ports_dir, &options).unwrap();
        }

        // remove tmp ports dir
        util::fs::remove_dirs(&tmp_ports_path);
    }

    pub fn generate_port_manifests(
        &self,
        asc_registry_dir: &String,
        changed_ports: &BTreeSet<String>,
        all_port_versions: &BTreeMap<String, String>,
        all_ports_manifest: &BTreeMap<String, (String, String)>,
    ) {
        for port_name in changed_ports {
            if let Some(version) = all_port_versions.get(port_name) {
                let new_name = format!("{port_name}-{version}");
                if let Some((control_file_text, vcpkg_json_file_text)) =
                    all_ports_manifest.get(port_name)
                {
                    if !control_file_text.is_empty() {
                        let (version, version_date, version_semver, version_string, port_version) =
                            VcpkgPortManifest::get_versions_from_control_file(control_file_text);
                        vcpkg::json::gen_port_versions(
                            asc_registry_dir,
                            &new_name,
                            &version,
                            &version_date,
                            &version_semver,
                            &version_string,
                            port_version,
                        );
                    } else if !vcpkg_json_file_text.is_empty() {
                        let (version, version_date, version_semver, version_string, port_version) =
                            VcpkgPortManifest::get_versions_from_vcpkg_json_file(
                                vcpkg_json_file_text,
                            );
                        vcpkg::json::gen_port_versions(
                            asc_registry_dir,
                            &new_name,
                            &version,
                            &version_date,
                            &version_semver,
                            &version_string,
                            port_version,
                        );
                    }
                }
            }
        }
    }
}
