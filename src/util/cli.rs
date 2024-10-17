use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(version, about, long_about = None)]
pub struct CommandLines {
    #[clap(long)]
    pub project: String,
    #[clap(long)]
    pub source_dir: String,
    #[clap(long)]
    pub entry_point_source: String,
    #[clap(long, default_value = "", value_delimiter(','))]
    pub include_dirs: Vec<String>,
}

impl CommandLines {
    pub fn replace(&mut self) {
        self.source_dir = self.source_dir.replace(r"\", "/");

        for include in self.include_dirs.iter_mut() {
            *include = include.replace(r"\", "/");
        }
    }

    pub fn format(&mut self) {}
}
