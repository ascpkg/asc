pub static CONFIG_IN_CM_HBS: &str = r#"/* config compiler, os, header, library, symbol */

/* Define to 1 if you have the <stdlib.h> header file. */
#cmakedefine HAVE_STDLIB_H 1

/* Define to 1 if you have the `gettimeofday' function. */
#cmakedefine HAVE_GETTIMEOFDAY 1

/* Define to 1 if you have the `O_BINARY' symbol. */
#cmakedefine HAVE_O_BINARY 1
"#;

pub static USER_CMAKE_HBS: &str = r#"# check compiler, os, header, library, symbol
# check headers
include(CheckIncludeFiles)
check_include_files("stdlib.h" HAVE_STDLIB_H)

# check types
include(CheckTypeSize)
# check_type_size(size_t SIZE_T)

# check functions
include(CheckFunctionExists)
check_function_exists(gettimeofday HAVE_GETTIMEOFDAY)

# check symbols
include(CheckSymbolExists)
check_symbol_exists(O_BINARY "fcntl.h" HAVE_O_BINARY)
"#;

pub static CMAKE_CONFIG_HBS: &str = r#"@PACKAGE_INIT@

include("${CMAKE_CURRENT_LIST_DIR}/{{project}}-targets.cmake")

check_required_components({{project}})
"#;

pub static VERSION_IN_HBS: &str = r#"
#pragma once

#define {{project_upper}}_VERSION_CODE(major, minor, micro) (((major) * 10000) + ((minor) * 100) + ((micro) * 1))

#define {{project_upper}}_VERSION_MAJOR @{{project_upper}}_VERSION_MAJOR@
#define {{project_upper}}_VERSION_MINOR @{{project_upper}}_VERSION_MINOR@
#define {{project_upper}}_VERSION_MICRO @{{project_upper}}_VERSION_MICRO@

#define {{project_upper}}_VERSION_STRING "@{{project_upper}}_VERSION_MAJOR@.@{{project_upper}}_VERSION_MINOR@.@{{project_upper}}_VERSION_MICRO@"

#define {{project_upper}}_VERSION {{project_upper}}_VERSION_CODE({{project_upper}}_VERSION_MAJOR, {{project_upper}}_VERSION_MINOR, {{project_upper}}_VERSION_MICRO)
"#;

pub static CMKAE_WORKSPACE_HBS: &str = r#"
# set cmake version required
cmake_minimum_required(VERSION {{cmake_version}})

# set project name
project ({{project}} C CXX)

# add workspace members
{{#each members as |member|}}
add_subdirectory({{member}})
{{/each}}
"#;

pub static CMAKE_LISTS_HBS: &str = r#"# set cmake version required
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
{{#if is_workspace}}
configure_file(${CMAKE_SOURCE_DIR}/{{project}}/version.h.in ${CMAKE_BINARY_DIR}/{{project}}/version.h @ONLY)
{{else}}
configure_file(${CMAKE_SOURCE_DIR}/version.h.in ${CMAKE_BINARY_DIR}/version.h @ONLY)
{{/if}}

{{{check_cmake_txt}}}
{{#if is_workspace}}
configure_file(${CMAKE_SOURCE_DIR}/{{project}}/config.h.cm ${CMAKE_BINARY_DIR}/{{project}}/config.h)
{{else}}
configure_file(${CMAKE_SOURCE_DIR}/config.h.cm ${CMAKE_BINARY_DIR}/config.h)
{{/if}}

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
