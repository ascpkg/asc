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

pub fn is_executable_or_dynamic_library(path: &str) -> bool {
    #[cfg(target_family = "unix")]
    {
        if path.ends_with(".so") {
            return true;
        }

        use std::os::unix::fs::MetadataExt;
        if let Ok(metadata) = std::fs::metadata(path) {
            return metadata.mode() & 0o111 != 0;
        }
    }

    #[cfg(target_family = "windows")]
    {
        if path.ends_with(".exe") {
            return true;
        }
        if path.ends_with(".dll") {
            return true;
        }
    }

    false
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

pub fn is_patch(ext: &std::ffi::OsStr) -> bool {
    ext == "patch"
}

pub fn find_patch_files(dir: &String) -> Vec<String> {
    let mut files = Vec::new();

    let walker = walkdir::WalkDir::new(dir.clone())
        .into_iter()
        .filter_map(|e| e.ok());
    for entry in walker {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if is_patch(ext) {
                if let Some(file_name) = path.to_str() {
                    files.push(file_name.replace(r"\", "/"));
                }
            }
        }
    }

    files
}

pub fn find_executable_and_dynamic_library_files(dir: &String) -> Vec<String> {
    let mut files = Vec::new();

    let walker = walkdir::WalkDir::new(dir.clone())
        .into_iter()
        .filter_map(|e| e.ok());
    for entry in walker {
        let path = entry.path();
        if is_executable_or_dynamic_library(path.to_str().unwrap()) {
            if let Some(file_name) = path.to_str() {
                files.push(file_name.replace(r"\", "/"));
            }
        }
    }

    files
}

pub fn is_file_exists(path: &str) -> bool {
    if let Ok(metadata) = std::fs::metadata(path) {
        if metadata.is_file() {
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
