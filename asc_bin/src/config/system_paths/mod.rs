use crate::util;

static QUALIFIER: &str = "";
static ORGANIZATION: &str = "";
static APPLICATION: &str = "asc";

fn build(prefix: &str, names: Vec<String>, ensure_dirs: bool, is_dir: bool) -> String {
    let path = format!("{prefix}/{}", names.join("/"));
    let dir = if is_dir {
        path.clone()
    } else {
        util::fs::get_parent_dir(&path)
    };
    if ensure_dirs && !dir.is_empty() && !util::fs::is_dir_exists(&dir) {
        util::fs::create_dirs(&dir);
    }
    return path;
}

pub mod conf;
pub use conf::ConfigPath;

pub mod data;
pub use data::DataPath;
