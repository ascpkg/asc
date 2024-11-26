# set cmake version required
cmake_minimum_required(VERSION 3.20)

# set project name
project (ast C CXX)

# set c standard version
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

# set c++ standard version
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# configure msvc
if(MSVC)
    # utf-8 source code encoding
    add_compile_options("$<$<C_COMPILER_ID:MSVC>:/utf-8>")
    add_compile_options("$<$<CXX_COMPILER_ID:MSVC>:/utf-8>")

    # Visual Studio - Property - C/C++ - General - Debug Information Format - Program Database (/Zi)
    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} /Zi /Od")  # Disable Optimization
    set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /Zi /O1")  # Favor Size

    # Visual Studio - Property - Linker - Debugging - Generate Debug Info - Generate Debug Information (/Debug)
    set(CMAKE_EXE_LINKER_FLAGS_DEBUG "${CMAKE_EXE_LINKER_FLAGS_DEBUG} /DEBUG")
    set(CMAKE_EXE_LINKER_FLAGS_RELEASE "${CMAKE_EXE_LINKER_FLAGS_RELEASE} /DEBUG")

    # Visual Studio - Property - Linker - Debugging - Generate Debug Info - Generate Debug Information (/Debug)
    set(CMAKE_SHARED_LINKER_FLAGS_DEBUG "${CMAKE_SHARED_LINKER_FLAGS_DEBUG} /DEBUG")
    set(CMAKE_SHARED_LINKER_FLAGS_RELEASE "${CMAKE_SHARED_LINKER_FLAGS_RELEASE} /DEBUG")
endif(MSVC)

# version
set(AST_VERSION_MAJOR 2024)
set(AST_VERSION_MINOR 11)
set(AST_VERSION_MICRO 26)
set(VERSION_STRING "2024.11.26")
configure_file(${CMAKE_SOURCE_DIR}/ast/version.h.in ${CMAKE_BINARY_DIR}/ast/version.h @ONLY)

# check compiler, os, header, library, symbol

# check header
include(CheckIncludeFileCXX)
# check <filesystem> header
check_include_file_cxx("filesystem" HAVE_STD_CXX_FILESYSTEM)
# check <experimental/filesystem> header
check_include_file_cxx("experimental/filesystem" HAVE_STD_CXX_EXPERIMENTAL_FILESYSTEM)

# check library
include(CheckLibraryExists)
include(CheckCXXSourceCompiles)
# check c++ library
check_library_exists(c++ __cxa_throw "" HAVE_CXX_LIBRARY)
# check stdc++ library
check_library_exists(stdc++ __cxa_throw "" HAVE_STD_CXX_LIBRARY)
# check stdc++fs library
check_cxx_source_compiles(
    "#include <filesystem>
    int main() {
        std::filesystem::path p;
        return 0;
    }"
    HAVE_STD_CXX_FS_LIBRARY
)


configure_file(${CMAKE_SOURCE_DIR}/ast/config.h.cm ${CMAKE_BINARY_DIR}/ast/config.h)

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
FILE(
    GLOB
    src
    "../../src/lib.cpp"
    "../../src/lib.h"
)
FILE(
    GLOB
    target_ast
    "../../target/ast/config.h"
)

# group sources
SOURCE_GROUP("src" FILES ${src})
SOURCE_GROUP("target/ast" FILES ${target_ast})

# generate library
add_library(
    ${PROJECT_NAME}
    ${src}
    ${target_ast}
)

if(DEFINED VCPKG_TARGET_TRIPLET)
    if(VCPKG_TARGET_TRIPLET STREQUAL "x64-windows-static")
        if(MSVC)
            set_target_properties(
                ${PROJECT_NAME}
                PROPERTIES
                MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>"
            )
        endif()
    endif()
endif()

# find package
find_package(fmt CONFIG REQUIRED)
find_package(Clang CONFIG REQUIRED)

# set include dirs
target_include_directories(
    ${PROJECT_NAME}
    PRIVATE
    ${CMAKE_SOURCE_DIR}/../src
    ${CMAKE_CURRENT_BINARY_DIR}
    ${CLANG_INCLUDE_DIRS}
)

# link libraries
set(LINK_LIBS "")
list(APPEND LINK_LIBS fmt::fmt)
list(APPEND LINK_LIBS libclang)
if(HAVE_CXX_LIBRARY)
list(APPEND LINK_LIBS c++)
endif()
if(HAVE_STD_CXX_LIBRARY)
list(APPEND LINK_LIBS stdc++)
endif()
if(NOT HAVE_STD_CXX_FS_LIBRARY)
list(APPEND LINK_LIBS stdc++fs)
endif()
target_link_libraries(
    ${PROJECT_NAME}
    PRIVATE
    ${LINK_LIBS}
)

# export dynamic library symbols
if(BUILD_SHARED_LIBS)
    add_definitions(-DBUILD_SHARED_LIBS)
    add_definitions(-DAST_EXPORTS)
endif()

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

# install archive, library, bin
install(
    TARGETS ${PROJECT_NAME}
    EXPORT ${PROJECT_NAME}
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
)

# install headers
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/config.h" DESTINATION include/${PROJECT_NAME}/)
install(FILES "../../src/lib.h" DESTINATION include/${PROJECT_NAME}/)

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
