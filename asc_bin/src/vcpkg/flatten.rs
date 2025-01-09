use std::collections::{BTreeMap, BTreeSet};

use fs_extra;
use rayon;

use crate::{
    config::{
        relative_paths::{
            ASC_REGISTRY_DIR_NAME, VCPKG_BASELINE_JSON_FILE_NAME, VCPKG_CONTROL_FILE_NAME,
            VCPKG_DIR_NAME, VCPKG_JSON_FILE_NAME, VCPKG_PORTS_DIR_NAME, VCPKG_VERSIONS_DIR_NAME,
        },
        vcpkg::{port_manifest::VcpkgPortManifest, versions_baseline::VcpkgBaseline},
    },
    git::{self, log::GitCommitInfo},
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
        util::fs::remove_dirs(&tmp_dir);
        util::fs::create_dirs(&tmp_dir);

        // get vcpkg registry commits
        let vcpkg_ports_changed_commits =
            git::log::get_changed_commits(&vcpkg_registry_dir, VCPKG_PORTS_DIR_NAME);
        let mut next_index = vcpkg_ports_changed_commits
            .iter()
            .position(|c| c.0.hash.starts_with(&check_point_hash))
            .unwrap_or(0); // redo last commit because versions dir may not be added and committed

        // process ports/ in thread pool
        let threads = self.args.threads as usize;
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();
        let total_count = vcpkg_ports_changed_commits.len();
        let pending_tasks = std::sync::Arc::new(std::sync::Mutex::new(0));
        let pending_tasks_cloned = std::sync::Arc::clone(&pending_tasks);
        let (version_task_sender, version_task_receiver) = std::sync::mpsc::channel();
        let tmp_dir_cloned = tmp_dir.clone();
        let mut start_index = next_index;
        std::thread::spawn(move || {
            thread_pool.install(|| {
                let pending_tasks_cloned = std::sync::Arc::clone(&pending_tasks);
                while start_index < vcpkg_ports_changed_commits.len() {
                    // wait until pending_task_count <= threads
                    {
                        let mut pending_task_count = pending_tasks_cloned.lock().unwrap();
                        if *pending_task_count > threads {
                            drop(pending_task_count);
                            std::thread::sleep(std::time::Duration::from_secs(1));
                            tracing::warn!("wait until pending_task_count <= threads");
                            continue;
                        }
                        // pending_task_count += 1
                        *pending_task_count += 1;
                    }

                    // clone
                    let tmp_dir = tmp_dir_cloned.clone();
                    let tar_name = tar_name.clone();
                    let ports = ports.clone();
                    let vcpkg_registry_dir = vcpkg_registry_dir.clone();
                    let (commit, changed_files) = vcpkg_ports_changed_commits[start_index].clone();
                    let version_task_sender = version_task_sender.clone();

                    // spawn thread
                    let int_index = start_index;
                    start_index += 1;
                    thread_pool.spawn(move || {
                        let str_index = format!("{:0>8}", int_index);
                        let tmp_tar_path = format!("{tmp_dir}/{str_index}_{tar_name}");
                        let tmp_ports_path = format!("{tmp_dir}/{str_index}_{ports}");

                        tracing::warn!(
                            "---------- {} / {total_count} = {}% ----------",
                            int_index,
                            int_index as f32 * 100.0 / total_count as f32
                        );

                        // get all ports
                        let (all_port_versions, all_ports_manifest) =
                            Self::get_all_port_versions(&vcpkg_registry_dir, &commit.hash, false);

                        // get port git trees
                        let changed_ports = Self::get_changed_ports(&changed_files);

                        // output git archive
                        Self::output_git_archive(
                            &vcpkg_registry_dir,
                            &commit.hash,
                            &tmp_tar_path,
                            &tmp_ports_path,
                        );

                        // append version to port name in CONTROL/vcpkg.json
                        for port_name in &changed_ports {
                            Self::append_version_to_port_manifest(
                                format!("{tmp_ports_path}/{port_name}"),
                                &all_port_versions,
                            );
                        }

                        // send task
                        version_task_sender
                            .send((
                                int_index,
                                commit.clone(),
                                tmp_ports_path,
                                changed_ports,
                                all_port_versions,
                                all_ports_manifest,
                            ))
                            .unwrap();
                    });
                }
            });
        });

        // process versions/ one by one
        let mut reorder_map = BTreeMap::new();
        loop {
            if let Ok((
                index,
                commit,
                tmp_ports_path,
                changed_ports,
                all_port_versions,
                all_ports_manifest,
            )) = version_task_receiver.recv()
            {
                {
                    // pending_task_count -= 1
                    let mut pending_task_count = pending_tasks_cloned.lock().unwrap();
                    *pending_task_count -= 1;
                }

                if index == next_index {
                    self.process_versions(
                        index as f32,
                        total_count as f32,
                        &commit,
                        &asc_registry_dir,
                        &asc_registry_ports_dir,
                        &tmp_ports_path,
                        &changed_ports,
                        &all_port_versions,
                        &all_ports_manifest,
                    );
                    next_index += 1;
                    if index == total_count as usize - 1 {
                        break;
                    }
                } else {
                    reorder_map.insert(
                        index,
                        (
                            index,
                            commit,
                            tmp_ports_path,
                            changed_ports,
                            all_port_versions,
                            all_ports_manifest,
                        ),
                    );
                    match reorder_map.get(&next_index) {
                        None => {
                            std::thread::sleep(std::time::Duration::from_millis(100));
                            tracing::warn!("wait index == next_index");
                            continue;
                        }
                        Some((
                            int_index_,
                            commit_,
                            tmp_ports_path_,
                            changed_ports_,
                            all_port_versions_,
                            all_ports_manifest_,
                        )) => {
                            self.process_versions(
                                int_index_.clone() as f32,
                                total_count as f32,
                                &commit_,
                                &asc_registry_dir,
                                &asc_registry_ports_dir,
                                &tmp_ports_path_,
                                &changed_ports_,
                                &all_port_versions_,
                                &all_ports_manifest_,
                            );
                            next_index += 1;
                            if index == total_count as usize - 1 {
                                break;
                            }
                        }
                    }
                }
            }
        }

        // remove tmp dir
        util::fs::remove_dirs(&tmp_dir);

        return true;
    }

    fn process_versions(
        &self,
        index: f32,
        total: f32,
        commit: &GitCommitInfo,
        asc_registry_dir: &String,
        asc_registry_ports_dir: &String,
        tmp_ports_path: &String,
        changed_ports: &BTreeSet<String>,
        all_port_versions: &BTreeMap<String, String>,
        all_ports_manifest: &BTreeMap<String, (String, String)>,
    ) {
        tracing::warn!(
            "========== {} / {total} = {}% ==========",
            index as i32,
            index * 100.0 / total
        );

        // move versioned ports
        self.move_versioned_ports(
            asc_registry_ports_dir,
            &tmp_ports_path,
            &changed_ports,
            &all_port_versions,
        );

        // git add ports
        git::add::run(&vec![VCPKG_PORTS_DIR_NAME.to_string()], asc_registry_dir);
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
        if self.args.push {
            git::push::run(&asc_registry_dir, true);
        }
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
        vcpkg_registry_dir: &str,
        commit_hash: &str,
        skip_manifest: bool,
    ) -> (BTreeMap<String, String>, BTreeMap<String, (String, String)>) {
        if skip_manifest {
            let baseline_json_text = git::show::commit_file_content(
                vcpkg_registry_dir,
                commit_hash,
                &format!("{VCPKG_VERSIONS_DIR_NAME}/{VCPKG_BASELINE_JSON_FILE_NAME}"),
            );
            if let Some(baseline_data) = VcpkgBaseline::loads(&baseline_json_text, true) {
                let mut all_port_versions = BTreeMap::new();
                for (k, v) in baseline_data.default {
                    all_port_versions.insert(
                        k,
                        VcpkgPortManifest::normalize_port_name(v.format_version_text()),
                    );
                }
                return (all_port_versions, BTreeMap::new());
            }
        }

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

    pub fn get_changed_ports(changed_files: &Vec<String>) -> BTreeSet<String> {
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
