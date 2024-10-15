use bindgen;
use clap::Parser;
use walkdir;

#[derive(Clone, Debug, Parser)]
#[clap(version, about, long_about = None)]
pub struct CommandLines {
    #[clap(long, default_value = "", value_delimiter(','))]
    pub source_dirs: Vec<String>,
    #[clap(long, default_value = "", value_delimiter(','))]
    pub include_dirs: Vec<String>,
}

fn find_source_files(dir: &String) -> Vec<String> {
    let mut files = Vec::new();
    let walker = walkdir::WalkDir::new(dir.clone())
        .into_iter()
        .filter_map(|e| e.ok());
    for entry in walker {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "c" || ext == "cc" || ext == "cpp" || ext == "cxx" {
                if let Some(file_name) = path.to_str() {
                    files.push(file_name.to_string());
                }
            }
        }
    }
    files
}

#[derive(Debug)]
pub struct MyParseCallbacks {
    source_dir: String,
    rerun_on_header_files: bool,
}

impl MyParseCallbacks {
    pub fn new(source_dir: String) -> Self {
        Self {
            source_dir: source_dir,
            rerun_on_header_files: true,
        }
    }

    pub fn rerun_on_header_files(mut self, doit: bool) -> Self {
        self.rerun_on_header_files = doit;
        self
    }
}

impl Default for MyParseCallbacks {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl bindgen::callbacks::ParseCallbacks for MyParseCallbacks {
    fn header_file(&self, filename: &str) {
        if filename.starts_with(&self.source_dir) {
            println!("source_file: {filename}");
        }
    }

    fn include_file(&self, filename: &str) {
        if filename.starts_with(&self.source_dir) {
            println!("    include_file: {filename}");
        }
    }
}

fn main() {
    let options = CommandLines::parse();
    for source_dir in &options.source_dirs {
        for source_file in find_source_files(source_dir) {
            let mut builder = bindgen::Builder::default().header(source_file);
            for include_dir in &options.include_dirs {
                if !include_dir.is_empty() {
                    builder = builder.clang_arg("-I").clang_arg(include_dir);
                }
            }

            builder
                .parse_callbacks(Box::new(MyParseCallbacks::new(source_dir.clone())))
                .ignore_functions()
                .ignore_methods()
                .generate()
                .expect("Unable to generate source dependency tree");
        }
    }
}
