use crate::{cli, util};

pub fn gen(options: &cli::commands::scan::ScanOptions) {
    let mut args = vec!["-S", &options.project_dir, "-B", &options.target_dir];

    if options.shared_lib {
        args.push("-D BUILD_SHARED_LIBS=1");
    }

    util::shell::run("cmake", &args, false, false).unwrap();
}
