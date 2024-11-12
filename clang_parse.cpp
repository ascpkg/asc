// c
#include <stdlib.h>

// c++
#include <filesystem>
#include <format>
#include <iostream>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <vector>

// clang
#include <clang-c/Index.h>



struct ParsedResult {
	std::string source_dir;
	std::string target_dir;
	std::set<std::string> outer_parsed_files;
	std::set<std::string> parsed_files;
	std::map<std::string, std::set<std::string>> header_include_by_sources;
	std::map<std::string, std::set<std::string>> source_symbols;
};


std::string cx_string_to_string(CXString cx_str) {
	if (cx_str.data == nullptr) {
		return "";
	}

	std::string result = clang_getCString(cx_str);
	clang_disposeString(cx_str);
	return result;
}


std::string get_namespace(CXCursor cursor) {
	std::vector<std::string> namespaces;
	CXCursor parent_cursor = clang_getCursorSemanticParent(cursor);

	while (!clang_Cursor_isNull(parent_cursor)) {
		if (clang_getCursorKind(parent_cursor) == CXCursor_Namespace) {
			namespaces.push_back(cx_string_to_string(clang_getCursorSpelling(parent_cursor)));
		}
		parent_cursor = clang_getCursorSemanticParent(parent_cursor);
	}

	std::reverse(namespaces.begin(), namespaces.end());
	std::string result = "";
	for (const auto &ns : namespaces) {
		if (!result.empty()) {
			result += "::";
		}
		result += ns;
	}
	return result;
}


std::string get_class_name(CXCursor cursor) {
	CXCursor parent_cursor = clang_getCursorSemanticParent(cursor);

	while (clang_getCursorKind(parent_cursor) != CXCursor_ClassDecl) {
		parent_cursor = clang_getCursorSemanticParent(parent_cursor);
	}

	return cx_string_to_string(clang_getCursorSpelling(parent_cursor));
}


std::tuple<std::string, unsigned int, unsigned int> get_location(CXCursor cursor) {
	CXSourceLocation location = clang_getCursorLocation(cursor);

	CXFile cx_file;
	unsigned int line = 0;
	unsigned int column = 0;
	clang_getFileLocation(location, &cx_file, &line, &column, nullptr);

	CXString cx_str_file_name = clang_getFileName(cx_file);
	return std::make_tuple(cx_string_to_string(cx_str_file_name), line, column);
}


std::string remove_prefix(const std::string &path, const std::string &source_dir, const std::string &target_dir) {
	if (path == source_dir || path == target_dir) {
		return "";
	}
	else if (path.find(source_dir) == 0) {
		return path.substr(source_dir.length() + 1);
	}
	else if (path.find(target_dir) == 0) {
		return path.substr(target_dir.length() + 1);
	}
	else {
		return path;
	}
}


std::vector<std::string> find_source_files(const std::string source_dir, const std::string &exclude_path) {
	std::vector<std::string> source_paths;

	if (std::filesystem::exists(source_dir) && std::filesystem::is_directory(source_dir)) {
		std::vector<std::string> extensions = { ".c", ".cc", ".cpp", ".cxx" };
		for (const auto &entry : std::filesystem::recursive_directory_iterator(source_dir)) {
			if (std::filesystem::is_regular_file(entry)) {
				auto file_path = entry.path();
				for (const auto &ext : extensions) {
					if (file_path.extension().string() == ext) {
						std::string path = file_path.string();
						std::replace(path.begin(), path.end(), '\\', '/');
						if (path != exclude_path) {
							source_paths.push_back(path);
						}
					}
				}
			}
		}
	}

	return source_paths;
}


