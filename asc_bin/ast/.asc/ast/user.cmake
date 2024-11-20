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

