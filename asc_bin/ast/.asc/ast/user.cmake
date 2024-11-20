# check compiler, os, header, library, symbol

# check <filesystem> header
include(CheckIncludeFileCXX)
include(CheckCXXSourceCompiles)
check_include_file_cxx("filesystem" HAVE_CXX17_FILESYSTEM)
set(HAVE_CXX_FILESYSTEM_LIB FALSE)
if(HAVE_CXX17_FILESYSTEM)
    # check stdc++fs library
    check_cxx_source_compiles(
        "#include <filesystem>
        int main() {
            std::filesystem::path p;
            return 0;
        }"
        HAVE_CXX_FILESYSTEM_LIB
    )
else()
    # check <experimental/filesystem> header
    check_include_file_cxx("experimental/filesystem" HAVE_CXX14_EXPERIMENTAL_FILESYSTEM)
endif()

# check stdc++ library
find_library(HAVE_STD_CXX_LIB stdc++)
