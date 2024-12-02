use std::ffi::CString;

/// wrap rust CString dop for c
#[no_mangle]
pub extern "C" fn rust_c_str_drop(s: *mut std::ffi::c_char) {
    if s.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(s); // This will drop and free the memory
    };
}
