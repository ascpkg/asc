# Scan C/C++ source tree to get #include dependency and symbols

[![Docs](https://docs.rs/c_source_parser_ffi/badge.svg)](https://docs.rs/c_source_parser_ffi)
[![Crates.io](https://img.shields.io/crates/d/c_source_parser_ffi.svg)](https://crates.io/crates/c_source_parser_ffi)
[![Crates.io](https://img.shields.io/crates/v/c_source_parser_ffi.svg)](https://crates.io/crates/c_source_parser_ffi)



```rust
// copy ../asc_bin/src/clang/c_source_parser_ffi.rs
// full example can be found in ../test_source_parser

mod c_source_parser_ffi;

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    ffi::CString,
};

fn main() {
    let cwd = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(r"\", "/");

    let entry_point_source = format!("{cwd}/test_sources/test_package/src/main.cpp");
    let source_dir = format!("{cwd}/test_sources/test_package/src");
    let target_dir = format!("{cwd}/test_sources/test_package/target/test_package_bin");

    // collect from entry point file
    let result = unsafe {
        c_source_parser_ffi::scan_symbols_and_inclusions(
            CString::new(entry_point_source.clone()).unwrap().into_raw(),
            CString::new(source_dir.clone()).unwrap().into_raw(),
            CString::new(target_dir.clone()).unwrap().into_raw(),
            Box::into_raw(Box::new(BTreeSet::<String>::new())) as *mut std::ffi::c_void,
        )
    };
    let error_code = c_source_parser_ffi::AstCErrorCode::from(result.error_code);
    if error_code != c_source_parser_ffi::AstCErrorCode::AstCErrorNone {
        eprintln!(
            "ast::scan_symbols_and_inclusions error, code: {} ({})",
            std::any::type_name_of_val(&error_code),
            result.error_code,
        );
        return;
    }

    // convert from raw pointer and take ownership
    let current_parsed_files =
        unsafe { Box::from_raw(result.current_parsed_files as *mut BTreeSet<String>) };
    let last_parsed_files =
        unsafe { Box::from_raw(result.last_parsed_files as *mut BTreeSet<String>) };
    let source_symbols = unsafe {
        Box::from_raw(result.source_symbols as *mut BTreeMap<String, BTreeSet<String>>)
    };
    let source_include_headers = unsafe {
        Box::from_raw(result.source_include_headers as *mut BTreeMap<String, BTreeSet<String>>)
    };
    let header_include_by_sources = unsafe {
        Box::from_raw(
            result.header_include_by_sources as *mut BTreeMap<String, BTreeSet<String>>,
        )
    };
}
```
