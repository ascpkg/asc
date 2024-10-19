use chrono::Datelike;

use handlebars::Handlebars;

use serde::{Deserialize, Serialize};

use crate::clang;
use crate::util;

const CMAKE_CONFIG_HBS: &str = r#"@PACKAGE_INIT@

include("${CMAKE_CURRENT_LIST_DIR}/{{project}}-targets.cmake")

check_required_components({{project}})
"#;

const VERSION_IN_HBS: &str = r#"
#pragma once

#define {{project_upper}}_VERSION_CODE(major, minor, micro) (((major) * 10000) + ((minor) * 100) + ((micro) * 1))

#define {{project_upper}}_VERSION_MAJOR @{{project_upper}}_VERSION_MAJOR@
#define {{project_upper}}_VERSION_MINOR @{{project_upper}}_VERSION_MINOR@
#define {{project_upper}}_VERSION_MICRO @{{project_upper}}_VERSION_MICRO@

#define {{project_upper}}_VERSION_STRING "@{{project_upper}}_VERSION_MAJOR@.@{{project_upper}}_VERSION_MINOR@.@{{project_upper}}_VERSION_MICRO@"

#define {{project_upper}}_VERSION \
    {{project_upper}}_VERSION_CODE({{project_upper}}_VERSION_MAJOR, {{project_upper}}_VERSION_MINOR, {{project_upper}}_VERSION_MICRO)
"#;

const CMAKE_LISTS_HBS: &str = r#"# set cmake version required
cmake_minimum_required(VERSION {{cmake_version}})

# set project name
project ({{project}} C CXX)

# configure msvc
if(MSVC)
    # utf-8 source code encoding
    add_compile_options("$<$<C_COMPILER_ID:MSVC>:/utf-8>")
    add_compile_options("$<$<CXX_COMPILER_ID:MSVC>:/utf-8>")

    # Visual Studio - Property - C/C++ - General - Debug Information Format - Program Database (/Zi)
    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} /Zi /Od")  # Disable Optimization
    set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /Zi /O1")  # Favor Size

    # Visual Studio - Property - Linker - Debugging - Generate Debug Info - Generate Debug Information (/Debug)
    set(CMAKE_EXE_LINKER_FLAGS_DEBUG "${CMAKE_EXE_LINKER_FLAGS_DEBUG} /DEBUG /OPT:REF /OPT:ICF")
    set(CMAKE_EXE_LINKER_FLAGS_RELEASE "${CMAKE_EXE_LINKER_FLAGS_RELEASE} /DEBUG /OPT:REF /OPT:ICF")

    # Visual Studio - Property - Linker - Debugging - Generate Debug Info - Generate Debug Information (/Debug)
    set(CMAKE_SHARED_LINKER_FLAGS_DEBUG "${CMAKE_SHARED_LINKER_FLAGS_DEBUG} /DEBUG /OPT:REF /OPT:ICF")
    set(CMAKE_SHARED_LINKER_FLAGS_RELEASE "${CMAKE_SHARED_LINKER_FLAGS_RELEASE} /DEBUG /OPT:REF /OPT:ICF")
endif(MSVC)

# version
set({{project_upper}}_VERSION_MAJOR {{build_year}})
set({{project_upper}}_VERSION_MINOR {{build_month}})
set({{project_upper}}_VERSION_MICRO {{build_day}})
set(VERSION_STRING "{{build_year}}.{{build_month}}.{{build_day}}")
configure_file(${CMAKE_SOURCE_DIR}/version.h.in ${CMAKE_BINARY_DIR}/version.h @ONLY)

# package
include(CMakePackageConfigHelpers)
set(PACKAGE ${PROJECT_NAME})
set(PACKAGE_NAME ${PROJECT_NAME})
set(PACKAGE_STRING "${PROJECT_NAME} ${VERSION_STRING}")
set(PACKAGE_TARNAME ${PROJECT_NAME})
set(PACKAGE_URL "")
set(PACKAGE_VERSION ${VERSION_STRING})
set(STDC_HEADERS 1)
set(SUPPORT_ATTRIBUTE_VISIBILITY_DEFAULT 1)
set(SUPPORT_FLAG_VISIBILITY 1)
set(VERSION ${VERSION_STRING})

