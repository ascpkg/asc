// c++
#include <filesystem>
#include <format>
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

	std::string source_dir = std::format("{}/src", cwd);
	std::string target_dir = std::format("{}/target/test_package_bin", cwd);
	std::string entry_point_file = std::format("{}/main.cpp", source_dir);

	std::vector<char> buf(64 * 1024, 0);
	uint64_t len = scan_necessary_sources(entry_point_file.c_str(), source_dir.c_str(), target_dir.c_str(), buf.data(), buf.size());
	std::cout << buf.data();

	return 0;
}
