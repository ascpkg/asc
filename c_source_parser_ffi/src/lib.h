#ifndef C_SOURCE_PARSER_FFI_API
#define C_SOURCE_PARSER_FFI_API

#include "rs_container_ffi/btree_map.h"
#include "rs_container_ffi/btree_set.h"
#include "rs_container_ffi/c_str.h"
#include "rs_container_ffi/vec.h"

enum AstCErrorCode {
    AstCErrorNone,
    AstCErrorUnknown,
    AstCErrorLibraryClangNotFound,
    AstCErrorClangParseTranslationUnit,
};

typedef struct {
    enum AstCErrorCode error_code;
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

    ClangParsedResult scan_source_and_symbols(
        const char *library_clang_path,
        const char *source_path,
        const char *source_dir,
        const char *target_dir,
        const RustBtreeSetOfStr last_parsed_files
    );

#ifdef __cplusplus
}
#endif

#endif  // C_SOURCE_PARSER_FFI_API
