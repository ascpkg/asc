#ifndef RS_CONTAINER_FFI_BTREE_MAP_API
#define RS_CONTAINER_FFI_BTREE_MAP_API

// bindings of rs_container_ffi/btree_map.rs

// rust BTreeMap<String, BTreeSet<String>>
typedef void *RustBtreeMapOfStrSet;

// rust BTreeMap<String, BTreeSet<String>>
extern RustBtreeMapOfStrSet rust_btree_map_of_str_set_new();
extern void rust_btree_map_of_str_set_drop(RustBtreeMapOfStrSet instance);
extern void rust_btree_map_of_str_set_insert(RustBtreeMapOfStrSet instance, const char *key, const char *value);

#endif // RS_CONTAINER_FFI_BTREE_MAP_API
