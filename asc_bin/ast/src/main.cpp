// c++
#include <filesystem>
#include <iostream>
#include <string>

// lib
#include "lib.h"

// config
#include "config.h"

// version
#include "version.h"



int main(int argc, char **argv) {
	std::string cwd = std::filesystem::current_path().string();
	std::replace(cwd.begin(), cwd.end(), '\\', '/');

	std::string source_dir = cwd + "/src";
	std::string target_dir = cwd + "/target/test_package_bin";
	std::string entry_point_file = source_dir + "/main.cpp";

	std::vector<char> buf(64 * 1024, 0);
	uint64_t len = scan_necessary_sources(entry_point_file.c_str(), source_dir.c_str(), target_dir.c_str(), buf.data(), static_cast<int>(buf.size()));
	std::cout << buf.data();

	return 0;
}
