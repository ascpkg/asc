use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use clang_sys;

type StringSet = BTreeSet<String>;
type StringSetMap = BTreeMap<String, BTreeSet<String>>;
type RcRefCellStringSetMap = Rc<RefCell<StringSetMap>>;

#[derive(Debug, Clone)]
pub struct SourceMappings {
    // header - sources
    pub header_include_by_sources: StringSetMap,
    // source - headers
    pub source_include_headers: StringSetMap,
}

impl SourceMappings {
    pub fn scan(
        entry_point_source: &String,
        source_dir: &String,
        target_dir: &String,
    ) -> SourceMappings {
        let mut parsed_files = BTreeSet::new();

        let (source_to_headers_from_entry_point, header_to_sources_from_entry_point) =
            Self::get_includes_from_entry_point(
                &mut parsed_files,
                entry_point_source.clone(),
                source_dir,
                target_dir,
            );

        let (_source_to_headers_from_source_files, header_to_sources_from_sources_files) =
            Self::get_includes_from_source_files(source_dir, target_dir, &mut parsed_files);

        for (header, sources) in header_to_sources_from_entry_point.borrow_mut().iter_mut() {
            if header_to_sources_from_sources_files
                .borrow()
                .contains_key(header)
            {
                for source in header_to_sources_from_sources_files
                    .borrow()
                    .get(header)
                    .unwrap()
                {
                    sources.insert(source.clone());
                }
            }
        }

        for (header, sources) in source_to_headers_from_entry_point.borrow_mut().iter_mut() {
            if header_to_sources_from_sources_files
                .borrow()
                .contains_key(header)
            {
                for source in header_to_sources_from_sources_files
                    .borrow()
                    .get(header)
                    .unwrap()
                {
                    sources.insert(source.clone());
                }
            }
        }

        return SourceMappings {
            header_include_by_sources: header_to_sources_from_entry_point.borrow().clone(),
            source_include_headers: source_to_headers_from_entry_point.borrow().clone(),
        };
    }

    fn get_includes_from_source_files(
        source_dir: &String,
        target_dir: &String,
        parsed_files: &mut StringSet,
    ) -> (RcRefCellStringSetMap, RcRefCellStringSetMap) {
        let source_to_headers = Rc::new(RefCell::new(BTreeMap::new()));
        let header_to_sources = Rc::new(RefCell::new(BTreeMap::new()));

        for source_file in find_source_files(source_dir) {
            Self::get_include_files_in_source_dir(
                parsed_files,
                &source_file,
                source_dir,
                target_dir,
                source_to_headers.clone(),
                header_to_sources.clone(),
            );
        }

        return (source_to_headers, header_to_sources);
    }

    fn get_includes_from_entry_point(
        parsed_files: &mut StringSet,
        source_file: String,
        source_dir: &String,
        target_dir: &String,
    ) -> (RcRefCellStringSetMap, RcRefCellStringSetMap) {
        let source_to_headers = Rc::new(RefCell::new(BTreeMap::new()));
        let header_to_sources = Rc::new(RefCell::new(BTreeMap::new()));

        Self::get_include_files_in_source_dir(
            parsed_files,
            &source_file,
            source_dir,
            target_dir,
            source_to_headers.clone(),
            header_to_sources.clone(),
        );

        return (source_to_headers, header_to_sources);
    }

    fn get_include_files_in_source_dir(
        parsed_files: &mut StringSet,
        source_file: &String,
        source_dir: &String,
        target_dir: &String,
        source_include_headers: RcRefCellStringSetMap,
        header_include_by_sources: RcRefCellStringSetMap,
    ) {
        // skip parsed
        if parsed_files.contains(source_file) {
            return;
        }
        parsed_files.insert(source_file.clone());

        for include in get_include_files(source_file, source_dir, target_dir) {
            // map source to headers
            source_include_headers
                .borrow_mut()
                .entry(source_file.clone())
                .or_insert_with(BTreeSet::new)
                .insert(include.clone());

            // map header to sources
            let header_include_by_sources_cloned = header_include_by_sources.clone();
            header_include_by_sources_cloned
                .borrow_mut()
                .entry(include.clone())
                .or_insert_with(BTreeSet::new)
                .insert(source_file.clone());

            // recurse
            Self::get_include_files_in_source_dir(
                parsed_files,
                &include,
                source_dir,
                target_dir,
                source_include_headers.clone(),
                header_include_by_sources.clone(),
            );
        }
    }
}

#[derive(Debug, Default)]
struct ClientData {
    pub source_dir: String,
    pub target_dir: String,
    pub include_files: StringSet,
}

