use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::string::String;

use clang_sys;

#[derive(Default)]
struct ParsedResult {
    source_dir: String,
    target_dir: String,
    outer_parsed_files: BTreeSet<String>,
    parsed_files: BTreeSet<String>,
    header_include_by_sources: BTreeMap<String, BTreeSet<String>>,
    source_symbols: BTreeMap<String, BTreeSet<String>>,
}

fn cx_string_to_string(cx_str: clang_sys::CXString) -> String {
    if cx_str.data.is_null() {
        return String::new();
    }

    struct CXStringGuard(clang_sys::CXString);
    impl Drop for CXStringGuard {
        fn drop(&mut self) {
            unsafe { clang_sys::clang_disposeString(self.0) }
        }
    }

    let guard = CXStringGuard(cx_str);

    unsafe {
        let c_str = clang_sys::clang_getCString(guard.0);
        if c_str.is_null() {
            return String::new();
        }

        std::ffi::CStr::from_ptr(c_str)
            .to_string_lossy()
            .into_owned()
    }
}

fn get_namespace(cursor: clang_sys::CXCursor) -> String {
    let mut namespaces = Vec::new();
    let mut parent_cursor = unsafe { clang_sys::clang_getCursorSemanticParent(cursor) };

    unsafe {
        while clang_sys::clang_Cursor_isNull(parent_cursor) != 0 {
            if clang_sys::clang_getCursorKind(parent_cursor) == clang_sys::CXCursor_Namespace {
                namespaces.push(cx_string_to_string(clang_sys::clang_getCursorSpelling(
                    parent_cursor,
                )));
            }
            parent_cursor = clang_sys::clang_getCursorSemanticParent(parent_cursor);
        }
    }

    namespaces.reverse();
    namespaces.join("::")
}

fn get_class_name(cursor: clang_sys::CXCursor) -> String {
    let mut parent_cursor = unsafe { clang_sys::clang_getCursorSemanticParent(cursor) };

    while unsafe { clang_sys::clang_getCursorKind(parent_cursor) } != clang_sys::CXCursor_ClassDecl
    {
        parent_cursor = unsafe { clang_sys::clang_getCursorSemanticParent(parent_cursor) };
    }

    cx_string_to_string(unsafe { clang_sys::clang_getCursorSpelling(parent_cursor) })
}

fn get_location(cursor: clang_sys::CXCursor) -> (String, u32, u32) {
    let location = unsafe { clang_sys::clang_getCursorLocation(cursor) };
    let mut file: clang_sys::CXFile = std::ptr::null_mut();
    let mut line: u32 = 0;
    let mut column: u32 = 0;

    unsafe {
        clang_sys::clang_getFileLocation(
            location,
            &mut file,
            &mut line,
            &mut column,
            std::ptr::null_mut(),
        );
    }

    let file_name = cx_string_to_string(unsafe { clang_sys::clang_getFileName(file) });
    (file_name, line, column)
}

fn remove_prefix(path: &str, source_dir: &str, target_dir: &str) -> String {
    if path == source_dir || path == target_dir {
        String::new()
    } else if path.starts_with(source_dir) {
        path[source_dir.len() + 1..].to_string()
    } else if path.starts_with(target_dir) {
        path[target_dir.len() + 1..].to_string()
    } else {
        path.to_string()
    }
}

pub fn is_source(ext: &std::ffi::OsStr) -> bool {
    ext == "c" || ext == "cc" || ext == "cpp" || ext == "cxx"
}

pub fn find_source_files(source_dir: &String, exclude_path: &String) -> Vec<String> {
    let mut paths = Vec::new();

    let walker = walkdir::WalkDir::new(source_dir.clone())
        .into_iter()
        .filter_map(|e| e.ok());
    for entry in walker {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if is_source(ext) {
                if let Some(file_name) = path.to_str() {
                    let path = file_name.replace(r"\", "/");
                    if &path != exclude_path {
                        paths.push(path);
                    }
                }
            }
        }
    }

    paths
}

