#pragma once

#ifndef BUILD_SHARED_LIBS_A
    #define A_API
#else
    #ifdef _WIN32
        #ifdef A_EXPORTS
            #define A_API __declspec(dllexport)
        #else
            #define A_API __declspec(dllimport)
        #endif
    #elif defined(__GNUC__) || defined(__clang__)
        #define A_API __attribute__((visibility("default")))
    #else
        #define A_API
    #endif
#endif


