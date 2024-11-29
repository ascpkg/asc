#ifndef C_SOURCE_PARSER_FFI_DYLIB_API
#define C_SOURCE_PARSER_FFI_DYLIB_API

#define INVALID_DYLIB_HANDLE ((void *)0)

typedef void *dylib_handle;
typedef void *dylib_symbol;

dylib_handle dylib_open(const char *path);
void dylib_close(dylib_handle handle);

int dylib_has(dylib_handle handle, const char *name);
dylib_symbol dylib_get(dylib_handle handle, const char *name);

const char *dylib_error();

#endif // C_SOURCE_PARSER_FFI_DYLIB_API
