use std::collections::BTreeSet;
use std::ffi::CStr;

/// wrap rust BTreeSet<String> new for c
#[no_mangle]
pub extern "C" fn rust_btree_set_of_str_new() -> *mut BTreeSet<String> {
    return Box::into_raw(Box::new(BTreeSet::<String>::new()));
}

/// wrap rust BTreeSet<String> drop for c
#[no_mangle]
pub extern "C" fn rust_btree_set_of_str_drop(instance: *mut BTreeSet<String>) {
    if instance.is_null() {
        return;
    }

    unsafe {
        let _ = Box::from_raw(instance); // This will drop and free the memory
    }
}

/// wrap rust BTreeSet<String> insert for c
#[no_mangle]
pub extern "C" fn rust_btree_set_of_str_insert(instance: *mut BTreeSet<String>, value: *const std::ffi::c_char) {
    if instance.is_null() {
        return;
    }

    let value = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };

    let set = unsafe { &mut *instance };
    set.insert(value);
}

/// wrap rust BTreeSet<String> contains for c
#[no_mangle]
pub extern "C" fn rust_btree_set_of_str_contains(
    instance: *mut BTreeSet<String>,
    value: *const std::ffi::c_char,
) -> i32 {
    if instance.is_null() {
        return 0;
    }

    let value = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };

    let set = unsafe { &mut *instance };
    return if set.contains(&value) { 1 } else { 0 };
}