pub fn get_include_files(source: &String, source_dir: &String, target_dir: &String) -> StringSet {
    let mut client_data = ClientData::default();
    client_data.source_dir = source_dir.clone();
    client_data.target_dir = target_dir.clone();

    // set include search paths
    let mut rs_args = vec![];
    rs_args.push(format!("-I{}", source_dir));
    rs_args.push(format!("-I{}", target_dir));
    // println!(arguments = rs_args.join(" "), is_dir_exists=is_dir_exists(target_dir));
    // println!("{}", std::fs::read_to_string(source).unwrap());
    let c_args: Vec<*const std::ffi::c_char> = rs_args
        .iter()
        .map(|s| s.as_ptr() as *const std::ffi::c_char)
        .collect();

    // create an index
    let index = unsafe { clang_sys::clang_createIndex(0, 0) };

    // parse the translation unit
    let translation_unit = unsafe {
        clang_sys::clang_parseTranslationUnit(
            index,
            string_to_cstr(source).as_ptr(),
            c_args.as_ptr(),
            rs_args.len() as i32,
            std::ptr::null_mut(),
            0,
            clang_sys::CXTranslationUnit_DetailedPreprocessingRecord
                | clang_sys::CXTranslationUnit_SkipFunctionBodies
                | clang_sys::CXTranslationUnit_SingleFileParse
                | clang_sys::CXTranslationUnit_KeepGoing,
            // | clang_sys::CXTranslationUnit_RetainExcludedConditionalBlocks
        )
    };
    if translation_unit.is_null() {
        eprintln!("clang_sys::clang_parseTranslationUnit error, source: {source}");
        unsafe { clang_sys::clang_disposeIndex(index) };
        return client_data.include_files;
    }

    // get the cursor for the translation unit
    let cursor = unsafe { clang_sys::clang_getTranslationUnitCursor(translation_unit) };

    // visit the AST
    unsafe {
        clang_sys::clang_visitChildren(
            cursor,
            visit_inclusion_directive,
            &mut client_data as *mut _ as *mut std::ffi::c_void,
        );
    }

    // clean up
    unsafe {
        clang_sys::clang_disposeTranslationUnit(translation_unit);
        clang_sys::clang_disposeIndex(index);
    }

    println!("{}", remove_prefix(source, source_dir, target_dir));
    for include in &client_data.include_files {
        if include.starts_with(source_dir) || include.starts_with(target_dir) {
            // println!("    {}", remove_prefix(include, source_dir, target_dir));
        }
    }

    // skip third-party
    client_data
        .include_files
        .retain(|s| s.starts_with(source_dir) || s.starts_with(target_dir));

    return client_data.include_files;
}

fn string_to_cstr(rust_str: &String) -> std::ffi::CString {
    std::ffi::CString::new(rust_str.as_str()).unwrap()
}

fn cxstring_to_string(cx_str: clang_sys::CXString) -> String {
    if cx_str.data.is_null() {
        return String::new();
    }

    let c_str =
        unsafe { std::ffi::CStr::from_ptr(clang_sys::clang_getCString(cx_str) as *const _) };
    let rust_str = c_str.to_string_lossy().into_owned();

    unsafe { clang_sys::clang_disposeString(cx_str) };

    return rust_str;
}

