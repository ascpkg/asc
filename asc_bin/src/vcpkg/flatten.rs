use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;
use config_file_types;

use crate::{
    config::{
        relative_paths::{
            ASC_REGISTRY_CHECK_POINT_FILE_NAME, ASC_REGISTRY_DIR_NAME, VCPKG_DIR_NAME,
            VCPKG_PORTS_DIR_NAME, VCPKG_VERSIONS_DIR_NAME,
        },
        vcpkg::port_manifest::VcpkgPortManifest,
    },
    git::{self, log::GitCommitInfo},
    util::{self, shell},
};

use super::VcpkgManager;

/*
 * delete CONTROL and add vcpkg.json
 *
 * commit 1d8f0acc9c3085d18152a3f639077a28109196b6
 * Author: nicole mazzuca <mazzucan@outlook.com>
 * Date:   Tue Jun 30 10:40:18 2020 -0700
 *
 *     [vcpkg manifest] Manifest Implementation (#11757)
 *
 */
pub static GIT_COMMIT_HASH_ADD_VCPKG_JSON: &str = "1d8f0acc9c3085d18152a3f639077a28109196b6";

/*
 * add port_versions
 *
 * commit 2f6537fa2b8928d2329e827f862692112793435d
 * Author: Victor Romero <romerosanchezv@gmail.com>
 * Date:   Thu Jan 14 16:08:36 2021 -0800
 *
 *    [vcpkg] Add version files (#15652)
 *
 *    * Add version files
 *
 *    * Remove unnecessary file
 *
 */
pub static GIT_COMMIT_HASH_ADD_PORT_VERSIONS: &str = "2f6537fa2b8928d2329e827f862692112793435d";

/*
* rename port_versions to versions
*
* commit 68a74950d0400f5a803026d0860f49853984bf11
* Author: nicole mazzuca <mazzucan@outlook.com>
* Date:   Thu Jan 21 09:53:22 2021 -0800
*
*     [vcpkg] Rename `port_versions` to `versions` (#15784)
*
*/
pub static GIT_COMMIT_HASH_RENAME_TO_VERSIONS: &str = "68a74950d0400f5a803026d0860f49853984bf11";

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
struct AscRegistryCheckPoint {
    #[serde(skip)]
    path: String,

    modified: String,
    check_point: GitCommitInfo,
}

impl VcpkgManager {
    pub fn flatten(&mut self) -> bool {
        // update registries
        self.update();

        // get registry dirs
        let (vcpkg_registry_dir, asc_registry_dir) = self.get_registry_dirs();

        // load asc registry check point
        let mut sync_check_point = AscRegistryCheckPoint::load(
            &format!("{asc_registry_dir}/{ASC_REGISTRY_CHECK_POINT_FILE_NAME}"),
            true,
        )
        .unwrap();

        // prepare dirs
        let ports = VCPKG_PORTS_DIR_NAME.replace("/", "");
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
            .position(|c| c.0.hash == sync_check_point.check_point.hash)
        {
            next_index = index + 1;
        }
        let mut need_update = false;
        for (index, (commit, changed_files)) in
            vcpkg_ports_changed_commits[next_index..].iter().enumerate()
        {
            need_update = true;

            // get all ports
            let mut all_port_versions = BTreeMap::new();
            for (port, (control_file_text, vcpkg_json_file_text)) in
                git::ls_tree::list_ports(&commit.hash, &vcpkg_registry_dir, true)
            {
                if !control_file_text.is_empty() {
                    all_port_versions.insert(
                        port,
                        VcpkgPortManifest::get_version_from_control_file(&control_file_text),
                    );
                } else if !vcpkg_json_file_text.is_empty() {
                    all_port_versions.insert(
                        port,
                        VcpkgPortManifest::get_version_from_vcpkg_json_file(&vcpkg_json_file_text),
                    );
                }
            }

            // get port git trees
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

            // output git archive
            git::archive::run(
                &vcpkg_registry_dir,
                "tar",
                &tmp_tar_path,
                &commit.hash,
                VCPKG_PORTS_DIR_NAME,
            );
            // extract git archive
            util::fs::create_dirs(&tmp_ports_path);
            shell::run(
                "tar",
                &vec!["-xf", &tmp_tar_path],
                &tmp_ports_path,
                false,
                false,
                true,
            )
            .unwrap();
            util::fs::remove_file(&tmp_tar_path);

            // append version to port name in CONTROL/vcpkg.json
            for port_name in &changed_ports {
                self.append_version_to_port_manifest(
                    format!("{tmp_ports_path}/{port_name}"),
                    &all_port_versions,
                );
            }

            // remove tmp ports dir
            util::fs::remove_dirs(&tmp_ports_path);

            // git add ports
            git::add::run(&vec![VCPKG_PORTS_DIR_NAME.to_string()], &vcpkg_registry_dir);
            git::commit::run(
                format!("flatten microsoft/vcpkg {}", commit.hash.split_at(7).0),
                &vcpkg_registry_dir,
            );

            // git add versions
            git::add::run(
                &vec![VCPKG_VERSIONS_DIR_NAME.to_string()],
                &vcpkg_registry_dir,
            );
            git::commit_amend::run(&vcpkg_registry_dir);

            // save asc registry check point
            if index % 200 == 0 || vcpkg_ports_changed_commits.len() < 1000 {
                need_update = false;
                sync_check_point.check_point = commit.clone();
                sync_check_point.modified = chrono::Local::now()
                    .format("%Y-%m-%d %H:%M:%S.%f %z")
                    .to_string();
                sync_check_point.dump(true, false);
            }
        }

        // remove tmp dir
        util::fs::remove_dirs(&tmp_dir);

        // save asc registry check point
        if need_update {
            sync_check_point.check_point = vcpkg_ports_changed_commits.last().unwrap().0.clone();
            sync_check_point.modified = chrono::Local::now()
                .format("%Y-%m-%d %H:%M:%S.%f %z")
                .to_string();
            sync_check_point.dump(true, false);
        }

        return true;
    }

    pub fn append_version_to_port_manifest(
        &mut self,
        port_manifest_dir: String,
        all_port_versions: &BTreeMap<String, String>,
    ) {
        let control_file = format!("{port_manifest_dir}/CONTROL");
        let vcpkg_json_file = format!("{port_manifest_dir}/vcpkg.json");
        if util::fs::is_file_exists(&control_file) {
            let version = VcpkgPortManifest::update_control_file(&control_file, all_port_versions);
            std::fs::rename(&port_manifest_dir, format!("{port_manifest_dir}-{version}")).unwrap();
        } else if util::fs::is_file_exists(&vcpkg_json_file) {
            let version =
                VcpkgPortManifest::update_vcpkg_json_file(&vcpkg_json_file, all_port_versions);
            std::fs::rename(&port_manifest_dir, format!("{port_manifest_dir}-{version}")).unwrap();
        };
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
}
