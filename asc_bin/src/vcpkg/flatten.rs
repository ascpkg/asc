use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;
use config_file_types;

use crate::{
    config::relative_paths::{
        ASC_REGISTRY_CHECK_POINT_FILE_NAME, ASC_REGISTRY_DIR_NAME, VCPKG_DIR_NAME,
        VCPKG_PORTS_DIR_NAME, VCPKG_VERSIONS_DIR_NAME,
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

        let tmp_dir = format!("{asc_registry_dir}/tmp");
        let tar_name = "ports.tar";
        let tar_path = format!("{asc_registry_dir}/tmp/{tar_name}");

        // get vcpkg registry commits
        let vcpkg_commits =
            git::log::get_changed_commits(&vcpkg_registry_dir, VCPKG_PORTS_DIR_NAME);
        for (commit, changed_files) in vcpkg_commits {
            if commit.hash == sync_check_point.check_point.hash {
                continue;
            }

            // get port git trees
            let mut ports = BTreeSet::new();
            for file in changed_files {
                ports.insert(
                    file.split_at(VCPKG_PORTS_DIR_NAME.len())
                        .1
                        .split_once("/")
                        .unwrap()
                        .0
                        .to_string(),
                );
            }

            // create tmp dir
            util::fs::create_dirs(&tmp_dir);
            // output git archive
            git::archive::run(
                &vcpkg_registry_dir,
                "tar",
                &tar_path,
                &commit.hash,
                VCPKG_PORTS_DIR_NAME,
            );
            // extract git archive
            shell::run("tar", &vec!["-xf", tar_name], &tmp_dir, false, false, true).unwrap();
            util::fs::remove_file(&tar_path);

            // append version to port name in CONTROL/vcpkg.json

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

            // remove tmp dir
            util::fs::remove_dirs(&tmp_dir);

            // save asc registry check point
            sync_check_point.check_point = commit.clone();
            sync_check_point.modified = chrono::Local::now()
                .format("%Y-%m-%d %H:%M:%S.%f %z")
                .to_string();
            sync_check_point.dump(true, false);
        }

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
}
