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
    AstCErrorNone = 0,
    AstCErrorUnknown = 2,
    AstCErrorClangParseTranslationUnit = 3,
}

impl From<i32> for AstCErrorCode {
    fn from(value: i32) -> Self {
        match value {
            0 => AstCErrorCode::AstCErrorNone,
            1 => AstCErrorCode::AstCErrorClangParseTranslationUnit,
            _ => AstCErrorCode::AstCErrorUnknown,
        }
    }
}
