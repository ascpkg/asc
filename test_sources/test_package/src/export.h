#pragma once

#ifndef BUILD_SHARED_LIBS_TEST_PACKAGE
    #define TEST_PACKAGE_API
#else
    #ifdef _WIN32
        #ifdef TEST_PACKAGE_EXPORTS
            #define TEST_PACKAGE_API __declspec(dllexport)
        #else
            #define TEST_PACKAGE_API __declspec(dllimport)
        #endif
    #elif defined(__GNUC__) || defined(__clang__)
        #define TEST_PACKAGE_API __attribute__((visibility("default")))
    #else
        #define TEST_PACKAGE_API
    #endif
#endif

