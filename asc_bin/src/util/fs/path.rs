use crate::util;

pub fn remove_prefix(path: &String, source_dir: &String, target_dir: &String) -> String {
    if path == source_dir || path == target_dir {
        String::new()
    } else if path.starts_with(source_dir) {
        path.clone().split_off(source_dir.len() + 1)
    } else if path.starts_with(target_dir) {
        path.clone().split_off(target_dir.len() + 1)
    } else {
        path.clone()
    }
}

pub fn replace_common_prefix(
    path: &String,
    source_dir: &String,
    target_dir: &String,
    replacement: &str,
) -> String {
    let common_prefix = util::str::longest_common_prefix(source_dir, target_dir);
    if path.starts_with(&common_prefix) {
        path.replace(&common_prefix, replacement)
    } else {
        path.clone()
    }
}

pub fn get_file_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