extern "C" fn visit_inclusion_directive(
    cursor: clang_sys::CXCursor,
    _parent: clang_sys::CXCursor,
    client_data: clang_sys::CXClientData,
) -> clang_sys::CXChildVisitResult {
    let client_data = unsafe { &mut *(client_data as *mut ClientData) };

    let cursor_kind = unsafe { clang_sys::clang_getCursorKind(cursor) };
    match cursor_kind {
        clang_sys::CXCursor_InclusionDirective => {
            let display_name = unsafe { clang_sys::clang_getCursorDisplayName(cursor) };
            let name = cxstring_to_string(display_name);

            let include_file = unsafe { clang_sys::clang_getIncludedFile(cursor) };
            if !include_file.is_null() {
                let include_file_name = unsafe { clang_sys::clang_getFileName(include_file) };
                let path = cxstring_to_string(include_file_name).replace(r"\", "/");

                print!(
                    "{name}  // {}",
                    remove_prefix(&path, &client_data.source_dir, &client_data.target_dir)
                );
                get_location(cursor, &client_data.source_dir, &client_data.target_dir);

                client_data.include_files.insert(path);
            }
        }

        clang_sys::CXCursor_FunctionDecl => {
            unsafe {
                let func_name = clang_sys::clang_getCursorSpelling(cursor);
                print!("    fn {}(", cxstring_to_string(func_name));
                let func_args_count = clang_sys::clang_Cursor_getNumArguments(cursor) as u32;
                for i in 0..func_args_count {
                    let arg_cursor = clang_sys::clang_Cursor_getArgument(cursor, i);
                    let _arg_name = clang_sys::clang_getCursorSpelling(arg_cursor);

                    let arg_type = clang_sys::clang_getCursorType(arg_cursor);
                    let arg_type_name = clang_sys::clang_getTypeSpelling(arg_type);
                    if i != 0 {
                        print!(", ")
                    }
                    // print!("{}: {}", cxstring_to_string(arg_name), cxstring_to_string(arg_type_name));
                    print!("{}", cxstring_to_string(arg_type_name));
                }

                let func_type = clang_sys::clang_getCursorType(cursor);
                let return_type = clang_sys::clang_getResultType(func_type);
                let return_type_name = clang_sys::clang_getTypeSpelling(return_type);

                print!(") -> {}  // ", cxstring_to_string(return_type_name));
                get_location(cursor, &client_data.source_dir, &client_data.target_dir);
            }
        }

        clang_sys::CXCursor_StructDecl => unsafe {
            let struct_name = clang_sys::clang_getCursorSpelling(cursor);

            print!("    struct {}  // ", cxstring_to_string(struct_name));
            get_location(cursor, &client_data.source_dir, &client_data.target_dir);
        },

        clang_sys::CXCursor_EnumDecl => unsafe {
            let enum_name = clang_sys::clang_getCursorSpelling(cursor);
            print!("    enum {}  // ", cxstring_to_string(enum_name));
            get_location(cursor, &client_data.source_dir, &client_data.target_dir);
        },

        clang_sys::CXCursor_UnionDecl => unsafe {
            let union_name = clang_sys::clang_getCursorSpelling(cursor);
            print!("    union {}  // ", cxstring_to_string(union_name));
            get_location(cursor, &client_data.source_dir, &client_data.target_dir);
        },

        clang_sys::CXCursor_VarDecl => unsafe {
            let var_name = clang_sys::clang_getCursorSpelling(cursor);
            print!("    var {}  // ", cxstring_to_string(var_name));
            get_location(cursor, &client_data.source_dir, &client_data.target_dir);
        },

        clang_sys::CXCursor_TypedefDecl => unsafe {
            let typedef_name = clang_sys::clang_getCursorSpelling(cursor);
            print!("    typedef {}  // ", cxstring_to_string(typedef_name));
            get_location(cursor, &client_data.source_dir, &client_data.target_dir);
        },

        _ => {}
    }

    clang_sys::CXChildVisit_Recurse
}

fn get_location(cursor: clang_sys::CXCursor, source_dir: &String, target_dir: &String) {
    unsafe {
        let location = clang_sys::clang_getCursorLocation(cursor);
        let file: *mut clang_sys::CXString =
            Box::into_raw(Box::new(clang_sys::CXString::default()));
        let mut line: u32 = 0;
        let mut column: u32 = 0;

        clang_sys::clang_getPresumedLocation(location, file, &mut line, &mut column);

        println!(
            "{}@{}#{}",
            remove_prefix(&cxstring_to_string(*file), source_dir, target_dir),
            line,
            column
        );
    }
}

pub fn find_source_files(dir: &String) -> Vec<String> {
    let mut files = Vec::new();

    let walker = walkdir::WalkDir::new(dir.clone())
        .into_iter()
        .filter_map(|e| e.ok());
    for entry in walker {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if is_source(ext) {
                if let Some(file_name) = path.to_str() {
                    files.push(file_name.replace(r"\", "/"));
                }
            }
        }
    }

    files
}

pub fn is_source(ext: &std::ffi::OsStr) -> bool {
    ext == "c" || ext == "cc" || ext == "cpp" || ext == "cxx"
}

pub fn remove_prefix(path: &String, source_dir: &String, target_dir: &String) -> String {
    if path == source_dir || path == target_dir {
        String::new()
    } else if path.starts_with(source_dir) {
        path.clone().split_off(source_dir.len() + 1)
    } else if path.starts_with(target_dir) {
        path.clone().split_off(target_dir.len() + 1)
    } else {
        String::new()
    }
}

// fn main() {
//     let source_mappings = SourceMappings::scan(
//         &String::from("test_sources/test_c/src/main.c"),
//         &String::from("test_sources/test_c/src"),
//         &String::from("test_sources/test_c/target"),
//     );
// }

use clang_sys::*;
use std::ffi::{CStr, CString};

#[derive(Debug)]
struct FunctionLocation {
    name: String,

    decl_file: String,
    decl_line: u32,
    decl_column: u32,

    def_file: String,
    def_line: u32,
    def_column: u32,
    has_definition: bool,
}

unsafe fn get_location_info(location: CXSourceLocation) -> (String, u32, u32) {
    let mut file = Box::new(CXString::default());
    let mut line: u32 = 0;
    let mut column: u32 = 0;

    clang_getPresumedLocation(location, file.as_mut(), &mut line, &mut column);

    let file_str = if !file.data.is_null() {
        let c_str = clang_getCString(*file);
        let result = CStr::from_ptr(c_str).to_string_lossy().into_owned();
        clang_disposeString(*file);
        result
    } else {
        String::from("")
    };

    (file_str, line, column)
}

unsafe fn get_function_info(cursor: CXCursor) -> Option<FunctionLocation> {
    if clang_getCursorKind(cursor) != CXCursor_FunctionDecl {
        return None;
    }

    let name = {
        let spelling = clang_getCursorSpelling(cursor);
        let result = CStr::from_ptr(clang_getCString(spelling))
            .to_string_lossy()
            .into_owned();
        clang_disposeString(spelling);
        result
    };

    let decl_location = clang_getCursorLocation(cursor);
    let (decl_file, decl_line, decl_column) = get_location_info(decl_location);

    let has_definition = clang_isCursorDefinition(cursor) != 0;

    let mut def_file = String::new();
    let mut def_line = 0;
    let mut def_column = 0;

    if !has_definition {
        let definition = clang_getCursorDefinition(cursor);
        if clang_Cursor_isNull(definition) == 0 {
            let def_location = clang_getCursorLocation(definition);
            let (file, line, column) = get_location_info(def_location);
            def_file = file;
            def_line = line;
            def_column = column;
        }
    } else {
        def_file = decl_file.clone();
        def_line = decl_line;
        def_column = decl_column;
    }

    Some(FunctionLocation {
        name,
        decl_file,
        decl_line,
        decl_column,
        def_file,
        def_line,
        def_column,
        has_definition,
    })
}

unsafe fn find_function_definition(cursor: CXCursor) -> Option<CXCursor> {
    if clang_getCursorKind(cursor) != CXCursor_FunctionDecl {
        return None;
    }

    if clang_isCursorDefinition(cursor) != 0 {
        return Some(cursor);
    }

    let definition = clang_getCursorDefinition(cursor);
    if clang_Cursor_isNull(definition) != 0 {
        None
    } else {
        Some(definition)
    }
}

struct FunctionCounter(*mut i32);

extern "C" fn visit_function(
    cursor: CXCursor,
    _parent: CXCursor,
    client_data: CXClientData,
) -> CXChildVisitResult {
    unsafe {
        if clang_getCursorKind(cursor) == CXCursor_FunctionDecl {
            if let Some(info) = get_function_info(cursor) {
                println!("\nFunction: {}", info.name);
                println!(
                    "Declaration at {}:{}:{}",
                    info.decl_file, info.decl_line, info.decl_column
                );

                if info.has_definition {
                    println!(
                        "Definition at {}:{}:{}",
                        info.def_file, info.def_line, info.def_column
                    );
                } else {
                    println!("No definition found");
                }

                let counter = client_data as *mut i32;
                *counter += 1;
            }
        }
        CXChildVisit_Recurse
    }
}

fn parse_file(file_path: &str) -> Result<i32, String> {
    unsafe {
        let index = clang_createIndex(0, 0);
        if index.is_null() {
            return Err("Failed to create index".to_string());
        }

        let c_file_path =
            CString::new(file_path).map_err(|e| format!("Failed to create CString: {}", e))?;

        let tu = clang_parseTranslationUnit(
            index,
            c_file_path.as_ptr(),
            std::ptr::null(),
            0,
            std::ptr::null_mut(),
            0,
            CXTranslationUnit_None as i32,
        );

        if tu.is_null() {
            clang_disposeIndex(index);
            return Err("Failed to parse translation unit".to_string());
        }

        let cursor = clang_getTranslationUnitCursor(tu);

        let mut counter = 0;
        let counter_ptr: *mut i32 = &mut counter;

        clang_visitChildren(cursor, visit_function, counter_ptr as CXClientData);

        clang_disposeTranslationUnit(tu);
        clang_disposeIndex(index);

        Ok(counter)
    }
}

fn main() {
    let file_path = "test_sources/test_c/src/test.c";
    match parse_file(file_path) {
        Ok(count) => println!("\nFound {} functions", count),
        Err(e) => eprintln!("Error: {}", e),
    }
}
