use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(version, about, long_about = None)]
pub struct CommandLines {
    #[clap(long, default_value = "", value_delimiter(','))]
    pub source_dirs: Vec<String>,
    #[clap(long, default_value = "", value_delimiter(','))]
    pub include_dirs: Vec<String>,
}
