#ifndef C_SOURCE_PARSER_FFI_API
#define C_SOURCE_PARSER_FFI_API

#include "rs_container_ffi/btree_map.h"
#include "rs_container_ffi/btree_set.h"
#include "rs_container_ffi/c_str.h"
#include "rs_container_ffi/vec.h"


typedef enum {
    // no error
    AstCErrorNone = 0,

    // load library errors
    AstCErrorLibraryClangNotFound = 991,
    AstCErrorLibraryClangVersionMismatch = 992,
    AstCErrorLibraryClangOSMismatch = 993,
    AstCErrorLibraryClangArchMismatch = 994,
    
    // load symbol errors
    AstCErrorSymbolClangCreateIndexNotFound = 1001,
    AstCErrorSymbolClangDisposeIndexNotFound = 1002,
    AstCErrorSymbolClangParseTranslationUnitNotFound = 1003,
    AstCErrorSymbolClangDisposeTranslationUnitNotFound = 1004,
    AstCErrorSymbolClangVisitChildrenNotFound = 1005,
    AstCErrorSymbolClangGetTranslationUnitCursorNotFound = 1006,
    AstCErrorSymbolClangGetCursorLocationNotFound = 1007,
    AstCErrorSymbolClangGetFileLocationNotFound = 1008,
    AstCErrorSymbolClangGetCursorKindNotFound = 1009,
    AstCErrorSymbolClangGetIncludedFileNotFound = 1010,
    AstCErrorSymbolClangGetFileNameNotFound = 1011,
    AstCErrorSymbolClangGetCStringNotFound = 1012,
    AstCErrorSymbolClangDisposeStringNotFound = 1013,
    AstCErrorSymbolClangGetCursorSpellingNotFound = 1014,
    AstCErrorSymbolClangCursorGetNumArgumentsNotFound = 1015,
    AstCErrorSymbolClangCursorGetArgumentNotFound = 1016,
    AstCErrorSymbolClangGetCursorTypeNotFound = 1017,
    AstCErrorSymbolClangGetCanonicalTypeNotFound = 1018,
    AstCErrorSymbolClangGetResultTypeNotFound = 1019,
    AstCErrorSymbolClangGetTypeSpellingNotFound = 1020,
    AstCErrorSymbolClangGetCursorSemanticParentNotFound = 1021,
    AstCErrorSymbolClangCursorIsNullNotFound = 1022,

    // call function errors
    AstCErrorSymbolClangCreateIndexCall = 5001,
    AstCErrorSymbolClangDisposeIndexCall = 5002,
    AstCErrorSymbolClangParseTranslationUnitCall = 5003,
    AstCErrorSymbolClangDisposeTranslationUnitCall = 5004,
    AstCErrorSymbolClangVisitChildrenCall = 5005,
    AstCErrorSymbolClangGetTranslationUnitCursorCall = 5006,
    AstCErrorSymbolClangGetCursorLocationCall = 5007,
    AstCErrorSymbolClangGetFileLocationCall = 5008,
    AstCErrorSymbolClangGetCursorKindCall = 5009,
    AstCErrorSymbolClangGetIncludedFileCall = 5010,
    AstCErrorSymbolClangGetFileNameCall = 5011,
    AstCErrorSymbolClangGetCStringCall = 5012,
    AstCErrorSymbolClangDisposeStringCall = 5013,
    AstCErrorSymbolClangGetCursorSpellingCall = 5014,
    AstCErrorSymbolClangCursorGetNumArgumentsCall = 5015,
    AstCErrorSymbolClangCursorGetArgumentCall = 5016,
    AstCErrorSymbolClangGetCursorTypeCall = 5017,
    AstCErrorSymbolClangGetCanonicalTypeCall = 5018,
    AstCErrorSymbolClangGetResultTypeCall = 5019,
    AstCErrorSymbolClangGetTypeSpellingCall = 5020,
    AstCErrorSymbolClangGetCursorSemanticParentCall = 5021,
    AstCErrorSymbolClangCursorIsNullCall = 5022,

    // unknown error
    AstCErrorUnknown = 65535,
} AstCErrorCode;


typedef struct {
    AstCErrorCode error_code;
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

    AstCErrorCode load_library_clang(const char *library_clang_path);

    ClangParsedResult scan_source_and_symbols(
        const char *source_path,
        const char *source_dir,
        const char *target_dir,
        const RustBtreeSetOfStr last_parsed_files
    );

#ifdef __cplusplus
}
#endif

#endif  // C_SOURCE_PARSER_FFI_API
