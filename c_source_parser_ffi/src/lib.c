// c
#include <stdio.h>
#include <string.h>

// clang
#include <clang-c/Index.h>

// dylib
#include "dylib.h"

// self
#include "lib.h"


#ifndef TRUE
#define TRUE 1
#endif

#ifndef FALSE
#define FALSE 0
#endif

#ifndef NULL
#define NULL ((void *)0)
#endif

#ifndef IN
#define IN
#endif

#ifndef OUT
#define OUT
#endif

#ifndef IN_OUT
#define IN_OUT
#endif


static void replace_chars(IN_OUT char *str, IN const char old_char, IN const char new_char) {
    if (NULL == str) {
        return;
    }

    while (*str) {
        if (*str == old_char) {
            *str = new_char;
        }
        str++;
    }
}


static int starts_with(IN const char *str, IN const char *sub) {
    if (NULL == str || NULL == sub) {
        return FALSE;
    }

    size_t str_len = strlen(str);
    size_t sub_len = strlen(sub);
    if (sub_len > str_len) {
        return FALSE;
    }

    return (0 == strncmp(str, sub, sub_len)) ? TRUE : FALSE;
}

static char *get_namespaces(IN CXCursor cursor) {
    RustVecOfStr vec = rust_vec_of_str_new();

    CXCursor parent_cursor = clang_getCursorSemanticParent(cursor);
    while (!clang_Cursor_isNull(parent_cursor)) {
        if (clang_getCursorKind(parent_cursor) == CXCursor_Namespace) {
            CXString spelling = clang_getCursorSpelling(parent_cursor);
            rust_vec_of_str_push(vec, clang_getCString(spelling));
            // free clang resources
            clang_disposeString(spelling);
        }

        parent_cursor = clang_getCursorSemanticParent(parent_cursor);
    }

    rust_vec_of_str_reverse(vec);
    char *text = rust_vec_of_str_join(vec, "::");

    // free rust resources
    rust_vec_of_str_drop(vec);

    return text;
}

static char *get_classes(IN CXCursor cursor) {
    RustVecOfStr vec = rust_vec_of_str_new();

    CXCursor parent_cursor = clang_getCursorSemanticParent(cursor);
    while (!clang_Cursor_isNull(parent_cursor)) {
        if (clang_getCursorKind(parent_cursor) == CXCursor_ClassDecl) {
            CXString spelling = clang_getCursorSpelling(parent_cursor);
            rust_vec_of_str_push(vec, clang_getCString(spelling));
            // free clang resources
            clang_disposeString(spelling);
        }

        parent_cursor = clang_getCursorSemanticParent(parent_cursor);
    }

    rust_vec_of_str_reverse(vec);
    char *text = rust_vec_of_str_join(vec, "::");

    // free rust resources
    rust_vec_of_str_drop(vec);

    return text;
}

void store_symbol(IN_OUT RustBtreeMapOfStrSet map_source_symbol, IN const char *source_path, IN const char *type_name, IN CXCursor cursor) {
    RustVecOfStr vec = rust_vec_of_str_new();
    rust_vec_of_str_push(vec, type_name);

    char *namespaces = get_namespaces(cursor);
    if (strlen(namespaces) != 0) {
        rust_vec_of_str_push(vec, namespaces);
        rust_vec_of_str_push(vec, "::");
    }

    char *classes = get_classes(cursor);
    if (strlen(classes) != 0) {
        rust_vec_of_str_push(vec, classes);
        rust_vec_of_str_push(vec, "::");
    }

    CXString spell = clang_getCursorSpelling(cursor);
    const char *name = clang_getCString(spell);
    rust_vec_of_str_push(vec, strlen(name) > 0 ? name : "@UNNAMED@");
    char *symbol = rust_vec_of_str_join(vec, " ");

    rust_btree_map_of_str_set_insert(map_source_symbol, source_path, symbol);

    // free clang resources
    clang_disposeString(spell);
    // free rust resources
    rust_vec_of_str_drop(vec);
    rust_c_str_drop(namespaces);
    rust_c_str_drop(classes);
    rust_c_str_drop(symbol);
}

static enum CXChildVisitResult visit_symbols_and_inclusions(IN CXCursor cursor, IN CXCursor parent, IN_OUT CXClientData client_data) {
    ClangParsedResult *result = (ClangParsedResult *)client_data;

    // get location
    CXSourceLocation location = clang_getCursorLocation(cursor);
    CXFile cx_file = NULL;
    unsigned int line = 0;
    unsigned int column = 0;
    clang_getFileLocation(location, &cx_file, &line, &column, NULL);
    CXString cx_str_source_path = clang_getFileName(cx_file);

    // skip null
    if(NULL == cx_str_source_path.data) {
        return CXChildVisit_Continue;
    }
    char *source_path = (char *)cx_str_source_path.data;
    replace_chars(source_path, '\\', '/');
    // skip parsed
    if (TRUE == rust_btree_set_of_str_contains(result->last_parsed_files, source_path)) {
        return CXChildVisit_Continue;
    }
    // skip third party files
    if (FALSE == starts_with(source_path, result->source_dir) && FALSE == starts_with(source_path, result->target_dir)) {
        return CXChildVisit_Continue;
    }
    rust_btree_set_of_str_insert(result->current_parsed_files, source_path);

