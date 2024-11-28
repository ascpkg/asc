// bindings of asc_c/src/lib.h
#![allow(dead_code)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

pub type RustBtreeMapOfStrSet = *mut std::ffi::c_void;
pub type RustBtreeSetOfStr = *mut std::ffi::c_void;
pub type RustVecOfStr = *mut std::ffi::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct ClangParsedResult {
    pub source_path: *const i8,
    pub source_dir: *const i8,
    pub target_dir: *const i8,
    pub last_parsed_files: RustBtreeSetOfStr,
    pub current_parsed_files: RustBtreeSetOfStr,
    pub source_symbols: RustBtreeMapOfStrSet,
    pub source_include_headers: RustBtreeMapOfStrSet,
    pub header_include_by_sources: RustBtreeMapOfStrSet,
}

extern "C" {
    pub fn scan_symbols_and_inclusions(result: *mut ClangParsedResult) -> i32;
}
