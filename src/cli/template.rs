pub static NEW_BIN_HBS: &str = r#"#include <iostream>
#include "config.h"
#include "version.h"


int main(int argc, char **argv) {
    std::cout << "Hello, world!" << std::endl;;
}
"#;

pub static NEW_LIB_HDR_HBS: &str = r#"#include "export.h"

#ifdef __cplusplus
extern "C" {
#endif

int {{project_upper}}_API test(int left, int right);

#ifdef __cplusplus
}
#endif

class {{project_upper}}_API Test
{
public:
    int add(int left, int right);
};

"#;

pub static NEW_LIB_MAIN_HBS: &str = r#"#include "lib.hpp"
#include "config.h"
#include "version.h"


int test(int left, int right) {
    return left + right;
}


int Test::add(int left, int right) {
    return left + right;
}


int main(int argc, char **argv) {
    int a = test(1, 2);
    int b = Test().add(1, 2);
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
