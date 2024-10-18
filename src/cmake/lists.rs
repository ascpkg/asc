use crate::clang;
use crate::util;

pub fn gen(
    options: &util::cli::CommandLines,
    source_mappings: &clang::parser::SourceMappings,
    project_dir: &String,
    target_type: &util::cli::CMakeTargetType,
    lib_type: &util::cli::CMakeLibType,
) -> String {
    let project_dir_length = project_dir.len() + 1;

    let mut txt = String::new();

    // set cmake version required
    txt.push_str(r#"# set cmake version required"#);
    txt.push_str("\n");
    txt.push_str(&format!(
        r#"cmake_minimum_required(VERSION {})"#,
        &options.cmake_minimum_version
    ));
    txt.push_str("\n\n");

    // set project name
    txt.push_str(r#"# set project name"#);
    txt.push_str("\n");
    txt.push_str(&format!(r#"project ({} C CXX)"#, &options.project));
    txt.push_str("\n\n");

    // configure msvc
    txt.push_str(r#"# configure msvc"#);
    txt.push_str("\n");
    txt.push_str(r#"if(MSVC)"#);
    txt.push_str("\n");
    txt.push_str(r#"    # utf-8 source code encoding"#);
    txt.push_str("\n");
    txt.push_str(r#"    add_compile_options("$<$<C_COMPILER_ID:MSVC>:/utf-8>")"#);
    txt.push_str("\n");
    txt.push_str(r#"    add_compile_options("$<$<CXX_COMPILER_ID:MSVC>:/utf-8>")"#);
    txt.push_str("\n\n");
    txt.push_str(r#"    # Visual Stuido - Properity - C/C++ - General - Debug Informat Format > Program Database (/Zi)"#);
    txt.push_str("\n");
    txt.push_str(r#"    set(CMAKE_CXX_FLAGS_DEBUUG "${CMAKE_CXX_FLAGS_DEBUG} /Zi /Od")  # Disable Optimization"#);
    txt.push_str("\n");
    txt.push_str(
        r#"    set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /Zi /O1")  # Favor Size"#,
    );
    txt.push_str("\n\n");
    txt.push_str(r#"    # Visual Stuido - Properity - Linker - Debugging - Generate Debug Info > Generate Debug Information (/Debug)"#);
    txt.push_str("\n");
    txt.push_str(r#"    set(CMAKE_EXE_LINKER_FLAGS_DEBUUG "${CMAKE_EXE_LINKER_FLAGS_DEBUG} /DEBUG /OPT:REF /OPT:ICF")"#);
    txt.push_str("\n");
    txt.push_str(r#"    set(CMAKE_EXE_LINKER_FLAGS_RELEASE "${CMAKE_EXE_LINKER_FLAGS_RELEASE} /DEBUG /OPT:REF /OPT:ICF")"#);
    txt.push_str("\n\n");
    txt.push_str(r#"    # Visual Stuido - Properity - Linker - Debugging - Generate Debug Info > Generate Debug Information (/Debug)"#);
    txt.push_str("\n");
    txt.push_str(r#"    set(CMAKE_SHARED_LINKER_FLAGS_DEBUUG "${CMAKE_SHARED_LINKER_FLAGS_DEBUG} /DEBUG /OPT:REF /OPT:ICF")"#);
    txt.push_str("\n");
    txt.push_str(r#"    set(CMAKE_SHARED_LINKER_FLAGS_RELEASE "${CMAKE_SHARED_LINKER_FLAGS_RELEASE} /DEBUG /OPT:REF /OPT:ICF")"#);
    txt.push_str("\n");
    txt.push_str(r#"endif(MSVC)"#);
    txt.push_str("\n\n");

    // group headers and source by dir name
    let mut group_sources =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    let mut classify_to_dir = std::collections::HashMap::<String, String>::new();
    for (header, sources) in &source_mappings.header_inclued_by_sources {
        {
            // group header
            let relative_path: String = header.clone().split_off(project_dir_length);
            let dir = std::path::Path::new(&relative_path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap();
            let classify = dir.replace("/", "_");
            classify_to_dir.insert(classify.clone(), dir.to_string());
            group_sources
                .entry(classify.to_string())
                .or_default()
                .insert(relative_path);
        }

        {
            for src in sources {
                // group source
                let relative_path = src.clone().split_off(project_dir_length);
                let dir = std::path::Path::new(&relative_path)
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let classify = dir.replace("/", "_");
                classify_to_dir.insert(classify.clone(), dir.to_string());
                group_sources
                    .entry(classify.to_string())
                    .or_default()
                    .insert(relative_path);
            }
        }
    }

    // glob headers and sources
    txt.push_str(r#"# glob headers and sources"#);
    txt.push_str("\n");
    for (classify, sources) in &group_sources {
        txt.push_str(r#"FILE(GLOB_RECURSE"#);
        txt.push_str("\n");
        txt.push_str(&format!(r#"    {}"#, classify));
        txt.push_str("\n");
        for src in sources {
            txt.push_str(&format!(r#"    "{}""#, src));
            txt.push_str("\n");
        }
        txt.push_str(r#")"#);
        txt.push_str("\n\n");
    }

    // group headers and sources
    txt.push_str(r#"# group headers and sources"#);
    txt.push_str("\n");
    for classify in group_sources.keys() {
        txt.push_str(r#"SOURCE_GROUP(""#);
        txt.push_str(classify_to_dir.get(classify).unwrap());
        txt.push_str(r#"" FILES ${"#);
        txt.push_str(&classify);
        txt.push_str(r#"})"#);
        txt.push_str("\n");
    }
    txt.push_str("\n\n");

    if target_type == &util::cli::CMakeTargetType::Executable {
        // generate executable
        txt.push_str(&format!(r#"# generate executable"#));
        txt.push_str("\n");
        txt.push_str(r#"add_executable("#);
    } else {
        // generate library
        txt.push_str(&format!(r#"# generate library"#));
        txt.push_str("\n");
        txt.push_str(r#"add_library("#);
    }
    txt.push_str("\n");
    txt.push_str(r#"    ${PROJECT_NAME}"#);
    txt.push_str("\n");
    if lib_type == &util::cli::CMakeLibType::Shared {
        txt.push_str(r#"    SHARED"#);
        txt.push_str("\n");
    }
    for classify in group_sources.keys() {
        txt.push_str(r#"    ${"#);
        txt.push_str(classify);
        txt.push_str("}\n");
    }
    txt.push_str(r#")"#);
    txt.push_str("\n\n");

    // set include paths
    if !options.include_dirs.is_empty() {
        txt.push_str(r#"# set include paths"#);
        txt.push_str("\n");
        txt.push_str(r#"target_include_directories("#);
        txt.push_str("\n");
        txt.push_str(r#"    ${PROJECT_NAME}"#);
        txt.push_str("\n");
        txt.push_str(r#"    PRIVATE"#);
        txt.push_str("\n");
        for dir in &options.include_dirs {
            txt.push_str(&format!(r#"    "{}""#, dir));
            txt.push_str("\n");
        }
        txt.push_str(r#"    ${CMAKE_CURRENT_BINARY_DIR}"#);
        txt.push_str("\n");
        txt.push_str(r#")"#);
        txt.push_str("\n\n");
    }

    // link libraries
    txt.push_str(r#"# link libraries"#);
    txt.push_str("\n");
    txt.push_str(r#"#target_link_libraries("#);
    txt.push_str("\n");
    txt.push_str(r#"#    ${PROJECT_NAME}"#);
    txt.push_str("\n");
    txt.push_str(r#"#    PRIVATE"#);
    txt.push_str("\n");
    txt.push_str(r#"#"#);
    txt.push_str("\n");
    txt.push_str(r#"#)"#);
    txt.push_str("\n\n");

    // add executable directory to deps search paths
    txt.push_str(r#"# add executable directory to deps search paths"#);
    txt.push_str("\n");
    txt.push_str(r#"set(CMAKE_SKIP_BUILD_RPATH FALSE) "#);
    txt.push_str("\n");
    txt.push_str(r#"set(CMAKE_INSTALL_RPATH "\$ORIGIN")"#);
    txt.push_str("\n");
    txt.push_str(r#"set(CMAKE_BUILD_WITH_INSTALL_RPATH TRUE)"#);
    txt.push_str("\n");
    txt.push_str(r#"set(CMAKE_INSTALL_RPATH_USE_LINK_PATH TRUE)"#);
    txt.push_str("\n\n");

    return txt;
}
