use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;
use config_file_types;

use crate::{
    cli::commands::VcpkgArgs,
    config::relative_paths::{
        ASC_REGISTRY_CHECK_POINT_FILE_NAME, ASC_REGISTRY_DIR_NAME, VCPKG_DIR_NAME,
    },
    git::{
        self,
        log::{GitCommitInfo, GIT_LOG_FORMAT_COMMIT_HASH_DATE},
    },
    util,
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

    history: Vec<(String, GitCommitInfo)>,

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

        // get vcpkg registry commits
        let vcpkg_commits =
            git::log::get_commits(&vcpkg_registry_dir, GIT_LOG_FORMAT_COMMIT_HASH_DATE);
        let (mut control_entry, mut vcpkg_json_entry, mut port_versions_entry, mut version_entry) =
            (true, false, false, false);
        for commit in vcpkg_commits {
            if commit.hash == sync_check_point.check_point.hash {
                continue;
            }

            // reset vcpkg registry to commit
            git::reset::run(&vcpkg_registry_dir, "", &commit.hash);

            // set vcpkg registry break changes flag
            if &commit.hash == GIT_COMMIT_HASH_ADD_VCPKG_JSON {
                control_entry = false;
                vcpkg_json_entry = true;
            } else if &commit.hash == GIT_COMMIT_HASH_ADD_PORT_VERSIONS {
                port_versions_entry = true;
            } else if &commit.hash == GIT_COMMIT_HASH_RENAME_TO_VERSIONS {
                version_entry = true;
                port_versions_entry = false;
            }

            // sync vcpkg registry and flatten it to asc registry
            self.sync_and_flatten(
                control_entry,
                vcpkg_json_entry,
                port_versions_entry,
                version_entry,
                &vcpkg_registry_dir,
                &asc_registry_dir,
            );

            // save asc registry check point
            let local_now = chrono::Local::now();
            sync_check_point.check_point = commit.clone();
            sync_check_point.history.push((
                local_now.format("%Y-%m-%d %H:%M:%S.%f %z").to_string(),
                commit.clone(),
            ));
            sync_check_point.dump(true, false);
        }

        return true;
    }

    pub fn sync_and_flatten(
        &mut self,
        control_entry: bool,
        vcpkg_json_entry: bool,
        port_versions_entry: bool,
        version_entry: bool,
        vcpkg_registry_dir: &str,
        asc_registry_dir: &str,
    ) {
        self.parse_control_or_vcpkg_json_manifiest(
            control_entry,
            vcpkg_json_entry,
            vcpkg_registry_dir,
            asc_registry_dir,
        );

        self.parse_versions(
            port_versions_entry,
            version_entry,
            vcpkg_registry_dir,
            asc_registry_dir,
        );
    }

    pub fn parse_control_or_vcpkg_json_manifiest(
        &mut self,
        control_entry: bool,
        vcpkg_json_entry: bool,
        vcpkg_registry_dir: &str,
        asc_registry_dir: &str,
    ) {
    }

    pub fn parse_versions(
        &mut self,
        port_versions_entry: bool,
        version_entry: bool,
        vcpkg_registry_dir: &str,
        asc_registry_dir: &str,
    ) {
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
