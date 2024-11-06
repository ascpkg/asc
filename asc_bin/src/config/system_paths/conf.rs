use super::{build, APPLICATION, ORGANIZATION, QUALIFIER};

pub struct ConfigPath {}

impl ConfigPath {
    fn prefix() -> String {
        if let Some(dir) = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            return dir.config_dir().to_str().unwrap().replace(r"\", "/");
        }
        return String::new();
    }

    pub fn vcpkg_toml() -> String {
        build(
            &Self::prefix(),
            vec![String::from("vcpkg.toml")],
            true,
            false,
        )
    }
}
