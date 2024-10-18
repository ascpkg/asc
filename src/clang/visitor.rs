use clang_sys;

type StringSet = std::collections::BTreeSet<String>;

pub fn get_include_files(source: &String, source_dir: &String) -> StringSet {
    let mut include_files = std::collections::BTreeSet::<String>::new();

    // create an index
    let index = unsafe { clang_sys::clang_createIndex(0, 0) };

    // parse the translation unit
    let translation_unit = unsafe {
        clang_sys::clang_parseTranslationUnit(
            index,
            string_to_cstr(source).as_ptr(),
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
            visit_inclusion_directive,
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
