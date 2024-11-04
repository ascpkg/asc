use crate::errors::ErrorTag;

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

pub fn get_parent_dir(current: &str) -> String {
    std::path::Path::new(current)
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(r"\", "/")
}

pub fn set_cwd(dir: &str) -> bool {
    std::env::set_current_dir(dir).is_ok()
}

pub fn is_dir_exists(path: &str) -> bool {
    if let Ok(metadata) = std::fs::metadata(path) {
        if metadata.is_dir() {
            return true;
        }
    }
    return false;
}

pub fn create_dir(path: &str) -> bool {
    match std::fs::create_dir(path) {
        Ok(_) => {
            tracing::info!(func = "std::fs::create_dir", path = path,);
            return true;
        }
        Err(e) => {
            tracing::error!(
                func = "std::fs::create_dir",
                path = path,
                error_tag = ErrorTag::CreateDirectoryError.as_ref(),
                error_str = e.to_string(),
            );
            return false;
        }
    }
}

pub fn create_dirs(path: &str) -> bool {
    match std::fs::create_dir_all(path) {
        Ok(_) => {
            tracing::info!(func = "std::fs::create_dir_all", path = path,);
            return true;
        }
        Err(e) => {
            tracing::error!(
                func = "std::fs::create_dir_all",
                path = path,
                error_tag = ErrorTag::CreateDirectoryError.as_ref(),
                error_str = e.to_string(),
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