extern "C" fn visit_symbols_and_inclusions(
    cursor: clang_sys::CXCursor,
    _parent: clang_sys::CXCursor,
    client_data: clang_sys::CXClientData,
) -> clang_sys::CXChildVisitResult {
    let result = unsafe { &mut *(client_data as *mut ParsedResult) };

    let (source_path, _line, _column) = get_location(cursor);
    let source_path = source_path.replace("\\", "/");

    if result.outer_parsed_files.contains(&source_path) {
        return clang_sys::CXChildVisit_Continue;
    }

    if !source_path.starts_with(&result.source_dir) && !source_path.starts_with(&result.target_dir)
    {
        return clang_sys::CXChildVisit_Continue;
    }

    result.parsed_files.insert(source_path.clone());

    let cursor_type = unsafe { clang_sys::clang_getCursorKind(cursor) };
    let mut symbol_signature = String::new();

    match cursor_type {
        clang_sys::CXCursor_InclusionDirective => {
            let include_file = unsafe { clang_sys::clang_getIncludedFile(cursor) };
            if !include_file.is_null() {
                let include_path =
                    cx_string_to_string(unsafe { clang_sys::clang_getFileName(include_file) });
                let include_path = include_path.replace("\\", "/");

                if include_path.starts_with(&result.source_dir)
                    || include_path.starts_with(&result.target_dir)
                {
                    result
                        .header_include_by_sources
                        .entry(include_path.clone())
                        .or_insert_with(BTreeSet::new)
                        .insert(source_path.clone());
                }
            }
        }
        clang_sys::CXCursor_FunctionDecl
        | clang_sys::CXCursor_CXXMethod
        | clang_sys::CXCursor_Constructor
        | clang_sys::CXCursor_Destructor => {
            let func_type = if cursor_type == clang_sys::CXCursor_FunctionDecl {
                "function"
            } else {
                "method"
            };

            let namespace_ = get_namespace(cursor);
            let class_name = if cursor_type != clang_sys::CXCursor_FunctionDecl {
                get_class_name(cursor)
            } else {
                String::new()
            };

            let func_name =
                cx_string_to_string(unsafe { clang_sys::clang_getCursorSpelling(cursor) });
            symbol_signature = format!(
                "{} {}{}{}{}{}(",
                func_type,
                namespace_,
                if namespace_.is_empty() { "" } else { ":" },
                class_name,
                if class_name.is_empty() { "" } else { ":" },
                func_name
            );

            let num_args = unsafe { clang_sys::clang_Cursor_getNumArguments(cursor) } as u32;
            for i in 0..num_args {
                let arg_cursor = unsafe { clang_sys::clang_Cursor_getArgument(cursor, i) };
                let arg_type = unsafe { clang_sys::clang_getCursorType(arg_cursor) };
                let arg_canonical_type = unsafe { clang_sys::clang_getCanonicalType(arg_type) };

                let arg_type_name = if arg_canonical_type.kind == arg_type.kind {
                    unsafe { clang_sys::clang_getTypeSpelling(arg_type) }
                } else {
                    unsafe { clang_sys::clang_getTypeSpelling(arg_canonical_type) }
                };

                if i > 0 {
                    symbol_signature += ", ";
                }
                symbol_signature += &cx_string_to_string(arg_type_name);
            }

            let return_type =
                unsafe { clang_sys::clang_getResultType(clang_sys::clang_getCursorType(cursor)) };
            let return_type_name = unsafe { clang_sys::clang_getTypeSpelling(return_type) };
            symbol_signature += &format!(") -> {}", cx_string_to_string(return_type_name));
        }
        clang_sys::CXCursor_ClassDecl => {
            let name = unsafe { clang_sys::clang_getCursorSpelling(cursor) };
            symbol_signature = format!("class {}", cx_string_to_string(name));
        }
        clang_sys::CXCursor_StructDecl => {
            let name = unsafe { clang_sys::clang_getCursorSpelling(cursor) };
            symbol_signature = format!("struct {}", cx_string_to_string(name));
        }
        clang_sys::CXCursor_EnumDecl => {
            let name = unsafe { clang_sys::clang_getCursorSpelling(cursor) };
            symbol_signature = format!("enum {}", cx_string_to_string(name));
        }
        clang_sys::CXCursor_UnionDecl => {
            let name = unsafe { clang_sys::clang_getCursorSpelling(cursor) };
            symbol_signature = format!("union {}", cx_string_to_string(name));
        }
        clang_sys::CXCursor_VarDecl => {
            let name = unsafe { clang_sys::clang_getCursorSpelling(cursor) };
            symbol_signature = format!("var {}", cx_string_to_string(name));
        }
        clang_sys::CXCursor_TypedefDecl => {
            let name = unsafe { clang_sys::clang_getCursorSpelling(cursor) };
            symbol_signature = format!("typedef {}", cx_string_to_string(name));
        }
        _ => {}
    }

    if !symbol_signature.is_empty() {
        result
            .source_symbols
            .entry(source_path)
            .or_insert_with(BTreeSet::new)
            .insert(symbol_signature);
    }

    clang_sys::CXChildVisit_Recurse
}

