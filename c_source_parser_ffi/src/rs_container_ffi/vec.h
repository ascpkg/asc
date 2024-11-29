#ifndef RS_CONTAINER_FFI_VEC_API
#define RS_CONTAINER_FFI_VEC_API

// bindings of rs_container_ffi/vec.rs

// rust Vec<String>
typedef void *RustVecOfStr;

// rust Vec<String>
extern RustVecOfStr rust_vec_of_str_new();
extern void rust_vec_of_str_drop(RustVecOfStr vec);
extern void rust_vec_of_str_push(RustVecOfStr vec, const char *value);
extern void rust_vec_of_str_reverse(RustVecOfStr vec);
extern char *rust_vec_of_str_join(RustVecOfStr vec, const char *sep);

#endif // RS_CONTAINER_FFI_VEC_API
