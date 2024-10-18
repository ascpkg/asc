use crate::clang;
use crate::util;

pub fn path(options: &util::cli::CommandLines) -> String {
    format!("{}/CMakeLists.txt", &options.project_dir)
}

pub fn gen(options: &util::cli::CommandLines, source_mappings: &clang::parser::SourceMappings) {
    let project_dir_length = options.project_dir.len() + 1;

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
    for (header, sources) in &source_mappings.header_include_by_sources {
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
        txt.push_str(r#"FILE(GLOB"#);
        txt.push_str("\n");
        txt.push_str(&format!(r#"    {}"#, classify));
        txt.push_str("\n");
        for src in sources {
            txt.push_str(&format!(r#"    "{}""#, src));
            txt.push_str("\n");
        }
        txt.push_str(r#")"#);
        txt.push_str("\n");
    }
    txt.push_str("\n");

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
    txt.push_str("\n");

    if options.cmake_target_type == util::cli::CMakeTargetType::Executable {
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
    if options.cmake_lib_type == util::cli::CMakeLibType::Shared {
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

    if options.cmake_target_type == util::cli::CMakeTargetType::Executable {
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
    }

    if options.cmake_target_type == util::cli::CMakeTargetType::Library {
        if options.cmake_lib_type == util::cli::CMakeLibType::Shared {
            // export dynamic symbols
            txt.push_str(r#"# export dynamic library symbols"#);
            txt.push_str("\n");
            txt.push_str(r#"if(BUILD_SHARED_LIBS)"#);
            txt.push_str("\n");
            txt.push_str(r#"    add_definitions(-DBUILD_SHARED_LIBS)"#);
            txt.push_str("\n");
            txt.push_str(&format!(
                r#"    add_definitions(-D{}_EXPORTS)"#,
                options.project.to_uppercase()
            ));
            txt.push_str("\n");
            txt.push_str(r#"endif()"#);
            txt.push_str("\n\n");
        }

        // export library
        txt.push_str(r#"# export library"#);
        txt.push_str("\n");
        txt.push_str(r#"add_library("#);
        txt.push_str("\n");
        txt.push_str(r#"    ${PROJECT_NAME}::${PROJECT_NAME}"#);
        txt.push_str("\n");
        txt.push_str(r#"    ALIAS"#);
        txt.push_str("\n");
        txt.push_str(r#"    ${PROJECT_NAME}"#);
        txt.push_str("\n");
        txt.push_str(")\n\n");

        // export include path
        txt.push_str(r#"# export include path"#);
        txt.push_str("\n");
        txt.push_str(r#"target_include_directories("#);
        txt.push_str("\n");
        txt.push_str(r#"    ${PROJECT_NAME}"#);
        txt.push_str("\n");
        txt.push_str(r#"    INTERFACE"#);
        txt.push_str("\n");
        txt.push_str(r#"    $<INSTALL_INTERFACE:include>"#);
        txt.push_str("\n");
        txt.push_str(")\n\n");
    }

    txt.push_str(r#"set(CMAKE_INSTALL_PREFIX "D:/sources/FutureOrientedGB/auto_build_source_tree/test_c_cpp/build/target")"#);
    txt.push_str("\n");

    // install archive, library, bin
    txt.push_str(r#"# install archive, library, bin"#);
    txt.push_str("\n");
    txt.push_str(r#"install("#);
    txt.push_str("\n");
    txt.push_str(r#"    TARGETS ${PROJECT_NAME}"#);
    txt.push_str("\n");
    txt.push_str(r#"    EXPORT ${PROJECT_NAME}"#);
    txt.push_str("\n");
    txt.push_str(r#"    ARCHIVE DESTINATION lib"#);
    txt.push_str("\n");
    txt.push_str(r#"    LIBRARY DESTINATION lib"#);
    txt.push_str("\n");
    txt.push_str(r#"    RUNTIME DESTINATION bin"#);
    txt.push_str("\n");
    txt.push_str(")\n\n");

    if options.cmake_target_type == util::cli::CMakeTargetType::Library {
        // install headers
        txt.push_str(r#"# install headers"#);
        txt.push_str("\n");
        for header in source_mappings.header_include_by_sources.keys() {
            let src_prefix_length = std::path::Path::new(&options.source_dir)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .len()
                + 1;
            let dst_prefix_length = options.source_dir.len() + 1;

            txt.push_str(r#"install("#);
            txt.push_str(r#"FILES"#);
            txt.push_str(&format!(
                r#" "{}""#,
                header.clone().split_off(src_prefix_length)
            ));
            txt.push_str(r#" DESTINATION"#);
            txt.push_str(&format!(
                r#" include/${{PROJECT_NAME}}/{}"#,
                header.clone().split_off(dst_prefix_length)
            ));
            txt.push_str(")\n");
        }
        txt.push_str("\n\n");
    }

    // write cmake config
    std::fs::write(
        format!(
            "{}/{}-config.cmake.in",
            &options.project_dir, &options.project
        ),
        vec![
            "@PACKAGE_INIT@",
            "",
            &format!(
                r#"include("${{CMAKE_CURRENT_LIST_DIR}}/{}-targets.cmake")"#,
                &options.project
            ),
            "",
            &format!("check_required_components({})", &options.project),
            "",
        ]
        .join("\n")
        .as_bytes(),
    )
    .unwrap();

    // install package config
    txt.push_str(r#"# install package config"#);
    txt.push_str("\n");
    txt.push_str(r#"include(CMakePackageConfigHelpers)"#);
    txt.push_str("\n");
    txt.push_str(r#"configure_package_config_file("#);
    txt.push_str("\n");
    txt.push_str(r#"    "${CMAKE_CURRENT_SOURCE_DIR}/${PROJECT_NAME}-config.cmake.in""#);
    txt.push_str("\n");
    txt.push_str(r#"    "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}-config.cmake""#);
    txt.push_str("\n");
    txt.push_str(r#"    INSTALL_DESTINATION "share/${PROJECT_NAME}""#);
    txt.push_str("\n");
    txt.push_str(")\n");
    txt.push_str(r#"install("#);
    txt.push_str(r#"FILES"#);
    txt.push_str(r#" "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}-config.cmake""#);
    txt.push_str(r#" DESTINATION"#);
    txt.push_str(r#" "share/${PROJECT_NAME}""#);
    txt.push_str(")\n\n");

    // install cmake targets
    txt.push_str(r#"# install cmake targets"#);
    txt.push_str("\n");
    txt.push_str(r#"install("#);
    txt.push_str("\n");
    txt.push_str(r#"    EXPORT ${PROJECT_NAME}-targets"#);
    txt.push_str("\n");
    txt.push_str(r#"    DESTINATION share/${PROJECT_NAME}"#);
    txt.push_str("\n");
    txt.push_str(r#"    NAMESPACE ${PROJECT_NAME}::"#);
    txt.push_str("\n");
    txt.push_str(")\n\n");

    // write CMakeLists.txt
    std::fs::write(path(&options), txt.as_bytes()).unwrap();
}
