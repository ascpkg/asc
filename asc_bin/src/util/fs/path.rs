pub fn remove_prefix(path: &String, source_dir: &String, target_dir: &String) -> String {
    if path == source_dir || path == target_dir {
        String::new()
    } else if path.starts_with(source_dir) {
        path.clone().split_off(source_dir.len() + 1)
    } else if path.starts_with(target_dir) {
        path.clone().split_off(target_dir.len() + 1)
    } else {
        String::new()
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
