use std::error::Error;

use handlebars::Handlebars;
use serde_json::json;

fn main() -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();

    let template = std::fs::read_to_string("cmake_lists.hbs").unwrap();

    println!(
        "{}",
        reg.render_template(
            &template,
            &json!({
                "cmake_version": "3.20",
                "project": "test_package",
                "executable": false,
                "library": true,
                "project_exports": "TEST_PACKAGE_EXPORTS",
                "sources_group_by_dir": [
                    {
                        "dir": "src",
                        "variable": "${src}",
                        "files": [
                            "src/main.cpp",
                            "src/wrapping.hpp"
                        ]
                    },
                    {
                        "dir": "src_a",
                        "variable": "${src_a}",
                        "files": [
                            "src/a/a.c",
                            "src/a/a.cpp",
                            "src/a/a.h",
                            "src/a/a.hpp",
                            "src/a/mod.hpp"
                        ]
                    },
                    {
                        "dir": "src_a_..",
                        "variable": "${src_a_..}",
                        "files": [
                            "src/a/../export.h"
                        ]
                    },
                    {
                        "dir": "src_b",
                        "variable": "${src_b}",
                        "files": [
                            "src/b/b.c",
                            "src/b/b.cpp",
                            "src/b/b.h",
                            "src/b/b.hpp",
                            "src/b/mod.hpp"
                        ]
                    },
                    {
                        "dir": "src_b_..",
                        "variable": "${src_b_..}",
                        "files": [
                            "src/b/../export.h"
                        ]
                    },
                    {
                        "dir": "src_c",
                        "variable": "${src_c}",
                        "files": [
                            "src/c/c.c",
                            "src/c/c.cpp",
                            "src/c/c.h",
                            "src/c/c.hpp",
                            "src/c/mod.hpp"
                        ]
                    },
                    {
                        "dir": "src_c_..",
                        "variable": "${src_c_..}",
                        "files": [
                            "src/c/../export.h"
                        ]
                    },
                ],
                "link_libraries": true,
                "link_public_libraries": false,
                "public_libraries": [],
                "link_private_libraries": true,
                "private_libraries": [
                    "fmt::fmt",
                    "spdlog::spdlog"
                ],
                "install_headers": [
                    {"src": "src/a/../export.h", "dst": "a/../"},
                    {"src": "src/a/a.h", "dst": "a/"},
                    {"src": "src/a/a.hpp", "dst": "a/"},
                    {"src": "src/a/mod.hpp", "dst": "a/"},
                    {"src": "src/b/../export.h", "dst": "b/../"},
                    {"src": "src/b/b.h", "dst": "b/"},
                    {"src": "src/b/b.hpp", "dst": "b/"},
                    {"src": "src/b/mod.hpp", "dst": "b/"},
                    {"src": "src/c/../export.h", "dst": "c/../"},
                    {"src": "src/c/c.h", "dst": "c/"},
                    {"src": "src/c/c.hpp", "dst": "c/"},
                    {"src": "src/c/mod.hpp", "dst": "c/"},
                    {"src": "src/wrapping.hpp", "dst": ""},
                ]
            })
        )?
    );

    Ok(())
}
