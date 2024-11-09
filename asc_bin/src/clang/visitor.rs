use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::util;

use clang_sys;

use super::parser::SourceMappings;

struct ClientData {
    source_dir: String,
    target_dir: String,
    include_files: BTreeSet<String>,
    source_symbols: BTreeMap<String, BTreeSet<String>>,
}

pub fn get_symbols_and_inclusions(
    source: &String,
    parser: &SourceMappings,
) -> (BTreeSet<String>, BTreeMap<String, BTreeSet<String>>) {
    unsafe {
        // init data for ast visitor
        let mut client_data = ClientData {
            source_dir: parser.source_dir.clone(),
            target_dir: parser.target_dir.clone(),
            include_files: BTreeSet::new(),
            source_symbols: BTreeMap::new(),
        };

        // set include search paths
        let mut rs_args: Vec<String> = parser
            .include_dirs
            .iter()
            .map(|s| format!("-I{}", s))
            .collect();
        rs_args.push(format!("-I{}", parser.source_dir));
        rs_args.push(format!("-I{}", parser.target_dir));
        rs_args.push(String::from("-nostdinc"));
        rs_args.push(String::from("-nostdinc++"));
        let c_args: Vec<*const std::ffi::c_char> = rs_args
            .iter()
            .map(|s| s.as_ptr() as *const std::ffi::c_char)
            .collect();

        // create an index
        let index = clang_sys::clang_createIndex(0, 0);

        // parse the translation unit
        let translation_unit = clang_sys::clang_parseTranslationUnit(
            index,
            string_to_c_str(source).as_ptr(),
            c_args.as_ptr(),
            rs_args.len() as i32,
            std::ptr::null_mut(),
            0,
            // | clang_sys::CXTranslationUnit_SingleFileParse
            // | clang_sys::CXTranslationUnit_RetainExcludedConditionalBlocks
            clang_sys::CXTranslationUnit_DetailedPreprocessingRecord
                | clang_sys::CXTranslationUnit_SkipFunctionBodies
                | clang_sys::CXTranslationUnit_KeepGoing,
        );
        if translation_unit.is_null() {
            tracing::error!(
                message = "clang_sys::clang_parseTranslationUnit error",
                path = source
            );
            clang_sys::clang_disposeIndex(index);
            return (client_data.include_files, client_data.source_symbols);
        }

        // visit the AST
        clang_sys::clang_visitChildren(
            clang_sys::clang_getTranslationUnitCursor(translation_unit),
            visit_symbols_and_inclusions,
            &mut client_data as *mut _ as *mut std::ffi::c_void,
        );

        // clean up
        clang_sys::clang_disposeTranslationUnit(translation_unit);
        clang_sys::clang_disposeIndex(index);

        // logging
        tracing::info!(
            "{}",
            util::fs::remove_prefix(source, &parser.source_dir, &parser.target_dir)
        );
        for include in &client_data.include_files {
            if include.starts_with(&parser.source_dir) || include.starts_with(&parser.target_dir) {
                tracing::info!(
                    "    {}",
                    util::fs::remove_prefix(include, &parser.source_dir, &parser.target_dir)
                );
            }
        }

        return (client_data.include_files, client_data.source_symbols);
    }
}

