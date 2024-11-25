use crate::{
    git::{self, log::GIT_LOG_FORMAT_ABBR_COMMIT_HASH_DATE},
    util,
};

pub fn make_package(name: &str, dir: String, pack_cli: &str) {
    let mut command = String::new();
    let mut args = vec![];
    for arg in pack_cli.split(" ") {
        if command.is_empty() {
            command = arg.to_string();
        } else {
            args.push(arg);
        }
    }

    let info = git::log::get_latest_commit(".", GIT_LOG_FORMAT_ABBR_COMMIT_HASH_DATE);
    let work_dir = util::fs::get_parent_dir(&dir);
    let new_name: String = format!(
        "{name}.{}.{}.{}",
        info.hash,
        info.date_time[..10].replace("-", ""),
        info.date_time[11..19].replace(":", ""),
    );
    let new_dir: String = format!(
        "{work_dir}/{new_name}",
    );

    if command == "auto" {
        if cfg!(target_os = "windows") {
            command = String::from("7z");
            args.push("a");
            args.push("-mx9");
            let f = format!("{new_name}.7z");
            args.push(&f);
            args.push(&new_name);
            pack(&dir, &new_dir, command, args);
        } else {
            command = String::from("tar");
            args.push("-Jcf");
            let f = format!("{new_name}.tar.xz");
            args.push(&f);
            args.push(&new_name);
            pack(&dir, &new_dir, command, args);
        };
    } else if command == "7z" {
        let f = format!("{new_name}.7z");
        if args.is_empty() {
            args.push("a");
            args.push("-mx9");
            args.push(&f);
            args.push(&new_name);
        }
        pack(&dir, &new_dir, command, args)
    } else if command == "tar" {
        let f = format!("{new_name}.tar.xz");
        if args.is_empty() {
            args.push("-Jcf");
            args.push(&f);
            args.push(&new_name);
        }
        pack(&dir, &new_dir, command, args)
    } else if command == "issc" {
        if args.is_empty() {
            args.push(crate::config::relative_paths::INNO_SETUP_ISS_FILE_NAME);
            args.push(&new_name);
        }
        pack(&dir, &new_dir, command, args)
    }
}

fn pack(dir: &str, new_dir: &str, command: String, args: Vec<&str>) {
    let stat = git::log::get_latest_commit_stat(".");
    let version_txt_path = format!(
        "{dir}/{}",
        crate::config::relative_paths::VERSION_TXT_FILE_NAME
    );
    std::fs::write(&version_txt_path, stat.as_bytes()).unwrap();

    std::fs::rename(dir, &new_dir).unwrap();

    let work_dir = util::fs::get_parent_dir(&dir);
    util::shell::run(&command, &args, &work_dir, false, false, false).unwrap();

    std::fs::rename(&new_dir, dir).unwrap();
    std::fs::remove_file(&version_txt_path).unwrap();
}
