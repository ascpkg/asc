use walkdir;

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

pub fn remove_prefix(path: &String, source_dir: &String, build_dir: &String) -> String {
    if path == source_dir || path == build_dir {
        String::new()
    } else if path.starts_with(source_dir) {
        path.clone().split_off(source_dir.len() + 1)
    } else if path.starts_with(build_dir) {
        path.clone().split_off(build_dir.len() + 1)
    } else {
        String::new()
    }
}
