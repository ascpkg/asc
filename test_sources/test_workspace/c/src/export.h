#pragma once

#ifndef BUILD_SHARED_LIBS_C
    #define C_API
#else
    #ifdef _WIN32
        #ifdef C_EXPORTS
            #define C_API __declspec(dllexport)
        #else
            #define C_API __declspec(dllimport)
        #endif
    #elif defined(__GNUC__) || defined(__clang__)
        #define C_API __attribute__((visibility("default")))
    #else
        #define C_API
    #endif
#endif


