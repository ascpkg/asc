mod ast;

use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::CString,
};

fn main() {
    let source_path = "D:/__develop__/FutureOrientedGB/asc/test_sources/test_package/src/main.cpp";
    let c_str_source_path = CString::new(source_path).unwrap().into_raw();

    let source_dir = "D:/__develop__/FutureOrientedGB/asc/test_sources/test_package/src";
    let c_str_source_dir = CString::new(source_dir).unwrap().into_raw();

    let target_dir =
        "D:/__develop__/FutureOrientedGB/asc/test_sources/test_package/target/test_package_bin";
    let c_str_target_dir = CString::new(target_dir).unwrap().into_raw();

    let last_parsed_files = BTreeSet::<String>::new();
    let c_void_last_parsed_files =
        Box::into_raw(Box::new(last_parsed_files)) as *mut std::ffi::c_void;

    let current_parsed_files = BTreeSet::<String>::new();
    let c_current_parsed_files =
        Box::into_raw(Box::new(current_parsed_files)) as *mut std::ffi::c_void;

    let source_symbols = BTreeMap::<String, BTreeSet<String>>::new();
    let c_void_source_symbols = Box::into_raw(Box::new(source_symbols)) as *mut std::ffi::c_void;

    let source_include_headers = BTreeMap::<String, BTreeSet<String>>::new();
    let c_void_source_include_headers =
        Box::into_raw(Box::new(source_include_headers)) as *mut std::ffi::c_void;

    let header_include_by_sources = BTreeMap::<String, BTreeSet<String>>::new();
    let c_void_header_include_by_sources =
        Box::into_raw(Box::new(header_include_by_sources)) as *mut std::ffi::c_void;

    let result = ast::ClangParsedResult {
        source_path: c_str_source_path,
        source_dir: c_str_source_dir,
        target_dir: c_str_target_dir,
        last_parsed_files: c_void_last_parsed_files,
        current_parsed_files: c_current_parsed_files,
        source_symbols: c_void_source_symbols,
        source_include_headers: c_void_source_include_headers,
        header_include_by_sources: c_void_header_include_by_sources,
    };
    let box_result= Box::new(result);
    let ptr_result = Box::into_raw(box_result);
    let error_code = ast::AstCErrorCode::from(unsafe { ast::scan_symbols_and_inclusions(ptr_result) });

    match error_code {
        ast::AstCErrorCode::AstCErrorNone => {}
        ast::AstCErrorCode::AstCErrorUnknown => {}
        ast::AstCErrorCode::AstCErrorClangParseTranslationUnit => {}
    }
}
