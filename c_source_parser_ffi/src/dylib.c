// self
#include "dylib.h"

// c
#include <string.h>

// platforms
#if (defined(_WIN32) || defined(_WIN64))
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#define DYLIB_WIN32_LEAN_AND_MEAN
#endif
#include <windows.h>
#ifdef DYLIB_UNDEFINE_NOMINMAX
#undef WIN32_LEAN_AND_MEAN
#undef DYLIB_UNDEFINE_LEAN_AND_MEAN
#endif
#else
#include <dlfcn.h>
#endif

#ifndef NULL
#define NULL ((void *)0)
#endif


dylib_handle dylib_open(const char *path) {
    if (NULL == path || 0 == strlen(path)) {
        return INVALID_DYLIB_HANDLE;
    }

#if (defined(_WIN32) || defined(_WIN64))
    return (dylib_handle)LoadLibraryA(path);
#else
    return dlopen(path, RTLD_NOW | RTLD_LOCAL);
#endif
}

void dylib_close(dylib_handle handle) {
    if (INVALID_DYLIB_HANDLE == handle) {
        return;
    }

#if (defined(_WIN32) || defined(_WIN64))
    FreeLibrary((HMODULE)handle);
#else
    dlclose(handle);
#endif
}

int dylib_has(dylib_handle handle, const char *name) {
    return INVALID_DYLIB_HANDLE == dylib_get(handle, name) ? 1 : 0;
}

dylib_symbol dylib_get(dylib_handle handle, const char *name) {
    if (INVALID_DYLIB_HANDLE == handle || NULL == name || 0 == strlen(name)) {
        return INVALID_DYLIB_SYMBOL;
    }

#if (defined(_WIN32) || defined(_WIN64))
    return GetProcAddress((HMODULE)handle, name);
#else
    return dlsym(handle, name);
#endif
}

static char error_text[512];
const char *dylib_error() {
#if (defined(_WIN32) || defined(_WIN64))
        const DWORD error_code = GetLastError();
        if (0 == error_code) {
            return "No error reported by GetLastError";
        }
        const DWORD length = FormatMessageA(FORMAT_MESSAGE_FROM_SYSTEM, NULL, error_code, MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US), error_text, sizeof(error_text), NULL);
        return (length == 0) ? "Unknown error (FormatMessage failed)" : error_text;
#else
        const char *error_text = dlerror();
        return (NULL == error_text) ? "No error reported by dlerror" : error_text;
#endif
}