# glob sources
{{#each sources_group_by_dir as |group|}}
FILE(
    GLOB
    {{group.dir}}
    {{#each group.files as |path|}}
    "{{path}}"
    {{/each}}
)
{{/each}}

# group sources
{{#each sources_group_by_dir as |group|}}
SOURCE_GROUP("{{group.original_dir}}" FILES {{group.variable}})
{{/each}}

{{#if executable}}
# generate executable
add_executable(
{{else}}
# generate library
add_library(
{{/if}}
    ${PROJECT_NAME}
{{#if shared_library}}
    SHARED
{{/if}}
    {{#each sources_group_by_dir as |group|}}
    {{group.variable}}
    {{/each}}
)

# set include dirs
target_include_directories(
    ${PROJECT_NAME}
    PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_CURRENT_BINARY_DIR}
{{#each include_dirs as |dir|}}
    "{{dir}}"
{{/each}}
)

{{#if link_libraries }}
# link libraries
target_link_libraries(
    ${PROJECT_NAME}
    {{#if link_public_libraries}}
    PUBLIC
    {{#each public_libraries as |lib|}}
    {{lib}}
    {{/each}}
    {{/if}}
    {{#if link_private_libraries}}
    PRIVATE
    {{#each private_libraries as |lib|}}
    {{lib}}
    {{/each}}
    {{/if}}
)
{{/if}}

# export dynamic library symbols
if(BUILD_SHARED_LIBS)
    add_definitions(-DBUILD_SHARED_LIBS)
    add_definitions(-D{{project_upper}}_EXPORTS)
endif()

{{#if library}}
# export library
add_library(
    ${PROJECT_NAME}::${PROJECT_NAME}
    ALIAS
    ${PROJECT_NAME}
)

# export include path
target_include_directories(
    ${PROJECT_NAME}
    INTERFACE
    $<INSTALL_INTERFACE:include>
)
{{/if}}

# install archive, library, bin
install(
    TARGETS ${PROJECT_NAME}
    EXPORT ${PROJECT_NAME}
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
)

{{#if library}}
# install headers
{{#each install_headers as |header|}}
install(FILES "{{header.src}}" DESTINATION include/${PROJECT_NAME}/{{header.dst}})
{{/each}}

# install package config
configure_package_config_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/${PROJECT_NAME}-config.cmake.in"
    "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}-config.cmake"
    INSTALL_DESTINATION "share/${PROJECT_NAME}"
)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/${PROJECT_NAME}-config.cmake" DESTINATION "share/${PROJECT_NAME}")

# install cmake targets
install(
    EXPORT ${PROJECT_NAME}-targets
    DESTINATION share/${PROJECT_NAME}
    NAMESPACE ${PROJECT_NAME}::
)
{{/if}}
"#;

pub fn path(options: &util::cli::CommandLines) -> String {
    format!("{}/CMakeLists.txt", &options.project_dir)
}

pub fn version_h_in_path(options: &util::cli::CommandLines) -> String {
    format!("{}/version.h.in", &options.project_dir)
}

pub fn config_path(options: &util::cli::CommandLines) -> String {
    format!(
        "{}/{}-config.cmake.in",
        &options.project_dir, &options.project
    )
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct SourcesGroup {
    dir: String,
    original_dir: String,
    variable: String,
    files: Vec<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct InstallHeader {
    src: String,
    dst: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct CMakeListsData {
    cmake_version: String,
    project: String,
    project_upper: String,
    build_year: i32,
    build_month: u32,
    build_day: u32,
    executable: bool,
    library: bool,
    shared_library: bool,
    sources_group_by_dir: Vec<SourcesGroup>,
    include_dirs: Vec<String>,
    link_libraries: bool,
    link_public_libraries: bool,
    public_libraries: Vec<String>,
    link_private_libraries: bool,
    private_libraries: Vec<String>,
    install_headers: Vec<InstallHeader>,
}

pub fn gen(options: &util::cli::CommandLines, source_mappings: &clang::parser::SourceMappings) {
    // group sources by dir name
    let mut group_sources =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    let mut classify_to_dir = std::collections::HashMap::<String, String>::new();
    let mut install_headers = std::collections::BTreeMap::<String, String>::new();
    for (header, sources) in &source_mappings.header_include_by_sources {
        {
            let header_locate_dir = std::path::Path::new(header)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            // prepare install headers's src and dst
            let src = if header.starts_with(&options.source_dir) {
                util::fs::remove_prefix(header, &options.project_dir, &options.build_dir)
            } else {
                format!(
                    "${{CMAKE_CURRENT_BINARY_DIR}}/{}",
                    util::fs::remove_prefix(header, &options.source_dir, &options.build_dir)
                )
            };

            let dst = if header_locate_dir.starts_with(&options.source_dir) {
                util::fs::remove_prefix(&header_locate_dir, &options.source_dir, &options.build_dir)
            } else {
                String::new()
            };
            install_headers.insert(src, dst);
        }

        {
            // group header
            let relative_path: String =
                util::fs::remove_prefix(header, &options.project_dir, &options.build_dir);
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
                let relative_path: String =
                    util::fs::remove_prefix(src, &options.project_dir, &options.build_dir);
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

    let local_date_time = chrono::prelude::Local::now();
    let mut data = CMakeListsData::default();
    data.cmake_version = options.cmake_minimum_version.clone();
    data.project = options.project.clone();
    data.project_upper = options.project.to_uppercase();
    data.build_year = local_date_time.year();
    data.build_month = local_date_time.month();
    data.build_day = local_date_time.day();
    data.executable = options.cmake_target_type == util::cli::CMakeTargetType::Executable;
    data.library = options.cmake_target_type == util::cli::CMakeTargetType::Library;
    data.shared_library = data.library && options.cmake_lib_type == util::cli::CMakeLibType::Shared;
    data.include_dirs = options.include_dirs.clone();
    data.link_libraries = false;
    data.link_public_libraries = false;
    data.link_private_libraries = false;

    for (dir, sources) in &group_sources {
        let mut group = SourcesGroup::default();
        group.dir = dir.clone();
        group.original_dir = classify_to_dir.get(dir).unwrap().clone();
        group.variable = format!("${}{}{}", "{", dir, "}");
        for src in sources {
            group.files.push(src.clone());
        }
        data.sources_group_by_dir.push(group);
    }

    for (src, dst) in install_headers {
        data.install_headers
            .push(InstallHeader { src: src, dst: dst });
    }

    // write project-config.cmake.in
    {
        let reg = Handlebars::new();
        let text = reg.render_template(&CMAKE_CONFIG_HBS, &data).unwrap();
        std::fs::write(config_path(options), text.as_bytes()).unwrap();
    }

    // write version.h.in
    {
        let reg = Handlebars::new();
        let text = reg.render_template(&VERSION_IN_HBS, &data).unwrap();
        std::fs::write(version_h_in_path(options), text.as_bytes()).unwrap();
    }

    // write CMakeLists.txt
    {
        let reg = Handlebars::new();
        let text = reg.render_template(CMAKE_LISTS_HBS, &data).unwrap();
        std::fs::write(path(&options), text.as_bytes()).unwrap();
    }
}
