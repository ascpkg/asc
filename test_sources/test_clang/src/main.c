#include <memory.h>
#include <stdio.h>
#include <string.h>

#include <clang-c/Index.h>

#include "config.h"
#include "version.h"

#define SYMBOL_SYGNATURE_BUF_SIZE 1024

typedef struct {
	const char *source_path;
	const char *source_dir;
	const char *target_dir;
} ParsedResult;

static void replace_chars(char *str, char old_char, char new_char) {
	if (0 == str) {
		return;
	}

	while (*str) {
		if (*str == old_char) {
			*str = new_char;
		}
		str++;
	}
}

static int starts_with(const char *str, const char *sub) {
	if (0 == str || 0 == sub) {
		return 0;
	}

	size_t str_len = strlen(str);
	size_t sub_len = strlen(sub);
	if (sub_len > str_len) {
		return 0;
	}

	return 0 == strncmp(str, sub, sub_len);
}

static char *get_namespaces(CXCursor cursor) {
	char **namespaces = NULL;
	size_t namespaces_count = 0;

	CXCursor parent_cursor = clang_getCursorSemanticParent(cursor);
	while (!clang_Cursor_isNull(parent_cursor)) {
		if (clang_getCursorKind(parent_cursor) == CXCursor_Namespace) {
			CXString spelling = clang_getCursorSpelling(parent_cursor);
			const char *namespace_name = clang_getCString(spelling);

			void *addr = realloc(namespaces, sizeof(char *) * (namespaces_count + 1));
			if (0 == addr) {
				free(namespaces);
				namespaces = (char **)addr;
				fprintf(stderr, "func: %s, line: %d, error: realloc namespaces error\n", __FUNCTION__, __LINE__);
			}
			else {
				namespaces = (char **)addr;
				size_t n = strlen(namespace_name);
				void *a = calloc(sizeof(char) * (n + 1));
				if (a != 0) {
					namespaces[namespaces_count] = (char *)a;
					snprintf(namespaces[namespaces_count], n + 1, "%s\0", namespace_name);
				}
				else {
					fprintf(stderr, "func: %s, line: %d, error: malloc namespaces error\n", __FUNCTION__, __LINE__);
				}
				namespaces_count++;
			}

			clang_disposeString(spelling);

			if (0 == namespaces) {
				return strdup("");
			}
		}

		parent_cursor = clang_getCursorSemanticParent(parent_cursor);
	}

	if (0 == namespaces_count) {
		return strdup("");
	}

	size_t total_length = 0;
	for (size_t i = 0; i < namespaces_count; i++) {
		total_length += strlen(namespaces[i]) + (i > 0 ? 2 : 0);
	}
	char *result = (char *)calloc(total_length + 1);
	if (0 == result) {
		for (size_t i = 0; i < namespaces_count; i++) {
			free(namespaces[i]);
		}
	}
	else {
		result[0] = '\0';

		for (size_t i = 0; i < namespaces_count; i++) {
			if (i > 0) {
				strcat(result, "::");
			}
			strcat(result, namespaces[i]);
			free(namespaces[i]);
		}
	}

	free(namespaces);

	return result;
}

