// c
#include <stdio.h>
#include <string.h>

// clang
#include <clang-c/Index.h>

// self
#include "lib.h"

///////////////////////////////////////////////////////////////////////////////////////////////////
// bindings of asc_r/src/lib.rs

// rust BTreeMap<String, BTreeSet<String>>
extern RustBtreeMapOfStrSet rust_btree_map_of_str_set_new();
extern void rust_btree_map_of_str_set_drop(RustBtreeMapOfStrSet instance);
extern void rust_btree_map_of_str_set_insert(RustBtreeMapOfStrSet instance, const char *key, const char *value);

// rust BTreeSet<String>
extern RustBtreeSetOfStr rust_btree_set_of_str_new();
extern void rust_btree_set_of_str_drop(RustBtreeSetOfStr instance);
extern int rust_btree_set_of_str_contains(RustBtreeSetOfStr instance, const char *value);
extern void rust_btree_set_of_str_insert(RustBtreeSetOfStr instance, const char *value);

// rust Vec<String>
extern RustVecOfStr rust_vec_of_str_new();
extern void rust_vec_of_str_drop(RustVecOfStr vec);
extern void rust_vec_of_str_push(RustVecOfStr vec, const char *value);
extern void rust_vec_of_str_reverse(RustVecOfStr vec);
extern char *rust_vec_of_str_join(RustVecOfStr vec, const char *sep);

// rust CString
extern void rust_c_str_drop(char *s);
///////////////////////////////////////////////////////////////////////////////////////////////////

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

static char *get_classes(CXCursor cursor) {
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

void store_symbol(RustBtreeMapOfStrSet map, const char *source_path, char *type_name, CXCursor cursor) {
    CXString spell = clang_getCursorSpelling(cursor);

    RustVecOfStr vec = rust_vec_of_str_new();
    rust_vec_of_str_push(vec, type_name);
    rust_vec_of_str_push(vec, clang_getCString(spell));
    char *text = rust_vec_of_str_join(vec, " ");

    rust_btree_map_of_str_set_insert(map, source_path, text);

    // free clang resources
    clang_disposeString(spell);
    // free resut resources
    rust_vec_of_str_drop(vec);
    rust_c_str_drop(text);
}

static enum CXChildVisitResult visit_symbols_and_inclusions(CXCursor cursor, CXCursor parent, CXClientData client_data) {
    ClangParsedResult *result = (ClangParsedResult *)(client_data);

    // get location
    CXSourceLocation location = clang_getCursorLocation(cursor);
    CXFile cx_file = 0;
    unsigned int line = 0;
    unsigned int column = 0;
    clang_getFileLocation(location, &cx_file, &line, &column, 0);
    CXString cx_str_source_path = clang_getFileName(cx_file);
    if(0 == cx_str_source_path.data) {
        return CXChildVisit_Continue;
    }
    char *source_path = (char *)cx_str_source_path.data;
    replace_chars(source_path, '\\', '/');

    // skip parsed
    if (1 == rust_btree_set_of_str_contains(result->last_parsed_files, source_path)) {
        return CXChildVisit_Continue;
    }
    // skip third party files
    if (0 == starts_with(source_path, result->source_dir) && 0 == starts_with(source_path, result->target_dir)) {
        return CXChildVisit_Continue;
    }
    rust_btree_set_of_str_insert(result->current_parsed_files, source_path);

    

    printf("-------------- %s\n", result->source_dir);
    printf("-------------- %s\n", result->target_dir);
    printf("-------------- %s\n", result->source_path);

    // format symbol signature
    enum CXCursorKind cursor_type = clang_getCursorKind(cursor);
    switch (cursor_type) {
    case CXCursor_InclusionDirective:
    {
        CXFile include_file = clang_getIncludedFile(cursor);
        if (include_file != 0) {
            CXString cx_str_include_path = clang_getFileName(include_file);
            char *include_path = (char *)clang_getCString(cx_str_include_path);
            replace_chars(include_path, '\\', '/');

            // skip third-party
            if (0 == starts_with(include_path, result->source_dir) || 0 == starts_with(include_path, result->target_dir)) {
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

        char *namespace_names = get_namespaces(cursor);
        if (namespace_names != 0) {
            rust_vec_of_str_push(vec, namespace_names);
        }

        char *class_names = get_classes(cursor);
        if (class_names != 0) {
            if (namespace_names != 0) {
                rust_vec_of_str_push(vec, "::");
            }
            rust_vec_of_str_push(vec, class_names);
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
        rust_c_str_drop(namespace_names);
        rust_c_str_drop(class_names);
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

int scan_symbols_and_inclusions(ClangParsedResult *result) {
    const char *args[4] = {
        "-I",
        result->source_dir,
        "-I",
        result->target_dir,
    };

    printf("-------------- %s\n", result->source_dir);
    printf("-------------- %s\n", result->target_dir);
    printf("-------------- %s\n", result->source_path);

    CXIndex index = clang_createIndex(0, 0);
    CXTranslationUnit translation_unit = clang_parseTranslationUnit(
        index,
        result->source_path,
        args,
        4,
        0,
        0,
        CXTranslationUnit_DetailedPreprocessingRecord | CXTranslationUnit_SkipFunctionBodies | CXTranslationUnit_KeepGoing
    );
    if (0 == translation_unit) {
        clang_disposeIndex(index);
        return AstCErrorClangParseTranslationUnit;
    }

    clang_visitChildren(
        clang_getTranslationUnitCursor(translation_unit),
        visit_symbols_and_inclusions,
        (CXClientData)&result
    );

    printf("-------------- %s\n", result->source_dir);
    printf("-------------- %s\n", result->target_dir);
    printf("-------------- %s\n", result->source_path);

    clang_disposeTranslationUnit(translation_unit);
    clang_disposeIndex(index);

    return AstCErrorNone;
}
