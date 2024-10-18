use clang_sys;

use crate::util;

type StringSet = std::collections::BTreeSet<String>;
type StringMapSet = std::collections::BTreeMap<String, std::collections::BTreeSet<String>>;
type RcRefCellStringMapSet = std::rc::Rc<std::cell::RefCell<StringMapSet>>;

#[derive(Debug, Clone)]
pub struct SourceMappings {
    // header - sources
    pub header_inclued_by_sources: StringMapSet,
    // source - headers
    pub source_include_headers: StringMapSet,
}

impl SourceMappings {
    pub fn scan(options: &util::cli::CommandLines) -> SourceMappings {
        let mut parsed_files = std::collections::BTreeSet::new();

        let (source_to_headers_from_entry_point, header_to_sources_from_entry_point) =
            Self::get_includes_from_entry_point(
                &mut parsed_files,
                &options.source_dir,
                format!("{}/{}", &options.source_dir, &options.entry_point_source),
                &options.include_dirs,
            );

        let (_source_to_headers_from_source_files, header_to_sources_from_sources_files) =
            Self::get_includes_from_source_files(
                &mut parsed_files,
                &options.source_dir,
                &options.include_dirs,
            );

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
            header_inclued_by_sources: header_to_sources_from_entry_point.borrow().clone(),
            source_include_headers: source_to_headers_from_entry_point.borrow().clone(),
        };
    }

    fn get_includes_from_source_files(
        parsed_files: &mut StringSet,
        source_dir: &String,
        include_dirs: &Vec<String>,
    ) -> (RcRefCellStringMapSet, RcRefCellStringMapSet) {
        let source_to_headers =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));
        let header_to_sources =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));

        for source_file in util::fs::find_source_files(source_dir) {
            Self::get_include_files_in_source_dir(
                parsed_files,
                &source_file,
                source_dir,
                source_to_headers.clone(),
                header_to_sources.clone(),
                include_dirs,
            );
        }

        return (source_to_headers, header_to_sources);
    }

    fn get_includes_from_entry_point(
        parsed_files: &mut StringSet,
        source_dir: &String,
        source_file: String,
        include_dirs: &Vec<String>,
    ) -> (RcRefCellStringMapSet, RcRefCellStringMapSet) {
        let source_to_headers =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));
        let header_to_sources =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));

        Self::get_include_files_in_source_dir(
            parsed_files,
            &source_file,
            source_dir,
            source_to_headers.clone(),
            header_to_sources.clone(),
            include_dirs,
        );

        return (source_to_headers, header_to_sources);
    }

    fn get_include_files_in_source_dir(
        parsed_files: &mut StringSet,
        source_file: &String,
        source_dir: &String,
        source_include_headers: RcRefCellStringMapSet,
        header_inclued_by_sources: RcRefCellStringMapSet,
        include_dirs: &Vec<String>,
    ) {
        // skip parsed
        if parsed_files.contains(source_file) {
            return;
        }
        parsed_files.insert(source_file.clone());

        if !source_include_headers.borrow().contains_key(source_file) {
            // new headers' container for map source to headers
            source_include_headers
                .borrow_mut()
                .insert(source_file.clone(), std::collections::BTreeSet::new());
        }

        for include in Self::get_include_files(source_file, source_dir) {
            // skip third-party
            if !include.starts_with(source_dir) {
                continue;
            }

            // map source to headers
            source_include_headers
                .borrow_mut()
                .get_mut(source_file)
                .unwrap()
                .insert(include.clone());

            let header_inclued_by_sources_cloned = header_inclued_by_sources.clone();
            if !header_inclued_by_sources_cloned.borrow().contains_key(&include) {
                // new headers' container for map header to sources
                header_inclued_by_sources_cloned.borrow_mut()
                    .insert(include.clone(), std::collections::BTreeSet::new());
            }
            // map header to sources
            header_inclued_by_sources_cloned.borrow_mut().get_mut(&include).unwrap().insert(source_file.clone());

            // recurse
            Self::get_include_files_in_source_dir(
                parsed_files,
                &include,
                source_dir,
                source_include_headers.clone(),
                header_inclued_by_sources.clone(),
                include_dirs,
            );
        }
    }

    fn get_include_files(source: &String, source_dir: &String) -> StringSet {
        let mut include_files = std::collections::BTreeSet::<String>::new();

        // create an index
        let index = unsafe { clang_sys::clang_createIndex(0, 0) };

        // parse the translation unit
        let translation_unit = unsafe {
            clang_sys::clang_parseTranslationUnit(
                index,
                Self::string_to_cstr(source).as_ptr(),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
                clang_sys::CXTranslationUnit_DetailedPreprocessingRecord
                    | clang_sys::CXTranslationUnit_SkipFunctionBodies
                    | clang_sys::CXTranslationUnit_SingleFileParse
                    | clang_sys::CXTranslationUnit_KeepGoing,
            )
        };
        if translation_unit.is_null() {
            tracing::info!("clang_sys::clang_parseTranslationUnit error");
            unsafe { clang_sys::clang_disposeIndex(index) };
            return include_files;
        }

        // get the cursor for the translation unit
        let cursor = unsafe { clang_sys::clang_getTranslationUnitCursor(translation_unit) };

        // visit the AST
        unsafe {
            clang_sys::clang_visitChildren(
                cursor,
                get_include_files,
                &mut include_files as *mut _ as *mut std::ffi::c_void,
            );
        }

        // clean up
        unsafe {
            clang_sys::clang_disposeTranslationUnit(translation_unit);
            clang_sys::clang_disposeIndex(index);
        }

        let prefix_length = source_dir.len() + 1;
        tracing::info!("{}", source.clone().split_off(prefix_length));
        for include in &include_files {
            if include.starts_with(source_dir) {
                tracing::info!("    {}", include.clone().split_off(prefix_length));
            }
        }

        return include_files;
    }

    fn string_to_cstr(rust_str: &String) -> std::ffi::CString {
        std::ffi::CString::new(rust_str.as_str()).unwrap()
    }
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

extern "C" fn get_include_files(
    cursor: clang_sys::CXCursor,
    _parent: clang_sys::CXCursor,
    client_data: clang_sys::CXClientData,
) -> clang_sys::CXChildVisitResult {
    if unsafe { clang_sys::clang_getCursorKind(cursor) } == clang_sys::CXCursor_InclusionDirective {
        let include_file = unsafe { clang_sys::clang_getIncludedFile(cursor) };
        if !include_file.is_null() {
            let include_file_name = unsafe { clang_sys::clang_getFileName(include_file) };
            let path = cxstring_to_string(include_file_name).replace(r"\", "/");

            let include_paths =
                unsafe { &mut *(client_data as *mut std::collections::BTreeSet<String>) };
            include_paths.insert(path);
        }
    }
    clang_sys::CXChildVisit_Recurse
}