extern "C" fn visit_symbols_and_inclusions(
    cursor: clang_sys::CXCursor,
    _parent: clang_sys::CXCursor,
    client_data: clang_sys::CXClientData,
) -> clang_sys::CXChildVisitResult {
    unsafe {
        let client_data = &mut *(client_data as *mut ClientData);

        let (path, _line, _column) = get_location_info(cursor);
        if !path.starts_with(&client_data.source_dir) && !path.starts_with(&client_data.target_dir)
        {
            // skip third-party
            return clang_sys::CXChildVisit_Continue;
        }

        // format symbol signature
        let mut signature = String::new();
        match clang_sys::clang_getCursorKind(cursor) {
            clang_sys::CXCursor_InclusionDirective => {
                let include_file = clang_sys::clang_getIncludedFile(cursor);
                if !include_file.is_null() {
                    let include_path =
                        cx_string_to_string(clang_sys::clang_getFileName(include_file))
                            .replace(r"\", "/");

                    // let include_name = cx_string_to_string(clang_sys::clang_getCursorDisplayName(cursor));
                    // tracing::info!("{include_name}  // {include_path}");

                    // skip third-party
                    if include_path.starts_with(&client_data.source_dir)
                        || include_path.starts_with(&client_data.target_dir)
                    {
                        client_data.include_files.insert(include_path);
                    }
                }
            }

            clang_sys::CXCursor_FunctionDecl => {
                let func_name = clang_sys::clang_getCursorSpelling(cursor);
                signature.push_str(&format!("fn {}(", cx_string_to_string(func_name)));

                let func_args_count = clang_sys::clang_Cursor_getNumArguments(cursor) as u32;
                for i in 0..func_args_count {
                    let arg_cursor = clang_sys::clang_Cursor_getArgument(cursor, i);
                    // let arg_name = clang_sys::clang_getCursorSpelling(arg_cursor);

                    let arg_type = clang_sys::clang_getCursorType(arg_cursor);
                    let arg_canonical_type = clang_sys::clang_getCanonicalType(arg_type);
                    let arg_type_name = if arg_canonical_type.kind == arg_type.kind {
                        clang_sys::clang_getTypeSpelling(arg_type)
                    } else {
                        clang_sys::clang_getTypeSpelling(arg_canonical_type)
                    };
                    if i > 0 {
                        signature.push_str(", ")
                    }
                    signature.push_str(&cx_string_to_string(arg_type_name));
                }

                let func_type = clang_sys::clang_getCursorType(cursor);
                let return_type = clang_sys::clang_getResultType(func_type);
                let return_type_name = clang_sys::clang_getTypeSpelling(return_type);

                signature.push_str(&format!(") -> {}", cx_string_to_string(return_type_name)));
            }

            clang_sys::CXCursor_ClassDecl => {
                let name = clang_sys::clang_getCursorSpelling(cursor);
                signature = format!("class {}", cx_string_to_string(name));
            }

            clang_sys::CXCursor_StructDecl => {
                let name = clang_sys::clang_getCursorSpelling(cursor);
                signature = format!("struct {}", cx_string_to_string(name));
            }

            clang_sys::CXCursor_EnumDecl => {
                let name = clang_sys::clang_getCursorSpelling(cursor);
                signature = format!("enum {}", cx_string_to_string(name));
            }

            clang_sys::CXCursor_UnionDecl => {
                let name = clang_sys::clang_getCursorSpelling(cursor);
                signature = format!("union {}", cx_string_to_string(name));
            }

            clang_sys::CXCursor_VarDecl => {
                let name = clang_sys::clang_getCursorSpelling(cursor);
                signature = format!("var {}", cx_string_to_string(name));
            }

            clang_sys::CXCursor_TypedefDecl => {
                let name = clang_sys::clang_getCursorSpelling(cursor);
                signature = format!("typedef {}", cx_string_to_string(name));
            }

            _ => {}
        }

        if !signature.is_empty() {
            client_data
                .source_symbols
                .entry(path)
                .or_insert_with(BTreeSet::new)
                .insert(signature);
        }
    }

    return clang_sys::CXChildVisit_Recurse;
}

fn string_to_c_str(rust_str: &String) -> std::ffi::CString {
    std::ffi::CString::new(rust_str.as_str()).unwrap()
}

fn cx_string_to_string(cx_str: clang_sys::CXString) -> String {
    if cx_str.data.is_null() {
        return String::new();
    }

    unsafe {
        let c_str = std::ffi::CStr::from_ptr(clang_sys::clang_getCString(cx_str) as *const _);
        let rust_str = c_str.to_string_lossy().into_owned();
        clang_sys::clang_disposeString(cx_str);
        return rust_str;
    }
}

fn get_location_info(cursor: clang_sys::CXCursor) -> (String, u32, u32) {
    let file: *mut clang_sys::CXString = Box::into_raw(Box::new(clang_sys::CXString::default()));
    let mut line: u32 = 0;
    let mut column: u32 = 0;
    unsafe {
        let location = clang_sys::clang_getCursorLocation(cursor);
        clang_sys::clang_getPresumedLocation(location, file, &mut line, &mut column);
        let path = cx_string_to_string(*file).replace(r"\", "/");
        return (path, line, column);
    }
}
