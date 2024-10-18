use crate::clang;
use crate::util;

pub fn gen(
    options: &util::cli::CommandLines,
    source_mappings: &clang::parser::SourceMappings,
    project_dir: &str,
) -> String {
    let project_dir_length = project_dir.len() + 1;

    let mut txt = String::new();

    txt.push_str(r#"# cmake version"#);
    txt.push_str("\n");
    txt.push_str(r#"cmake_minimum_required(VERSION 3.20)"#);
    txt.push_str("\n\n");

    txt.push_str(r#"# project name"#);
    txt.push_str("\n");
    txt.push_str(&format!(r#"project ({} C CXX)"#, &options.project));
    txt.push_str("\n\n");

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

    let mut compile_sources = std::collections::BTreeSet::<String>::new();

    txt.push_str(r#"# headers"#);
    txt.push_str("\n");
    txt.push_str(r#"FILE(GLOB_RECURSE HEADER_FILES"#);
    for (header, sources) in &source_mappings.header_inclued_by_sources {
        txt.push_str("\n");
        txt.push_str(&format!(
            r#"    "{}""#,
            header.clone().split_off(project_dir_length)
        ));

        for src in sources {
            compile_sources.insert(src.clone());
        }
    }
    txt.push_str("\n");
    txt.push_str(r#")"#);
    txt.push_str("\n\n");

    txt.push_str(r#"# sources"#);
    txt.push_str("\n");
    txt.push_str(r#"FILE(GLOB_RECURSE SOURCES_FILES"#);
    for source in &compile_sources {
        txt.push_str("\n");
        txt.push_str(&format!(
            r#"    "{}""#,
            source.clone().split_off(project_dir_length)
        ));
    }
    txt.push_str("\n");
    txt.push_str(r#")"#);
    txt.push_str("\n\n");

    txt.push_str(r#"# group"#);
    txt.push_str("\n");
    txt.push_str(r#"SOURCE_GROUP("headers" FILES ${HEADER_FILES})"#);
    txt.push_str("\n");
    txt.push_str(r#"SOURCE_GROUP("sources" FILES ${SOURCES_FILES})"#);
    txt.push_str("\n\n");

    txt.push_str(r#"# generate executable"#);
    txt.push_str("\n");
    txt.push_str(r#"add_executable("#);
    txt.push_str("\n");
    txt.push_str(r#"    ${PROJECT_NAME}"#);
    txt.push_str("\n");
    txt.push_str(r#"    ${HEADER_FILES}"#);
    txt.push_str("\n");
    txt.push_str(r#"    ${SOURCES_FILES}"#);
    txt.push_str("\n");
    txt.push_str(r#")"#);
    txt.push_str("\n\n");

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

    txt
}
