use clang_sys;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use super::{
    parser::SourceMappings,
    util::{
        cx_string_to_string, get_class_name, get_location_info, get_namespace, string_to_c_str,
    },
};
use crate::util;

#[derive(Debug, Default, Clone)]
pub struct ParsedResult {
    source_dir: String,
    target_dir: String,
    outter_parsed_files: BTreeSet<String>,
    pub parsed_files: BTreeSet<String>,
    pub include_files: BTreeMap<String, String>,
    pub source_symbols: BTreeMap<String, BTreeSet<String>>,
}

pub fn collect_symbols_and_inclusions(source: &String, parser: &SourceMappings) -> ParsedResult {
    unsafe {
        // init data for ast visitor
        let mut result = ParsedResult {
            source_dir: parser.source_dir.clone(),
            target_dir: parser.target_dir.clone(),
            outter_parsed_files: parser.parsed_files.clone(),
            ..Default::default()
        };
        result.parsed_files.insert(source.clone());

        // set include search paths
        let mut rs_args: Vec<String> = parser
            .include_dirs
            .iter()
            .map(|s| format!("-I{}", s))
            .collect();
        rs_args.push(format!("-I{}", parser.source_dir));
        rs_args.push(format!("-I{}", parser.target_dir));
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
            return result;
        }

        // logging
        tracing::info!(
            "{}",
            util::fs::remove_prefix(source, &parser.source_dir, &parser.target_dir)
        );
        // visit the AST
        clang_sys::clang_visitChildren(
            clang_sys::clang_getTranslationUnitCursor(translation_unit),
            visit_symbols_and_inclusions,
            &mut result as *mut _ as *mut std::ffi::c_void,
        );
        // logging
        for (include, source) in &result.include_files {
            tracing::info!(
                "    {} <- {}",
                util::fs::remove_prefix(include, &parser.source_dir, &parser.target_dir),
                util::fs::remove_prefix(source, &parser.source_dir, &parser.target_dir),
            );
        }

        // clean up
        clang_sys::clang_disposeTranslationUnit(translation_unit);
        clang_sys::clang_disposeIndex(index);

        return result;
    }
}

extern "C" fn visit_symbols_and_inclusions(
    cursor: clang_sys::CXCursor,
    _parent: clang_sys::CXCursor,
    client_data: clang_sys::CXClientData,
) -> clang_sys::CXChildVisitResult {
    unsafe {
        let result = &mut *(client_data as *mut ParsedResult);

        let path = get_location_info(cursor).0;

        // skip parsed files
        if result.outter_parsed_files.contains(&path) {
            return clang_sys::CXChildVisit_Continue;
        }
        // skip third party files
        if !path.starts_with(&result.source_dir) && !path.starts_with(&result.target_dir) {
            return clang_sys::CXChildVisit_Continue;
        }
        result.parsed_files.insert(path.clone());

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
                    if include_path.starts_with(&result.source_dir)
                        || include_path.starts_with(&result.target_dir)
                    {
                        result.include_files.insert(include_path, path.clone());
                    }
                }
            }

            clang_sys::CXCursor_FunctionDecl => {
                let function_name = clang_sys::clang_getCursorSpelling(cursor);
                signature.push_str(&format!("function {}(", cx_string_to_string(function_name)));

                let function_args_count = clang_sys::clang_Cursor_getNumArguments(cursor) as u32;
                for i in 0..function_args_count {
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

                let function_type = clang_sys::clang_getCursorType(cursor);
                let return_type = clang_sys::clang_getResultType(function_type);
                let return_type_name = clang_sys::clang_getTypeSpelling(return_type);

                signature.push_str(&format!(") -> {}", cx_string_to_string(return_type_name)));
            }

            clang_sys::CXCursor_CXXMethod
            | clang_sys::CXCursor_Constructor
            | clang_sys::CXCursor_Destructor => {
                let method_name = clang_sys::clang_getCursorSpelling(cursor);
                let namespace = get_namespace(cursor);
                signature.push_str(&format!(
                    "method {}{}{}::{}(",
                    namespace,
                    if namespace.is_empty() { "" } else { ":" },
                    get_class_name(cursor),
                    cx_string_to_string(method_name)
                ));

                let method_args_count = clang_sys::clang_Cursor_getNumArguments(cursor) as u32;
                for i in 0..method_args_count {
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

                let method_type = clang_sys::clang_getCursorType(cursor);
                let return_type = clang_sys::clang_getResultType(method_type);
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
            result
                .source_symbols
                .entry(path)
                .or_default()
                .insert(signature);
        }

        return clang_sys::CXChildVisit_Recurse;
    }
}
