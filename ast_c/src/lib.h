#ifndef AST_C_LIB
#define AST_C_LIB

// rust BTreeMap<String, BTreeSet<String>>
typedef void *RustBtreeMapOfStrSet;
// rust BTreeSet<String>
typedef void *RustBtreeSetOfStr;
// rust Vec<String>
typedef void *RustVecOfStr;

enum AstCErrorCode {
    AstCErrorNone,
    AstCErrorUnknown,
    AstCErrorClangParseTranslationUnit,
};

typedef struct {
    const char *source_path;
    const char *source_dir;
    const char *target_dir;
    RustBtreeSetOfStr last_parsed_files;
    RustBtreeSetOfStr current_parsed_files;
    RustBtreeMapOfStrSet source_symbols;
    RustBtreeMapOfStrSet source_include_headers;
    RustBtreeMapOfStrSet header_include_by_sources;
} ClangParsedResult;

#ifdef __cplusplus
extern "C" {
#endif

    int scan_symbols_and_inclusions(ClangParsedResult *result);

#ifdef __cplusplus
}
#endif

#endif