CXChildVisitResult visit_symbols_and_inclusions(
	CXCursor cursor,
	CXCursor parent,
	CXClientData client_data
) {
	ParsedResult *result = static_cast<ParsedResult *>(client_data);

	// get location
	auto [source_path, line, column] = get_location(cursor);
	std::replace(source_path.begin(), source_path.end(), '\\', '/');

	// skip parsed
	if (result->outer_parsed_files.find(source_path) != result->outer_parsed_files.end()) {
		return CXChildVisit_Continue;
	}
	// skip third party files
	if (0 != source_path.find(result->source_dir) && 0 != source_path.find(result->target_dir)) {
		return CXChildVisit_Continue;
	}
	result->parsed_files.insert(source_path);

	// format symbol signature
	std::string symbol_signature;
	CXCursorKind cursor_type = clang_getCursorKind(cursor);
	switch (cursor_type) {
	case CXCursor_InclusionDirective: {
		CXFile include_file = clang_getIncludedFile(cursor);
		if (include_file != nullptr) {
			std::string include_path = cx_string_to_string(clang_getFileName(include_file));
			std::replace(include_path.begin(), include_path.end(), '\\', '/');

			// skip third-party
			if (0 == include_path.find(result->source_dir) || 0 == include_path.find(result->target_dir)) {
				auto iter = result->header_include_by_sources.find(include_path);
				if (iter == result->header_include_by_sources.end()) {
					result->header_include_by_sources[include_path] = std::set<std::string>();
					iter = result->header_include_by_sources.find(include_path);
				}
				iter->second.insert(source_path);
			}
		}
		break;
	}

	case CXCursor_FunctionDecl:
	case CXCursor_CXXMethod:
	case CXCursor_Constructor:
	case CXCursor_Destructor: {
		const char *func_type = (cursor_type == CXCursor_FunctionDecl) ? "function" : "method";

		std::string namespace_ = get_namespace(cursor);

		std::string class_name;
		if (cursor_type != CXCursor_FunctionDecl) {
			class_name = get_class_name(cursor);
		}

		std::string func_name = cx_string_to_string(clang_getCursorSpelling(cursor));
		symbol_signature += std::format(
			"{} {}{}{}{}{}(",
			func_type,
			namespace_,
			namespace_.empty() ? "" : ":",
			class_name,
			class_name.empty() ? "" : ":",
			func_name
		);

		int num_args = clang_Cursor_getNumArguments(cursor);
		for (int i = 0; i < num_args; ++i) {
			CXCursor arg_cursor = clang_Cursor_getArgument(cursor, i);
			CXType arg_type = clang_getCursorType(arg_cursor);
			CXType arg_canonical_type = clang_getCanonicalType(arg_type);

			CXString arg_type_name = (arg_canonical_type.kind == arg_type.kind)
				? clang_getTypeSpelling(arg_type)
				: clang_getTypeSpelling(arg_canonical_type);

			if (i > 0) {
				symbol_signature += ", ";
			}
			symbol_signature += cx_string_to_string(arg_type_name);
		}

		CXType return_type = clang_getResultType(clang_getCursorType(cursor));
		CXString return_type_name = clang_getTypeSpelling(return_type);
		symbol_signature += std::format(") -> {}", cx_string_to_string(return_type_name));
		break;
	}

	case CXCursor_ClassDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		symbol_signature = std::format("class {}", cx_string_to_string(name));
		break;
	}

	case CXCursor_StructDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		symbol_signature = std::format("struct {}", cx_string_to_string(name));
		break;
	}

	case CXCursor_EnumDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		symbol_signature = std::format("enum {}", cx_string_to_string(name));
		break;
	}

	case CXCursor_UnionDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		symbol_signature = std::format("union {}", cx_string_to_string(name));
		break;
	}

	case CXCursor_VarDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		symbol_signature = std::format("var {}", cx_string_to_string(name));
		break;
	}

	case CXCursor_TypedefDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		symbol_signature = std::format("typedef {}", cx_string_to_string(name));
		break;
	}

	default:
		break;
	}

	if (!symbol_signature.empty()) {
		auto iter = result->source_symbols.find(source_path);
		if (iter == result->source_symbols.end()) {
			result->source_symbols[source_path] = std::set<std::string>();
			iter = result->source_symbols.find(source_path);
		}
		iter->second.insert(symbol_signature);
	}

	return CXChildVisit_Recurse;
}


ParsedResult collect_symbols_and_inclusions(const std::string &source, const std::string &source_dir, const std::string &target_dir, const std::set<std::string> &parsed_files) {
	ParsedResult result;
	result.source_dir = source_dir;
	result.target_dir = target_dir;
	result.outer_parsed_files = parsed_files;

	std::vector<const char *> args = {
		"-I", source_dir.c_str(),
		"-I", target_dir.c_str()
	};

	CXIndex index = clang_createIndex(0, 0);
	CXTranslationUnit translation_unit = clang_parseTranslationUnit(
		index,
		source.c_str(),
		args.data(),
		(int)args.size(),
		nullptr,
		0,
		CXTranslationUnit_DetailedPreprocessingRecord
		| CXTranslationUnit_SkipFunctionBodies
		| CXTranslationUnit_KeepGoing
	);
	if (translation_unit == nullptr) {
		clang_disposeIndex(index);
		std::cerr << "clang_parseTranslationUnit error, path: " << source << std::endl;
		return result;
	}

	clang_visitChildren(
		clang_getTranslationUnitCursor(translation_unit),
		visit_symbols_and_inclusions,
		(CXClientData)&result
	);

	clang_disposeTranslationUnit(translation_unit);
	clang_disposeIndex(index);

	return result;
}


class SourceMappings {
public:
	SourceMappings(const std::string &source_dir, const std::string &target_dir)
		: source_dir(source_dir)
		, target_dir(target_dir)
	{
	}

