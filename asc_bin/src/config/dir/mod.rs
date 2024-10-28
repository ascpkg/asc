use crate::util;

static QUALIFIER: &str = "";
static ORGANIZATION: &str = "";
static APPLICATION: &str = "asc";

fn build(prefix: &str, name: &str) -> String {
    let path = format!("{prefix}/{name}");
    let dir = std::path::Path::new(&path)
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    if !util::fs::is_dir_exists(dir) {
        util::fs::create_dir(&dir);
    }
    return path;
}

pub mod conf;
pub use conf::ConfigDir;

pub mod data;
pub use data::DataDir;