    // format symbol signature
    enum CXCursorKind cursor_type = clang_getCursorKind(cursor);
    switch (cursor_type) {
    case CXCursor_InclusionDirective:
    {
        CXFile include_file = clang_getIncludedFile(cursor);
        if (include_file != NULL) {
            CXString cx_str_include_path = clang_getFileName(include_file);
            char *include_path = (char *)clang_getCString(cx_str_include_path);
            replace_chars(include_path, '\\', '/');

            // skip third-party
            if (TRUE == starts_with(include_path, result->source_dir) || TRUE == starts_with(include_path, result->target_dir)) {
                // collect inclusions
                rust_btree_map_of_str_set_insert(result->header_include_by_sources, include_path, source_path);
                rust_btree_map_of_str_set_insert(result->source_include_headers, source_path, include_path);
            }

            // free clang resources
            clang_disposeString(cx_str_include_path);
        }
        break;
    }

    case CXCursor_FunctionDecl:
    case CXCursor_CXXMethod:
    case CXCursor_Constructor:
    case CXCursor_Destructor:
    {
        RustVecOfStr vec = rust_vec_of_str_new();

        const char *func_type = (cursor_type == CXCursor_FunctionDecl) ? "function " : "method ";
        rust_vec_of_str_push(vec, func_type);

        char *namespaces = get_namespaces(cursor);
        if (strlen(namespaces) != 0) {
            rust_vec_of_str_push(vec, namespaces);
            rust_vec_of_str_push(vec, "::");
        }

        char *classes = get_classes(cursor);
        if (strlen(classes) != 0) {
            rust_vec_of_str_push(vec, classes);
            rust_vec_of_str_push(vec, "::");
        }

        CXString cx_str_func_name = clang_getCursorSpelling(cursor);
        const char *func_name = clang_getCString(cx_str_func_name);
        rust_vec_of_str_push(vec, func_name);
        rust_vec_of_str_push(vec, "(");
        // free clang resources
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
                rust_vec_of_str_push(vec, ", ");
            }
            rust_vec_of_str_push(vec, clang_getCString(arg_type_name));
            // free clang resources
            clang_disposeString(arg_type_name);
        }

        CXType return_type = clang_getResultType(clang_getCursorType(cursor));
        CXString return_type_name = clang_getTypeSpelling(return_type);
        rust_vec_of_str_push(vec, ") -> ");
        rust_vec_of_str_push(vec, clang_getCString(return_type_name));

        char *symbol = rust_vec_of_str_join(vec, "");
        rust_btree_map_of_str_set_insert(result->source_symbols, source_path, symbol);

        // free clang resources
        clang_disposeString(return_type_name);

        // free rust resources
        rust_c_str_drop(namespaces);
        rust_c_str_drop(classes);
        rust_vec_of_str_drop(vec);
        rust_c_str_drop(symbol);

        break;
    }

    case CXCursor_ClassDecl:
    {
        store_symbol(result->source_symbols, source_path, "class", cursor);
        break;
    }

    case CXCursor_StructDecl:
    {
        store_symbol(result->source_symbols, source_path, "struct", cursor);
        break;
    }

    case CXCursor_EnumDecl:
    {
        store_symbol(result->source_symbols, source_path, "enum", cursor);
        break;
    }

    case CXCursor_UnionDecl:
    {
        store_symbol(result->source_symbols, source_path, "union", cursor);
        break;
    }

    case CXCursor_VarDecl:
    {
        store_symbol(result->source_symbols, source_path, "var", cursor);
        break;
    }

    case CXCursor_TypedefDecl:
    {
        store_symbol(result->source_symbols, source_path, "typedef", cursor);
        break;
    }

    default:
        break;
    }

    clang_disposeString(cx_str_source_path);

    return CXChildVisit_Recurse;
}


ClangParsedResult scan_source_and_symbols(
    IN const char *library_clang_path,
    IN const char *source_path,
    IN const char *source_dir,
    IN const char *target_dir,
    IN const RustBtreeSetOfStr last_parsed_files
) {
    ClangParsedResult result;

    dylib_handle clang_lib = dylib_open(library_clang_path);
    if(INVALID_DYLIB_HANDLE == clang_lib) {
        printf(stderr, "@%s#%d dylib_open(%s) -> %s\n", __FUNCTION__, __LINE__ - 2, library_clang_path, dylib_error());
        result.error_code = AstCErrorLibraryClangNotFound;
        return result;
    }

    result.error_code = AstCErrorNone;
    result.source_path = source_path;
    result.source_dir = source_dir;
    result.target_dir = target_dir;
    result.last_parsed_files = last_parsed_files;
    result.current_parsed_files = rust_btree_set_of_str_new();
    result.source_symbols = rust_btree_map_of_str_set_new();
    result.source_include_headers = rust_btree_map_of_str_set_new();
    result.header_include_by_sources = rust_btree_map_of_str_set_new();

    const char *args[4] = {
        "-I",
        source_dir,
        "-I",
        target_dir,
    };

    CXIndex index = clang_createIndex(0, 0);
    CXTranslationUnit translation_unit = clang_parseTranslationUnit(
        index,
        source_path,
        args,
        4,
        NULL,
        0,
        CXTranslationUnit_DetailedPreprocessingRecord | CXTranslationUnit_SkipFunctionBodies | CXTranslationUnit_KeepGoing
    );
    if (NULL == translation_unit) {
        clang_disposeIndex(index);
        result.error_code = AstCErrorClangParseTranslationUnit;
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
