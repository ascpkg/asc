# Auto build source tree
- scan source dependences with clang ir
- output mermaid flowchat of source dependences

  for example: [test_c_cpp](./test_c_cpp.md)

- output CMakeLists.txt
- generate a build system with cmake
- build with cmake

# Usage
```
Usage: auto_build_source_tree.exe [OPTIONS] --project <PROJECT> --source-dir <SOURCE_DIR> --entry-point-source <ENTRY_POINT_SOURCE>

Options:
      --project <PROJECT>
      --source-dir <SOURCE_DIR>
      --entry-point-source <ENTRY_POINT_SOURCE>
      --include-dirs <INCLUDE_DIRS>              [default: ]
  -h, --help                                     Print help
  -V, --version                                  Print version
```

# Test
```
> .\target\debug\auto_build_source_tree.exe --source-dir=test_c_cpp/src --entry-point-source=main.cpp --project=test_c_cpp
2024-10-18 14:31:28.5545891  WARN auto_build_source_tree: 23: parse command lines
2024-10-18 14:31:28.5559923  INFO auto_build_source_tree: 26: CommandLines {
    project: "test_c_cpp",
    source_dir: "D:/__develop__/FutureOrientedGB/auto_build_source_tree/test_c_cpp/src",
    entry_point_source: "main.cpp",
    include_dirs: [],
}
2024-10-18 14:31:28.5562664  WARN auto_build_source_tree: 28: scan source dependences with clang ir
2024-10-18 14:31:28.5786256  INFO auto_build_source_tree::clang::parser: 220: main.cpp
2024-10-18 14:31:28.578905  INFO auto_build_source_tree::clang::parser: 223:     wrapping.hpp
2024-10-18 14:31:28.5878457  INFO auto_build_source_tree::clang::parser: 220: wrapping.hpp
2024-10-18 14:31:28.5883983  INFO auto_build_source_tree::clang::parser: 223:     a/mod.hpp
2024-10-18 14:31:28.5888263  INFO auto_build_source_tree::clang::parser: 223:     b/mod.hpp
2024-10-18 14:31:28.5891081  INFO auto_build_source_tree::clang::parser: 223:     c/mod.hpp
2024-10-18 14:31:28.5990728  INFO auto_build_source_tree::clang::parser: 220: a/mod.hpp
2024-10-18 14:31:28.5993449  INFO auto_build_source_tree::clang::parser: 223:     a/a.h
2024-10-18 14:31:28.599703  INFO auto_build_source_tree::clang::parser: 223:     a/a.hpp
2024-10-18 14:31:28.6123635  INFO auto_build_source_tree::clang::parser: 220: a/a.h
2024-10-18 14:31:28.6307098  INFO auto_build_source_tree::clang::parser: 220: a/a.hpp
2024-10-18 14:31:28.6393058  INFO auto_build_source_tree::clang::parser: 220: b/mod.hpp
2024-10-18 14:31:28.6395281  INFO auto_build_source_tree::clang::parser: 223:     b/b.h
2024-10-18 14:31:28.6396076  INFO auto_build_source_tree::clang::parser: 223:     b/b.hpp
2024-10-18 14:31:28.6487124  INFO auto_build_source_tree::clang::parser: 220: b/b.h
2024-10-18 14:31:28.6608193  INFO auto_build_source_tree::clang::parser: 220: b/b.hpp
2024-10-18 14:31:28.6711501  INFO auto_build_source_tree::clang::parser: 220: c/mod.hpp
2024-10-18 14:31:28.6717795  INFO auto_build_source_tree::clang::parser: 223:     c/c.h
2024-10-18 14:31:28.6721487  INFO auto_build_source_tree::clang::parser: 223:     c/c.hpp
2024-10-18 14:31:28.6833734  INFO auto_build_source_tree::clang::parser: 220: c/c.h
2024-10-18 14:31:28.6967625  INFO auto_build_source_tree::clang::parser: 220: c/c.hpp
2024-10-18 14:31:28.707241  INFO auto_build_source_tree::clang::parser: 220: a/a.c
2024-10-18 14:31:28.707524  INFO auto_build_source_tree::clang::parser: 223:     a/a.h
2024-10-18 14:31:28.7168508  INFO auto_build_source_tree::clang::parser: 220: a/a.cpp
2024-10-18 14:31:28.7170384  INFO auto_build_source_tree::clang::parser: 223:     a/a.hpp
2024-10-18 14:31:28.7304393  INFO auto_build_source_tree::clang::parser: 220: b/b.c
2024-10-18 14:31:28.730837  INFO auto_build_source_tree::clang::parser: 223:     b/b.h
2024-10-18 14:31:28.7411335  INFO auto_build_source_tree::clang::parser: 220: b/b.cpp
2024-10-18 14:31:28.741367  INFO auto_build_source_tree::clang::parser: 223:     b/b.hpp
2024-10-18 14:31:28.7527889  INFO auto_build_source_tree::clang::parser: 220: c/c.c
2024-10-18 14:31:28.7530691  INFO auto_build_source_tree::clang::parser: 223:     c/c.h
2024-10-18 14:31:28.7629876  INFO auto_build_source_tree::clang::parser: 220: c/c.cpp
2024-10-18 14:31:28.7634873  INFO auto_build_source_tree::clang::parser: 223:     c/c.hpp
2024-10-18 14:31:28.7745575  INFO auto_build_source_tree::clang::parser: 220: d/d.c
2024-10-18 14:31:28.7748593  INFO auto_build_source_tree::clang::parser: 223:     d/d.h
2024-10-18 14:31:28.7842503  INFO auto_build_source_tree::clang::parser: 220: d/d.h
2024-10-18 14:31:28.7948039  INFO auto_build_source_tree::clang::parser: 220: d/d.cc
2024-10-18 14:31:28.7950844  INFO auto_build_source_tree::clang::parser: 223:     d/d.hpp
2024-10-18 14:31:28.8067211  INFO auto_build_source_tree::clang::parser: 220: d/d.hpp
2024-10-18 14:31:28.8162067  INFO auto_build_source_tree::clang::parser: 220: test.cpp
2024-10-18 14:31:28.8163379  INFO auto_build_source_tree::clang::parser: 223:     d/mod.hpp
2024-10-18 14:31:28.8286772  INFO auto_build_source_tree::clang::parser: 220: d/mod.hpp
2024-10-18 14:31:28.8290074  INFO auto_build_source_tree::clang::parser: 223:     d/d.h
2024-10-18 14:31:28.8293365  INFO auto_build_source_tree::clang::parser: 223:     d/d.hpp
2024-10-18 14:31:28.8300045  WARN auto_build_source_tree: 31: output mermaid flowchat of source dependences
2024-10-18 14:31:28.830385  INFO auto_build_source_tree: 34: flowchart LR;
    a/a.c ---> a/a.h;
    a/mod.hpp ---> a/a.h;
    a/a.cpp ---> a/a.hpp;
    a/mod.hpp ---> a/a.hpp;
    wrapping.hpp ---> a/mod.hpp;
    b/b.c ---> b/b.h;
    b/mod.hpp ---> b/b.h;
    b/b.cpp ---> b/b.hpp;
    b/mod.hpp ---> b/b.hpp;
    wrapping.hpp ---> b/mod.hpp;
    c/c.c ---> c/c.h;
    c/mod.hpp ---> c/c.h;
    c/c.cpp ---> c/c.hpp;
    c/mod.hpp ---> c/c.hpp;
    wrapping.hpp ---> c/mod.hpp;
    main.cpp ---> wrapping.hpp;

2024-10-18 14:31:28.831687  WARN auto_build_source_tree: 47: output CMakeLists.txt
2024-10-18 14:31:28.8325096  WARN auto_build_source_tree: 51: generate a build system with cmake
-- Building for: Visual Studio 17 2022
-- Selecting Windows SDK version 10.0.18362.0 to target Windows 10.0.22631.
-- The C compiler identification is MSVC 19.41.34120.0
-- The CXX compiler identification is MSVC 19.41.34120.0
-- Detecting C compiler ABI info
-- Detecting C compiler ABI info - done
-- Check for working C compiler: D:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.41.34120/bin/Hostx64/x64/cl.exe - skipped
-- Detecting C compile features
-- Detecting C compile features - done
-- Detecting CXX compiler ABI info
-- Detecting CXX compiler ABI info - done
-- Check for working CXX compiler: D:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.41.34120/bin/Hostx64/x64/cl.exe - skipped
-- Detecting CXX compile features
-- Detecting CXX compile features - done
-- Configuring done (3.1s)
-- Generating done (0.0s)
-- Build files have been written to: D:/__develop__/FutureOrientedGB/auto_build_source_tree/test_c_cpp/build
2024-10-18 14:31:32.0142558  WARN auto_build_source_tree: 55: build with cmake
MSBuild version 17.11.9+a69bbaaf5 for .NET Framework

  1>Checking Build System
  Building Custom Rule D:/__develop__/FutureOrientedGB/auto_build_source_tree/test_c_cpp/CMakeLists.txt
  a.c
  a.cpp
  b.c
  b.cpp
  c.c
  c.cpp
  main.cpp
  test_c_cpp.vcxproj -> D:\__develop__\FutureOrientedGB\auto_build_source_tree\test_c_cpp\build\Debug\test_c_cpp.exe
  Building Custom Rule D:/__develop__/FutureOrientedGB/auto_build_source_tree/test_c_cpp/CMakeLists.txt
```