	void scan(std::string entry_point_file) {
		ParsedResult result = collect_symbols_and_inclusions(entry_point_file, this->source_dir, this->target_dir, this->parsed_files);
		this->collect(result);

		auto necessaries = result.header_include_by_sources;

		for (const auto source_path : find_source_files(this->source_dir, entry_point_file)) {
			result = collect_symbols_and_inclusions(source_path, this->source_dir, this->target_dir, this->parsed_files);
			this->collect(result);
		}

		this->clean(necessaries);
	}

	void collect(const ParsedResult &result) {
		this->parsed_files.insert(result.parsed_files.begin(), result.parsed_files.end());

		for (const auto &[header, sources] : result.header_include_by_sources) {
			auto iter = this->header_include_by_sources.find(header);
			if (iter == this->header_include_by_sources.end()) {
				this->header_include_by_sources[header] = std::set<std::string>();
				iter = this->header_include_by_sources.find(header);
			}
			iter->second.insert(sources.begin(), sources.end());
		}

		for (const auto &[source, symbols] : result.source_symbols) {
			auto iter = this->source_symbols.find(source);
			if (iter == this->source_symbols.end()) {
				this->source_symbols[source] = std::set<std::string>();
				iter = this->source_symbols.find(source);
			}
			iter->second.insert(symbols.begin(), symbols.end());
		}
	}

	void clean(std::map<std::string, std::set<std::string>> &necessaries) {
		for (auto &[header, sources] : necessaries) {
			auto iter = this->header_include_by_sources.find(header);
			if (iter != this->header_include_by_sources.end()) {
				auto header_symbols_iter = this->source_symbols.find(header);
				if (header_symbols_iter == this->source_symbols.end()) {
					continue;
				}

				for (const auto &source : iter->second) {
					auto source_symbols_iter = this->source_symbols.find(source);
					if (source_symbols_iter == this->source_symbols.end()) {
						continue;
					}

					std::set<std::string> intersection;
					std::set_intersection(
						header_symbols_iter->second.begin(), header_symbols_iter->second.end(),
						source_symbols_iter->second.begin(), source_symbols_iter->second.end(),
						std::inserter(intersection, intersection.begin())
					);
					if (!intersection.empty()) {
						sources.insert(source);
					}
				}
			}
		}

		std::set<std::string> necessary_sources;
		for (const auto &[header, sources] : necessaries) {
			necessary_sources.insert(header);
			necessary_sources.insert(sources.begin(), sources.end());
		}
		for (auto iter = this->source_symbols.begin(); iter != this->source_symbols.end();) {
			if (necessary_sources.find(iter->first) == necessary_sources.end()) {
				iter = this->source_symbols.erase(iter);
			}
			else {
				iter++;
			}
		}

		this->header_include_by_sources = necessaries;
	}

	void print() {
		print(this->header_include_by_sources, this->source_symbols);
	}

	void print(const std::map<std::string, std::set<std::string>> &header_include_by_sources, const std::map<std::string, std::set<std::string>> &source_symbols) {
		for (auto const header_sources : header_include_by_sources) {
			auto header = remove_prefix(header_sources.first, this->source_dir, this->target_dir);
			for (auto const source : header_sources.second) {
				std::cout << std::format("{}    {}", header, remove_prefix(source, this->source_dir, this->target_dir)) << std::endl;
			}
		}

		std::cout << "=========================================================" << std::endl;

		for (auto const source_symbols : source_symbols) {
			auto source = remove_prefix(source_symbols.first, this->source_dir, this->target_dir);
			for (auto const symbol : source_symbols.second) {
				std::cout << std::format("{}    {}", source, symbol) << std::endl;
			}
			std::cout << "---------------------------------------------------------" << std::endl;
		}
	}
	
private:
	std::string source_dir;
	std::string target_dir;
	std::set<std::string> parsed_files;
	std::map<std::string, std::set<std::string>> header_include_by_sources;
	std::map<std::string, std::set<std::string>> source_symbols;
};


int main(int argc, char **argv) {
	// cl clang_parse.cpp /EHsc /utf-8 /std:c++20 /I "D:/Program Files/LLVM/include" /link /LIBPATH:"D:/Program Files/LLVM/lib" libclang.lib

	std::string cwd = std::filesystem::current_path().string();
	std::replace(cwd.begin(), cwd.end(), '\\', '/');

	std::string source_dir = std::format("{}/src", cwd);
	std::string target_dir = std::format("{}/target/test_package_bin", cwd);

	SourceMappings mappings = SourceMappings(source_dir, target_dir);
	mappings.scan(std::format("{}/main.cpp", source_dir));

	std::cout << "---------------------------------------------------------" << std::endl;
	mappings.print();

	return 0;
}