use clang_sys;

pub fn string_to_c_str(rust_str: &String) -> std::ffi::CString {
    std::ffi::CString::new(rust_str.as_str()).unwrap()
}

pub fn cx_string_to_string(cx_str: clang_sys::CXString) -> String {
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

pub fn get_location_info(cursor: clang_sys::CXCursor) -> (String, u32, u32) {
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

pub fn get_namespace(cursor: clang_sys::CXCursor) -> String {
    unsafe {
        let mut parent_cursor = clang_sys::clang_getCursorSemanticParent(cursor);
        let mut namespaces = Vec::new();

        while clang_sys::clang_Cursor_isNull(parent_cursor) != 0 {
            if clang_sys::clang_getCursorKind(parent_cursor) == clang_sys::CXCursor_Namespace {
                namespaces.push(cx_string_to_string(clang_sys::clang_getCursorSpelling(
                    parent_cursor,
                )));
            }
            parent_cursor = clang_sys::clang_getCursorSemanticParent(parent_cursor);
        }

        namespaces.reverse();
        namespaces.join("::")
    }
}

pub fn get_class_name(cursor: clang_sys::CXCursor) -> String {
    unsafe {
        let mut parent_cursor = clang_sys::clang_getCursorSemanticParent(cursor);

        while clang_sys::clang_getCursorKind(parent_cursor) != clang_sys::CXCursor_ClassDecl {
            parent_cursor = clang_sys::clang_getCursorSemanticParent(parent_cursor);
        }

        cx_string_to_string(clang_sys::clang_getCursorSpelling(parent_cursor))
    }
}