static char *get_classes(CXCursor cursor) {
	char **classes = NULL;
	size_t classes_count = 0;

	CXCursor parent_cursor = clang_getCursorSemanticParent(cursor);
	while (!clang_Cursor_isNull(parent_cursor)) {
		if (clang_getCursorKind(parent_cursor) == CXCursor_ClassDecl) {
			CXString spelling = clang_getCursorSpelling(parent_cursor);
			const char *class_name = clang_getCString(spelling);

			void *addr = realloc(classes, sizeof(char *) * (classes_count + 1));
			if (0 == addr) {
				free(classes);
				classes = (char **)addr;
				fprintf(stderr, "func: %s, line: %d, error: realloc classes error\n", __FUNCTION__, __LINE__);
			}
			else {
				classes = (char **)addr;
				size_t n = strlen(class_name);
				void *a = calloc(sizeof(char) * (n + 1));
				if (a != 0) {
					classes[classes_count] = (char *)a;
					snprintf(classes[classes_count], n + 1, "%s\0", class_name);
				}
				else {
					fprintf(stderr, "func: %s, line: %d, error: malloc namespaces error\n", __FUNCTION__, __LINE__);
				}
				classes_count++;
			}

			clang_disposeString(spelling);

			if (0 == classes) {
				return strdup("");
			}
		}

		parent_cursor = clang_getCursorSemanticParent(parent_cursor);
	}

	if (0 == classes_count) {
		return strdup("");
	}

	size_t total_length = 0;
	for (size_t i = 0; i < classes_count; i++) {
		total_length += strlen(classes[i]) + (i > 0 ? 2 : 0);
	}
	char *result = (char *)calloc(total_length + 1);
	if (0 == result) {
		for (size_t i = 0; i < classes_count; i++) {
			free(classes[i]);
		}
	}
	else {
		result[0] = '\0';

		for (size_t i = 0; i < classes_count; i++) {
			if (i > 0) {
				strcat(result, "::");
			}
			strcat(result, classes[i]);
			free(classes[i]);
		}
	}

	free(classes);

	return result;
}

static enum CXChildVisitResult visit_symbols_and_inclusions(CXCursor cursor, CXCursor parent, CXClientData client_data) {
	ParsedResult *result = (ParsedResult *)(client_data);

	// get location
	CXSourceLocation location = clang_getCursorLocation(cursor);
	CXFile cx_file = 0;
	unsigned int line = 0;
	unsigned int column = 0;
	clang_getFileLocation(location, &cx_file, &line, &column, 0);
	CXString cx_str_source_path = clang_getFileName(cx_file);
	char *source_path = (char *)cx_str_source_path.data;
	replace_chars(source_path, '\\', '/');

	// skip parsed

	// skip third party files
	if (0 == starts_with(source_path, result->source_dir) && 0 == starts_with(source_path, result->target_dir)) {
		return CXChildVisit_Continue;
	}

	// format symbol signature
	char symbol_signature[SYMBOL_SYGNATURE_BUF_SIZE];
	memset(symbol_signature, 0, sizeof(char) * SYMBOL_SYGNATURE_BUF_SIZE);
	enum CXCursorKind cursor_type = clang_getCursorKind(cursor);
	switch (cursor_type) {
	case CXCursor_InclusionDirective: {
		symbol_signature[0] = '\0';

		CXFile include_file = clang_getIncludedFile(cursor);
		if (include_file != 0) {
			CXString cx_str_include_path = clang_getFileName(include_file);
			char *include_path = (char *)clang_getCString(cx_str_include_path);
			replace_chars(include_path, '\\', '/');

			// skip third-party
			if (0 == starts_with(include_path, result->source_dir) || 0 == starts_with(include_path, result->target_dir)) {
				// collect inclusions
				printf("%s\t\t%s\n", include_path, source_path);
			}

			clang_disposeString(cx_str_include_path);
		}
		break;
	}

	case CXCursor_FunctionDecl:
	case CXCursor_CXXMethod:
	case CXCursor_Constructor:
	case CXCursor_Destructor: {
		const char *func_type = (cursor_type == CXCursor_FunctionDecl) ? "function" : "method";
		snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "%s", func_type);

		const char *namespace_names = get_namespaces(cursor);
		if (namespace_names != 0) {
			snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "%s %s", symbol_signature, namespace_names);
		}

