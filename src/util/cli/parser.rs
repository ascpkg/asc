pub use super::types;

use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(version, about, long_about = None)]
pub struct Options {
    #[clap(long, default_value = "scan")]
    pub action_type: types::ActionType,

    #[clap(
        long,
        default_value = "",
        help = "default to parent dir name of source_dir"
    )]
    pub project: String,
    #[clap(long, default_value = "", help = "default to parent dir of source_dir")]
    pub project_dir: String,
    #[clap(long, default_value = "", help = "default to project_dir/build")]
    pub build_dir: String,
    #[clap(
        long,
        help = "source_dir must have a sub dir named src to store source files"
    )]
    pub source_dir: String,
    #[clap(long)]
    pub entry_point_source: String,
    #[clap(long, default_value = "", value_delimiter(','))]
    pub include_dirs: Vec<String>,

    #[clap(long, default_value = "3.20")]
    pub cmake_minimum_version: String,

    #[clap(long, default_value = "executable")]
    pub cmake_target_type: types::CMakeTargetType,

    #[clap(long, default_value = "static")]
    pub cmake_lib_type: types::CMakeLibraryType,

    #[clap(long, default_value = "debug")]
    pub cmake_config: types::CMakeConfigType,
}

impl Options {
    pub fn normalize(&mut self) {
        // set default project to parent dir name of source_dir
        if self.project.is_empty() {
            self.project = std::path::Path::new(&self.source_dir)
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        }

        // make source_dir to be absolute
        if !std::path::Path::new(&self.source_dir).is_absolute() {
            self.source_dir = format!(
                "{}/{}",
                std::env::current_dir().unwrap().to_str().unwrap(),
                self.source_dir
            );
        }
        // replace windows path separator in source_dir
        self.source_dir = self.source_dir.replace(r"\", "/");

        // replace windows path separator in include_dirs
        for include in self.include_dirs.iter_mut() {
            *include = include.replace(r"\", "/");
        }
        self.include_dirs.retain(|s| !s.is_empty());

        // set default project_dir to parent of source_dir
        if self.project_dir.is_empty() {
            self.project_dir = std::path::Path::new(&self.source_dir)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        }

        // set default build_dir to build in project_dir
        if self.build_dir.is_empty() {
            self.build_dir = format!("{}/build", &self.project_dir);
        }
    }
}
