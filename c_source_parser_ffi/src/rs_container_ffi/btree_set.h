#ifndef RS_CONTAINER_FFI_BTREE_SET_API
#define RS_CONTAINER_FFI_BTREE_SET_API

// bindings of rs_container_ffi/btree_set.rs

// rust BTreeSet<String>
typedef void *RustBtreeSetOfStr;

// rust BTreeSet<String>
extern RustBtreeSetOfStr rust_btree_set_of_str_new();
extern void rust_btree_set_of_str_drop(RustBtreeSetOfStr instance);
extern int rust_btree_set_of_str_contains(RustBtreeSetOfStr instance, const char *value);
extern void rust_btree_set_of_str_insert(RustBtreeSetOfStr instance, const char *value);

#endif // RS_CONTAINER_FFI_BTREE_SET_API