		const char *class_names = get_classes(cursor);
		if (class_names != 0) {
			if (namespace_names != 0) {
				snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "%s::", symbol_signature);
			}
			snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "%s%s", symbol_signature, class_names);
		}

		CXString cx_str_func_name = clang_getCursorSpelling(cursor);
		const char *func_name = clang_getCString(cx_str_func_name);
		if (namespace_names != 0 || class_names != 0) {
			snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "%s::", symbol_signature);
		}
		snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "%s%s(", symbol_signature, func_name);
		clang_disposeString(cx_str_func_name);

		int num_args = clang_Cursor_getNumArguments(cursor);
		for (int i = 0; i < num_args; ++i) {
			CXCursor arg_cursor = clang_Cursor_getArgument(cursor, i);
			CXType arg_type = clang_getCursorType(arg_cursor);
			CXType arg_canonical_type = clang_getCanonicalType(arg_type);

			CXString arg_type_name = (arg_canonical_type.kind == arg_type.kind)
				? clang_getTypeSpelling(arg_type)
				: clang_getTypeSpelling(arg_canonical_type);

			if (i > 0) {
				snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "%s, ", symbol_signature);
			}
			snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "%s%s", symbol_signature, clang_getCString(arg_type_name));

			clang_disposeString(arg_type_name);
		}

		CXType return_type = clang_getResultType(clang_getCursorType(cursor));
		CXString return_type_name = clang_getTypeSpelling(return_type);
		snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "%s) -> %s\0", symbol_signature, clang_getCString(return_type_name));
		clang_disposeString(return_type_name);

		break;
	}

	case CXCursor_ClassDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "class %s\0", clang_getCString(name));
		clang_disposeString(name);
		break;
	}

	case CXCursor_StructDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "struct %s\0", clang_getCString(name));
		clang_disposeString(name);
		break;
	}

	case CXCursor_EnumDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "enum %s\0", clang_getCString(name));
		clang_disposeString(name);
		break;
	}

	case CXCursor_UnionDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "union %s\0", clang_getCString(name));
		clang_disposeString(name);
		break;
	}

	case CXCursor_VarDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "var %s\0", clang_getCString(name));
		clang_disposeString(name);
		break;
	}

	case CXCursor_TypedefDecl: {
		CXString name = clang_getCursorSpelling(cursor);
		snprintf(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE, "typedef %s\0", clang_getCString(name));
		clang_disposeString(name);
		break;
	}

	default:
		symbol_signature[0] = '\0';
		break;
	}

	if (strnlen(symbol_signature, SYMBOL_SYGNATURE_BUF_SIZE) > 0) {
		// collect symbols
		printf("%s\t\t%s", source_path, symbol_signature);
	}

	clang_disposeString(cx_str_source_path);

	return CXChildVisit_Recurse;
}


int scan_symbols_and_inclusions(const char *source_path, const char *source_dir, const char *target_dir) {
	ParsedResult result;
	result.source_dir = source_dir;
	result.target_dir = target_dir;

	const char *args[4] = {
		"-I", source_dir,
		"-I", target_dir,
	};

	CXIndex index = clang_createIndex(0, 0);
	CXTranslationUnit translation_unit = clang_parseTranslationUnit(
		index,
		source_path,
		args,
		4,
		0,
		0,
		CXTranslationUnit_DetailedPreprocessingRecord
		| CXTranslationUnit_SkipFunctionBodies
		| CXTranslationUnit_KeepGoing
	);
	if (translation_unit == 0) {
		clang_disposeIndex(index);
		printf("clang_parseTranslationUnit error, source_path: %s\n", source_path);
		return 1;
	}

	clang_visitChildren(
		clang_getTranslationUnitCursor(translation_unit),
		visit_symbols_and_inclusions,
		(CXClientData)&result
	);

	clang_disposeTranslationUnit(translation_unit);
	clang_disposeIndex(index);

	return 0;
}


int main(int argc, char **argv) {
	scan_symbols_and_inclusions(
		"D:/__develop__/FutureOrientedGB/asc/test_sources/test_package/src/main.cpp",
		"D:/__develop__/FutureOrientedGB/asc/test_sources/test_package/src",
		"D:/__develop__/FutureOrientedGB/asc/test_sources/test_package/target/test_package_bin"
	);
	return 0;
}
