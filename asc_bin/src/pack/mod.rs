use crate::util;

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

    if command == "auto" {
        if cfg!(target_os = "windows") {
            command = String::from("7z");
            args.push("a");
            args.push("-mx9");
            let f = format!("{name}.7z");
            args.push(&f);
            args.push(name);
            pack(&dir, name, command, args);
        } else {
            command = String::from("tar");
            args.push("-Jcf");
            let f = format!("{name}.tar.xz");
            args.push(&f);
            args.push(name);
            pack(&dir, name, command, args);
        };
    } else if command == "7z" {
        let f = format!("{name}.7z");
        if args.is_empty() {
            args.push("a");
            args.push("-mx9");
            args.push(&f);
            args.push(name);
        }
        pack(&dir, name, command, args)
    } else if command == "tar" {
        let f = format!("{name}.tar.xz");
        if args.is_empty() {
            args.push("-Jcf");
            args.push(&f);
            args.push(name);
        }
        pack(&dir, name, command, args)
    } else if command == "issc" {
        let f = format!("{name}.iss");
        if args.is_empty() {
            args.push(&f);
            args.push(name);
        }
        pack(&dir, name, command, args)
    }
}

fn pack(dir: &str, name: &str, command: String, args: Vec<&str>) {
    let work_dir = util::fs::get_parent_dir(dir);
    let new = format!("{work_dir}/{name}");
    std::fs::rename(dir, &new).unwrap();
    util::shell::run(&command, &args, &work_dir, false, false, false).unwrap();
    std::fs::rename(&new, dir).unwrap();
}
