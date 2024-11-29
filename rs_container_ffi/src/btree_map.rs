use std::collections::{BTreeMap, BTreeSet};
use std::ffi::CStr;

/// wrap rust BTreeMap<String, BTreeSet<String>> new for c
#[no_mangle]
pub extern "C" fn rust_btree_map_of_str_set_new() -> *mut BTreeMap<String, BTreeSet<String>> {
    return Box::into_raw(Box::new(BTreeMap::new()));
}

/// wrap rust BTreeMap<String, BTreeSet<String>> drop for c
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

/// wrap rust BTreeMap<String, BTreeSet<String>> insert for c
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
