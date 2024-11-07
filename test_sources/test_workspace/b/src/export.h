#pragma once

#ifndef BUILD_SHARED_LIBS
    #define B_API
#else
    #ifdef _WIN32
        #ifdef B_EXPORTS
            #define B_API __declspec(dllexport)
        #else
            #define B_API __declspec(dllimport)
        #endif
    #elif defined(__GNUC__) || defined(__clang__)
        #define B_API __attribute__((visibility("default")))
    #else
        #define B_API
    #endif
#endif


