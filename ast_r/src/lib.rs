use std::collections::{BTreeMap, BTreeSet};
use std::ffi::{CStr, CString};

///////////////////////////////////////////////////////////////////////////////////////////////////
/// wrap rust BTreeMap<String, BTreeSet<String>> for c
#[no_mangle]
pub extern "C" fn rust_btree_map_of_str_set_new() -> *mut BTreeMap<String, BTreeSet<String>> {
    return Box::into_raw(Box::new(BTreeMap::new()));
}

#[no_mangle]
pub extern "C" fn rust_btree_map_of_str_set_drop(
    instance: *mut BTreeMap<String, BTreeSet<String>>,
) {
    if instance.is_null() {
        return;
    }

    unsafe {
        let _ = Box::from_raw(instance); // This will drop and free the memory
    }
}

#[no_mangle]
pub extern "C" fn rust_btree_map_of_str_set_insert(
    instance: *mut BTreeMap<String, BTreeSet<String>>,
    key: *const i8,
    value: *const i8,
) {
    if instance.is_null() {
        return;
    }

    let k = unsafe { CStr::from_ptr(key).to_string_lossy().into_owned() };
    let v = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };

    let map = unsafe { &mut *instance };
    map.entry(k).or_insert_with(BTreeSet::new).insert(v);
}
///////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////
/// wrap rust BTreeSet<String> for c
#[no_mangle]
pub extern "C" fn rust_btree_set_of_str_new() -> *mut BTreeSet<String> {
    return Box::into_raw(Box::new(BTreeSet::<String>::new()));
}

#[no_mangle]
pub extern "C" fn rust_btree_set_of_str_drop(instance: *mut BTreeSet<String>) {
    if instance.is_null() {
        return;
    }

    unsafe {
        let _ = Box::from_raw(instance); // This will drop and free the memory
    }
}

#[no_mangle]
pub extern "C" fn rust_btree_set_of_str_insert(instance: *mut BTreeSet<String>, value: *const i8) {
    if instance.is_null() {
        return;
    }

    let value = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };

    let set = unsafe { &mut *instance };
    set.insert(value);
}

#[no_mangle]
pub extern "C" fn rust_btree_set_of_str_contains(
    instance: *mut BTreeSet<String>,
    value: *const i8,
) -> i32 {
    if instance.is_null() {
        return 0;
    }

    let value = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };

    let set = unsafe { &mut *instance };
    return if set.contains(&value) { 1 } else { 0 };
}
///////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////
/// wrap rust Vec<String> for c
#[no_mangle]
pub extern "C" fn rust_vec_of_str_new() -> *mut Vec<String> {
    return Box::into_raw(Box::new(Vec::<String>::new()));
}

#[no_mangle]
pub extern "C" fn rust_vec_of_str_drop(p: *mut Vec<String>) {
    if p.is_null() {
        return;
    }

    unsafe {
        let _ = Box::from_raw(p); // This will drop and free the memory
    }
}

#[no_mangle]
pub extern "C" fn rust_vec_of_str_push(instance: *mut Vec<String>, value: *const i8) {
    if instance.is_null() {
        return;
    }

    let value = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };

    let vector = unsafe { &mut *instance };
    vector.push(value);
}

#[no_mangle]
pub extern "C" fn rust_vec_of_str_reverse(instance: *mut Vec<String>) {
    if instance.is_null() {
        return;
    }

    let vector = unsafe { &mut *instance };
    vector.reverse();
}

#[no_mangle]
pub extern "C" fn rust_vec_of_str_join(instance: *mut Vec<String>, sep: *const i8) -> *mut i8 {
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
///////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////
/// wrap rust String for c
#[no_mangle]
pub extern "C" fn rust_c_str_drop(s: *mut i8) {
    if s.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(s); // This will drop and free the memory
    };
}
///////////////////////////////////////////////////////////////////////////////////////////////////
