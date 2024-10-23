use walkdir;

use crate::errors::ErrorTag;

pub fn is_source(ext: &std::ffi::OsStr) -> bool {
    ext == "c" || ext == "cc" || ext == "cpp" || ext == "cxx"
}

pub fn is_cxx_source(name: &String) -> bool {
    let path = std::path::Path::new(name);

    if let Some(ext) = path.extension() {
        return ext == "hpp" || ext == "cc" || ext == "cpp" || ext == "cxx";
    }

    return false;
}

pub fn find_source_files(dir: &String) -> Vec<String> {
    let mut files = Vec::new();

    let walker = walkdir::WalkDir::new(dir.clone())
        .into_iter()
        .filter_map(|e| e.ok());
    for entry in walker {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if is_source(ext) {
                if let Some(file_name) = path.to_str() {
                    files.push(file_name.replace(r"\", "/"));
                }
            }
        }
    }

    files
}

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

pub fn get_cwd() -> String {
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(r"\", "/")
}

pub fn get_cwd_name() -> String {
    std::env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_cwd_parent() -> String {
    std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(r"\", "/")
}

pub fn set_cwd(dir: &str) -> bool {
    std::env::set_current_dir(dir).is_ok()
}

pub fn is_file_exists(path: &str) -> bool {
    if let Ok(metadata) = std::fs::metadata(path) {
        if metadata.is_file() {
            return true;
        }
    }
    return false;
}

pub fn is_dir_exists(path: &str) -> bool {
    if let Ok(metadata) = std::fs::metadata(path) {
        if metadata.is_dir() {
            return true;
        }
    }
    return false;
}

pub fn remove_file(path: &str) -> bool {
    match std::fs::remove_file(path) {
        Ok(_) => {
            tracing::info!(func = "std::fs::remove_file", path = path);
            return true;
        }
        Err(e) => {
            tracing::error!(
                func = "std::fs::remove_file",
                path = path,
                error_tag = ErrorTag::RemoveFileError.as_ref(),
                error_str = e.to_string()
            );
            return false;
        }
    }
}

pub fn remove_dirs(path: &str) -> bool {
    match std::fs::remove_dir_all(path) {
        Ok(_) => {
            tracing::info!(func = "std::fs::remove_dir_all", path = path,);
            return true;
        }
        Err(e) => {
            tracing::error!(
                func = "std::fs::remove_dir_all",
                path = path,
                error_tag = ErrorTag::RemoveDirectoryError.as_ref(),
                error_str = e.to_string(),
            );
            return false;
        }
    }
}
