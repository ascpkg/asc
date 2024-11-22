use goblin;

pub fn find_import_libraries(path: &str) -> Vec<String> {
    let content = std::fs::read(path).unwrap();
    match goblin::Object::parse(&content).unwrap() {
        goblin::Object::Elf(elf) => {
            // Linux/Unix Elf
            return elf
                .libraries
                .iter()
                .map(|lib| lib.to_string())
                .collect::<Vec<String>>();
        }
        goblin::Object::PE(pe) => {
            // Windows PE
            return pe
                .libraries
                .iter()
                .map(|lib| lib.to_string())
                .collect::<Vec<String>>();
        }
        goblin::Object::Mach(_mach) => match goblin::mach::Mach::parse(&content).unwrap() {
            goblin::mach::Mach::Binary(binary) => {
                // macOS single Mach-o
                let mut results = vec![];
                for command in binary.load_commands.iter() {
                    if let goblin::mach::load_command::CommandVariant::LoadDylib(load_dylib) =
                        &command.command
                    {
                        let name = load_dylib.dylib.name.to_string();
                        if !results.contains(&name) {
                            results.push(name);
                        }
                    }
                }
                return results;
            }
            goblin::mach::Mach::Fat(fat) => {
                // macOS multiple Mach-o
                let mut results = vec![];
                for arch in fat.iter_arches() {
                    if let Ok(mach) =
                        goblin::mach::Mach::parse(&content[arch.unwrap().offset as usize..])
                    {
                        if let goblin::mach::Mach::Binary(binary) = mach {
                            for command in binary.load_commands.iter() {
                                if let goblin::mach::load_command::CommandVariant::LoadDylib(
                                    load_dylib,
                                ) = &command.command
                                {
                                    let name = load_dylib.dylib.name.to_string();
                                    if !results.contains(&name) {
                                        results.push(name);
                                    }
                                }
                            }
                        }
                    }
                }
                return results;
            }
        },
        _ => Vec::new(),
    }
}
