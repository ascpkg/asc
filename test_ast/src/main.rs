mod ast;

use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::CString,
};

fn main() {
    let cwd = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(r"\", "/");
    let result = Box::into_raw(Box::new(ast::ClangParsedResult {
        source_path: CString::new(format!("{cwd}/test_sources/test_package/src/main.cpp")).unwrap().into_raw(),
        source_dir: CString::new(format!("{cwd}/test_sources/test_package/src")).unwrap().into_raw(),
        target_dir: CString::new(format!("{cwd}/test_sources/test_package/target/test_package_bin")).unwrap().into_raw(),
        last_parsed_files: Box::into_raw(Box::new(BTreeSet::<String>::new()))
            as *mut std::ffi::c_void,
        current_parsed_files: Box::into_raw(Box::new(BTreeSet::<String>::new()))
            as *mut std::ffi::c_void,
        source_symbols: Box::into_raw(Box::new(BTreeMap::<String, BTreeSet<String>>::new()))
            as *mut std::ffi::c_void,
        source_include_headers: Box::into_raw(Box::new(BTreeMap::<String, BTreeSet<String>>::new()))
            as *mut std::ffi::c_void,
        header_include_by_sources: Box::into_raw(Box::new(
            BTreeMap::<String, BTreeSet<String>>::new(),
        )) as *mut std::ffi::c_void,
    }));
    match ast::AstCErrorCode::from(unsafe { ast::scan_symbols_and_inclusions(result) }) {
        ast::AstCErrorCode::AstCErrorNone => {}
        ast::AstCErrorCode::AstCErrorUnknown => {}
        ast::AstCErrorCode::AstCErrorClangParseTranslationUnit => {}
    }

    let box_result = unsafe { Box::from_raw(result) };
    let current_parsed_files =
        unsafe { Box::from_raw(box_result.current_parsed_files as *mut BTreeSet<String>) };
    println!("current_parsed_files:\n{:#?}", current_parsed_files);
    let last_parsed_files =
        unsafe { Box::from_raw(box_result.last_parsed_files as *mut BTreeSet<String>) };
    println!("last_parsed_files:\n{:#?}", last_parsed_files);
    let source_symbols = unsafe {
        Box::from_raw(box_result.source_symbols as *mut BTreeMap<String, BTreeSet<String>>)
    };
    println!("source_symbols:\n{:#?}", source_symbols);
    let source_include_headers = unsafe {
        Box::from_raw(box_result.source_include_headers as *mut BTreeMap<String, BTreeSet<String>>)
    };
    println!("source_include_headers:\n{:#?}", source_include_headers);
    let header_include_by_sources = unsafe {
        Box::from_raw(
            box_result.header_include_by_sources as *mut BTreeMap<String, BTreeSet<String>>,
        )
    };
    println!(
        "header_include_by_sources:\n{:#?}",
        header_include_by_sources
    );
}
