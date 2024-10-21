pub static NEW_BIN_HBS: &str = r#"#include <iostream>

int main(int argc, char **argv) {
    std::cout << "Hello, world!" << std::endl;;
}
"#;

pub static NEW_LIB_MAIN_HBS: &str = r#"#include "export.h"

int {{project_upper}}_API test(int left, int right);

class {{project_upper}}_API Test
{
public:
    int add(int left, int right);
};


int test(int left, int right) {
    return left + right;
}


int Test::add(int left, int right) {
    return left + right;
}
"#;

pub static NEW_LIB_EXPORT_HBS: &str = r#"#pragma once

#ifndef BUILD_SHARED_LIBS
    #define {{project_upper}}_API
#else
    #ifdef _WIN32
        #ifdef {{project_upper}}_EXPORTS
            #define {{project_upper}}_API __declspec(dllexport)
        #else
            #define {{project_upper}}_API __declspec(dllimport)
        #endif
    #elif defined(__GNUC__) || defined(__clang__)
        #define {{project_upper}}_API __attribute__((visibility("default")))
    #else
        #define {{project_upper}}_API
    #endif
#endif


"#;
