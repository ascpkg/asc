use super::find::find_import_libraries;
use crate::{config, util};

pub fn copy_dependent_libraries(
    install_prefix: &str,
    profile: String,
    triplet: &str,
    executable_and_dynamic_library_files: Vec<String>,
) {
    // find deps
    let mut installed_libs = std::collections::HashSet::new();
    let installed_bin_dir =
        config::relative_paths::vcpkg_installed_bin_dir_path(&profile, &triplet);
    installed_libs.extend(util::fs::find_executable_and_dynamic_library_files(
        &installed_bin_dir,
    ));
    installed_libs.extend(util::fs::find_executable_and_dynamic_library_files(
        &config::relative_paths::vcpkg_installed_lib_dir_path(&profile, &triplet),
    ));

    let mut parsed_files = std::collections::HashSet::new();

    let mut deps = std::collections::HashSet::new();
    deps.extend(executable_and_dynamic_library_files.clone());

    loop {
        let mut mark_to_insert = std::collections::HashSet::new();
        for dep in &deps {
            if parsed_files.contains(dep) {
                continue;
            }
            parsed_files.insert(dep.clone());
            for lib_path in find_import_libraries(dep) {
                if installed_libs.contains(&lib_path) {
                    mark_to_insert.insert(lib_path);
                } else {
                    let lib_path = format!("{installed_bin_dir}/{lib_path}");
                    if installed_libs.contains(&lib_path) {
                        mark_to_insert.insert(lib_path);
                    }
                }
            }
        }
        if mark_to_insert.is_empty() {
            break;
        }
        deps.extend(mark_to_insert);
    }

    // copy deps
    for src_file in deps {
        let dst_file = format!(
            "{install_prefix}/{}/{}",
            config::relative_paths::CMAKE_INSTALL_BIN_DIR_NAME,
            std::path::PathBuf::from(&src_file)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        );
        if executable_and_dynamic_library_files.contains(&src_file) {
            continue;
        }
        let mut silent = false;
        if util::fs::is_file_exists(&dst_file) {
            silent = true;
            util::fs::remove_file(&dst_file);
            tracing::info!("-- Up-to-date: copy {src_file} to {dst_file}");
        }
        match std::fs::copy(&src_file, &dst_file) {
            Ok(_) => {
                if !silent {
                    tracing::info!("-- Installing: copy {src_file} to {dst_file}");
                }
            }
            Err(e) => {
                tracing::error!(
                    message = format!("copy {src_file} to {dst_file}"),
                    error = e.to_string()
                );
            }
        }
    }
}
