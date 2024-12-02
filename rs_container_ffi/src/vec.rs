use std::ffi::{CStr, CString};

/// wrap rust Vec<String> new for c
#[no_mangle]
pub extern "C" fn rust_vec_of_str_new() -> *mut Vec<String> {
    return Box::into_raw(Box::new(Vec::<String>::new()));
}

/// wrap rust Vec<String> drop for c
#[no_mangle]
pub extern "C" fn rust_vec_of_str_drop(p: *mut Vec<String>) {
    if p.is_null() {
        return;
    }

    unsafe {
        let _ = Box::from_raw(p); // This will drop and free the memory
    }
}

/// wrap rust Vec<String> push for c
#[no_mangle]
pub extern "C" fn rust_vec_of_str_push(instance: *mut Vec<String>, value: *const std::ffi::c_char) {
    if instance.is_null() {
        return;
    }

    let value = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };

    let vector = unsafe { &mut *instance };
    vector.push(value);
}

/// wrap rust Vec<String> reverse for c
#[no_mangle]
pub extern "C" fn rust_vec_of_str_reverse(instance: *mut Vec<String>) {
    if instance.is_null() {
        return;
    }

    let vector = unsafe { &mut *instance };
    vector.reverse();
}

/// wrap rust Vec<String> join for c
/// must use rust_c_str_drop to free return value
#[no_mangle]
pub extern "C" fn rust_vec_of_str_join(instance: *mut Vec<String>, sep: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    if instance.is_null() {
        return std::ptr::null_mut();
    }

    let sep = unsafe { CStr::from_ptr(sep).to_string_lossy().into_owned() };

    let vector = unsafe { &mut *instance };
    let result = vector.join(&sep);

    let c_str = CString::new(result.clone()).unwrap();
    let ptr = c_str.into_raw();
    return ptr;
}
