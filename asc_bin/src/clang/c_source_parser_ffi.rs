// bindings of c_source_parser_ffi/src/lib.h
#![allow(dead_code)]

pub type RustBtreeMapOfStrSet = *mut std::ffi::c_void;
pub type RustBtreeSetOfStr = *mut std::ffi::c_void;
pub type RustBtreeSetOfStrConst = *const std::ffi::c_void;
pub type RustVecOfStr = *mut std::ffi::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct ClangParsedResult {
    pub error_code: i32,
    // CString::new()::into_raw()
    pub source_path: *const i8,
    // CString::new()::into_raw()
    pub source_dir: *const i8,
    // CString::new()::into_raw()
    pub target_dir: *const i8,
    // Box::into_raw(Box::new(BTreeSet::<String>::new()))
    pub last_parsed_files: RustBtreeSetOfStr,
    // Box::into_raw(Box::new(BTreeSet::<String>::new()))
    pub current_parsed_files: RustBtreeSetOfStr,
    // Box::into_raw(Box::new(BTreeMap::<String, BTreeSet::<String>>::new()))
    pub source_symbols: RustBtreeMapOfStrSet,
    // Box::into_raw(Box::new(BTreeMap::<String, BTreeSet::<String>>::new()))
    pub source_include_headers: RustBtreeMapOfStrSet,
    // Box::into_raw(Box::new(BTreeMap::<String, BTreeSet::<String>>::new()))
    pub header_include_by_sources: RustBtreeMapOfStrSet,
}

extern "C" {
    pub fn load_library_clang(library_clang_path: *const i8) -> AstCErrorCode;

    pub fn scan_source_and_symbols(
        source_path: *const i8,
        source_dir: *const i8,
        target_dir: *const i8,
        last_parsed_files: RustBtreeSetOfStrConst,
    ) -> ClangParsedResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AstCErrorCode {
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
}

impl From<i32> for AstCErrorCode {
    fn from(value: i32) -> Self {
        // find: ([a-zA-Z]*) = (\d*)
        // replace: $2 => AstCErrorCode::$1
        match value {
            // no error
            0 => AstCErrorCode::AstCErrorNone,

            // load library errors
            991 => AstCErrorCode::AstCErrorLibraryClangNotFound,
            992 => AstCErrorCode::AstCErrorLibraryClangVersionMismatch,
            993 => AstCErrorCode::AstCErrorLibraryClangOSMismatch,
            994 => AstCErrorCode::AstCErrorLibraryClangArchMismatch,

            // load symbol errors
            1001 => AstCErrorCode::AstCErrorSymbolClangCreateIndexNotFound,
            1002 => AstCErrorCode::AstCErrorSymbolClangDisposeIndexNotFound,
            1003 => AstCErrorCode::AstCErrorSymbolClangParseTranslationUnitNotFound,
            1004 => AstCErrorCode::AstCErrorSymbolClangDisposeTranslationUnitNotFound,
            1005 => AstCErrorCode::AstCErrorSymbolClangVisitChildrenNotFound,
            1006 => AstCErrorCode::AstCErrorSymbolClangGetTranslationUnitCursorNotFound,
            1007 => AstCErrorCode::AstCErrorSymbolClangGetCursorLocationNotFound,
            1008 => AstCErrorCode::AstCErrorSymbolClangGetFileLocationNotFound,
            1009 => AstCErrorCode::AstCErrorSymbolClangGetCursorKindNotFound,
            1010 => AstCErrorCode::AstCErrorSymbolClangGetIncludedFileNotFound,
            1011 => AstCErrorCode::AstCErrorSymbolClangGetFileNameNotFound,
            1012 => AstCErrorCode::AstCErrorSymbolClangGetCStringNotFound,
            1013 => AstCErrorCode::AstCErrorSymbolClangDisposeStringNotFound,
            1014 => AstCErrorCode::AstCErrorSymbolClangGetCursorSpellingNotFound,
            1015 => AstCErrorCode::AstCErrorSymbolClangCursorGetNumArgumentsNotFound,
            1016 => AstCErrorCode::AstCErrorSymbolClangCursorGetArgumentNotFound,
            1017 => AstCErrorCode::AstCErrorSymbolClangGetCursorTypeNotFound,
            1018 => AstCErrorCode::AstCErrorSymbolClangGetCanonicalTypeNotFound,
            1019 => AstCErrorCode::AstCErrorSymbolClangGetResultTypeNotFound,
            1020 => AstCErrorCode::AstCErrorSymbolClangGetTypeSpellingNotFound,
            1021 => AstCErrorCode::AstCErrorSymbolClangGetCursorSemanticParentNotFound,
            1022 => AstCErrorCode::AstCErrorSymbolClangCursorIsNullNotFound,

            // call function errors
            5001 => AstCErrorCode::AstCErrorSymbolClangCreateIndexCall,
            5002 => AstCErrorCode::AstCErrorSymbolClangDisposeIndexCall,
            5003 => AstCErrorCode::AstCErrorSymbolClangParseTranslationUnitCall,
            5004 => AstCErrorCode::AstCErrorSymbolClangDisposeTranslationUnitCall,
            5005 => AstCErrorCode::AstCErrorSymbolClangVisitChildrenCall,
            5006 => AstCErrorCode::AstCErrorSymbolClangGetTranslationUnitCursorCall,
            5007 => AstCErrorCode::AstCErrorSymbolClangGetCursorLocationCall,
            5008 => AstCErrorCode::AstCErrorSymbolClangGetFileLocationCall,
            5009 => AstCErrorCode::AstCErrorSymbolClangGetCursorKindCall,
            5010 => AstCErrorCode::AstCErrorSymbolClangGetIncludedFileCall,
            5011 => AstCErrorCode::AstCErrorSymbolClangGetFileNameCall,
            5012 => AstCErrorCode::AstCErrorSymbolClangGetCStringCall,
            5013 => AstCErrorCode::AstCErrorSymbolClangDisposeStringCall,
            5014 => AstCErrorCode::AstCErrorSymbolClangGetCursorSpellingCall,
            5015 => AstCErrorCode::AstCErrorSymbolClangCursorGetNumArgumentsCall,
            5016 => AstCErrorCode::AstCErrorSymbolClangCursorGetArgumentCall,
            5017 => AstCErrorCode::AstCErrorSymbolClangGetCursorTypeCall,
            5018 => AstCErrorCode::AstCErrorSymbolClangGetCanonicalTypeCall,
            5019 => AstCErrorCode::AstCErrorSymbolClangGetResultTypeCall,
            5020 => AstCErrorCode::AstCErrorSymbolClangGetTypeSpellingCall,
            5021 => AstCErrorCode::AstCErrorSymbolClangGetCursorSemanticParentCall,
            5022 => AstCErrorCode::AstCErrorSymbolClangCursorIsNullCall,

            _ => AstCErrorCode::AstCErrorUnknown,
        }
    }
}
