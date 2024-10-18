#pragma once

#ifndef BUILD_SHARED_LIBS
    #define TEST_C_CPP_API
#else
    #ifdef _WIN32
        #ifdef TEST_C_CPP_EXPORTS
            #define TEST_C_CPP_API __declspec(dllexport)
        #else
            #define TEST_C_CPP_API __declspec(dllimport)
        #endif
    #elif defined(__GNUC__) || defined(__clang__)
        #define TEST_C_CPP_API __attribute__((visibility("default")))
    #else
        #define TEST_C_CPP_API
    #endif
#endif