fn collect_symbols_and_inclusions(
    source: &str,
    source_dir: &str,
    target_dir: &str,
    parsed_files: &BTreeSet<String>,
) -> ParsedResult {
    let mut result = ParsedResult {
        source_dir: source_dir.to_string(),
        target_dir: target_dir.to_string(),
        outer_parsed_files: parsed_files.clone(),
        ..Default::default()
    };

    let args = vec![
        "-I".as_ptr(),
        source_dir.as_ptr(),
        "-I".as_ptr(),
        target_dir.as_ptr(),
    ];

    let index = unsafe { clang_sys::clang_createIndex(0, 0) };
    let file: std::ffi::CString = std::ffi::CString::new(source).unwrap();
    let translation_unit = unsafe {
        clang_sys::clang_parseTranslationUnit(
            index,
            file.as_ptr(),
            args.as_ptr() as *const *const i8,
            args.len() as i32,
            std::ptr::null_mut(),
            0,
            clang_sys::CXTranslationUnit_DetailedPreprocessingRecord
                | clang_sys::CXTranslationUnit_SkipFunctionBodies
                | clang_sys::CXTranslationUnit_KeepGoing,
        )
    };

    if translation_unit.is_null() {
        unsafe { clang_sys::clang_disposeIndex(index) };
        eprintln!("clang_parseTranslationUnit error, path: {}", source);
        return result;
    }

    unsafe {
        clang_sys::clang_visitChildren(
            clang_sys::clang_getTranslationUnitCursor(translation_unit),
            visit_symbols_and_inclusions,
            &mut result as *mut _ as clang_sys::CXClientData,
        );
        clang_sys::clang_disposeTranslationUnit(translation_unit);
        clang_sys::clang_disposeIndex(index);
    }

    result
}

struct SourceMappings {
    source_dir: String,
    target_dir: String,
    parsed_files: BTreeSet<String>,
    header_include_by_sources: BTreeMap<String, BTreeSet<String>>,
    source_symbols: BTreeMap<String, BTreeSet<String>>,
}

impl SourceMappings {
    pub fn new(source_dir: String, target_dir: String) -> Self {
        Self {
            source_dir,
            target_dir,
            parsed_files: BTreeSet::new(),
            header_include_by_sources: BTreeMap::new(),
            source_symbols: BTreeMap::new(),
        }
    }

    pub fn scan(&mut self, entry_point_file: String) {
        let result = collect_symbols_and_inclusions(
            &entry_point_file,
            &self.source_dir,
            &self.target_dir,
            &self.parsed_files,
        );

        let necessaries = result.header_include_by_sources.clone();

        self.collect(result);

        for source_path in find_source_files(&self.source_dir, &entry_point_file) {
            let result = collect_symbols_and_inclusions(
                &source_path,
                &self.source_dir,
                &self.target_dir,
                &self.parsed_files,
            );
            self.collect(result);
        }

        self.clean(necessaries);
    }

    pub fn collect(&mut self, result: ParsedResult) {
        self.parsed_files.extend(result.parsed_files);

        for (header, sources) in result.header_include_by_sources {
            self.header_include_by_sources
                .entry(header)
                .or_insert_with(BTreeSet::new)
                .extend(sources);
        }

        for (source, symbols) in result.source_symbols {
            self.source_symbols
                .entry(source)
                .or_insert_with(BTreeSet::new)
                .extend(symbols);
        }
    }

    pub fn clean(&mut self, mut necessaries: BTreeMap<String, BTreeSet<String>>) {
        for (header, sources) in &mut necessaries {
            if let Some(header_sources) = self.header_include_by_sources.get_mut(header) {
                if let Some(header_symbols) = self.source_symbols.get(header) {
                    for source in header_sources.iter() {
                        if let Some(source_symbols) = self.source_symbols.get(source) {
                            let intersection: BTreeSet<_> =
                                header_symbols.intersection(source_symbols).collect();
                            if !intersection.is_empty() {
                                sources.insert(source.clone());
                            }
                        }
                    }
                }
            }
        }

        let mut necessary_sources: BTreeSet<String> = BTreeSet::new();
        for (header, sources) in &necessaries {
            necessary_sources.insert(header.clone());
            necessary_sources.extend(sources.clone());
        }

        for key in self.source_symbols.keys().cloned().collect::<Vec<_>>() {
            if !necessary_sources.contains(&key) {
                self.source_symbols.remove(&key);
            }
        }

        self.header_include_by_sources = necessaries;
    }

    pub fn print(&self) {
        self.print_map(&self.header_include_by_sources, &self.source_symbols);
    }

    fn print_map(
        &self,
        header_include_by_sources: &BTreeMap<String, BTreeSet<String>>,
        source_symbols: &BTreeMap<String, BTreeSet<String>>,
    ) {
        for (header, sources) in header_include_by_sources {
            let header_path = remove_prefix(header, &self.source_dir, &self.target_dir);
            for source in sources {
                println!(
                    "{}    {}",
                    header_path,
                    remove_prefix(source, &self.source_dir, &self.target_dir)
                );
            }
        }

        println!("=========================================================");

        for (source, symbols) in source_symbols {
            let source_path = remove_prefix(source, &self.source_dir, &self.target_dir);
            for symbol in symbols {
                println!("{}    {}", source_path, symbol);
            }
            println!("---------------------------------------------------------");
        }
    }
}

fn main() {
    let cwd = env::current_dir().expect("Failed to get current directory");
    let cwd_str = cwd.to_string_lossy().replace("\\", "/");

    let source_dir = format!("{}/src", cwd_str);
    let target_dir = format!("{}/target/test_package_bin", cwd_str);

    let mut mappings = SourceMappings::new(source_dir.clone(), target_dir.clone());
    mappings.scan(format!("{}/main.cpp", source_dir));

    println!("---------------------------------------------------------");
    mappings.print();
}
