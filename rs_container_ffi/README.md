# Call Rust Vec, std::collections::{BTreeMap, BTreeSet} in C

[![Docs](https://docs.rs/rs_container_ffi/badge.svg)](https://docs.rs/rs_container_ffi)
[![Crates.io](https://img.shields.io/crates/d/rs_container_ffi.svg)](https://crates.io/crates/rs_container_ffi)
[![Crates.io](https://img.shields.io/crates/v/rs_container_ffi.svg)](https://crates.io/crates/rs_container_ffi)



```c
// copy ../c_source_parser_ffi/src/rs_container_ffi/btree_map.h
// full example can be found at ../c_source_parser_ffi

// use rust BTreeMap<String, BTreeSet<String>>
// new BTreeMap<String, BTreeSet<String>>
RustBtreeMapOfStrSet map = rust_btree_set_of_str_new();
// insert key and value to BTreeMap<String, BTreeSet<String>>
rust_btree_set_of_str_insert(map, "key", "value");
// drop BTreeMap<String, BTreeSet<String>>
rust_btree_set_of_str_drop(map);

// use rust BTreeSet<String>
// new BTreeSet<String>
RustBtreeSetOfStr set = rust_btree_set_of_str_new();
// insert foo to BTreeSet<String>
rust_btree_set_of_str_insert(set, "foo");
// insert bar to BTreeSet<String>
rust_btree_set_of_str_insert(set, "bar");
// if foo in BTreeSet<String> return 1 else 0
int is_foo_exists = rust_btree_set_of_str_contains(set, "foo");
// drop BTreeSet<String>
rust_btree_set_of_str_drop(set);

// use rust Vec<String>
// new Vec<String>
RustVecOfStr vec = rust_vec_of_str_new();
// push a to Vec<String>
rust_vec_of_str_push(vec, "a");
// push b to Vec<String>
rust_vec_of_str_push(vec, "b");
// push c to Vec<String>
rust_vec_of_str_push(vec, "c");
// push d to Vec<String>
rust_vec_of_str_push(vec, "d");
// reverse Vec<String>
rust_vec_of_str_reverse(vec);
// join Vec<String>
const char *text = rust_vec_of_str_join("/");  // d/c/b/c
// drop Vec<String>
rust_vec_of_str_drop(vec)
// drop text (do not use c free or c++ delete)
rust_c_str_drop(text);
```
