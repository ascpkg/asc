# Auto build source tree
- scan source dependences with clang ir
- output mermaid flowchat of source dependences

  for example: [test_c_cpp](./test_c_cpp.md)

- output CMakeLists.txt
- generate a build system with cmake
- build with cmake

# Usage
```
Usage: auto_build_source_tree.exe [OPTIONS] --source-dir <SOURCE_DIR> --entry-point-source <ENTRY_POINT_SOURCE>

Options:
      --action-type <ACTION_TYPE>
          [default: scan] [possible values: all, scan, configure, build]
      --project <PROJECT>
          default to parent dir name of source_dir [default: ]
      --project-dir <PROJECT_DIR>
          default to parent dir of source_dir [default: ]
      --build-dir <BUILD_DIR>
          default to project_dir/build [default: ]
      --source-dir <SOURCE_DIR>
          source_dir must have a src subdir
      --entry-point-source <ENTRY_POINT_SOURCE>

      --include-dirs <INCLUDE_DIRS>
          [default: ]
      --cmake-minimum-version <CMAKE_MINIMUM_VERSION>
          [default: 3.20]
      --cmake-target-type <CMAKE_TARGET_TYPE>
          [default: executable] [possible values: executable, library]
      --cmake-lib-type <CMAKE_LIB_TYPE>
          [default: static] [possible values: static, shared]
      --cmake-config <CMAKE_CONFIG>
          [default: debug] [possible values: debug, release]
  -h, --help
          Print help
  -V, --version
          Print version
```

# Build test_c_cpp example
```
> .\target\debug\auto_build_source_tree.exe --source-dir=test_c_cpp/src --entry-point-source=main.cpp --action-type=all
2024-10-18 17:57:24.1574238  WARN auto_build_source_tree: 25: parse command lines
2024-10-18 17:57:24.1589353  INFO auto_build_source_tree: 28: CommandLines {
    action_type: All,
    project: "test_c_cpp",
    project_dir: "D:/__develop__/FutureOrientedGB/auto_build_source_tree/test_c_cpp",
    build_dir: "D:/__develop__/FutureOrientedGB/auto_build_source_tree/test_c_cpp/build",
    source_dir: "D:/__develop__/FutureOrientedGB/auto_build_source_tree/test_c_cpp/src",
    entry_point_source: "main.cpp",
    include_dirs: [],
    cmake_minimum_version: "3.20",
    cmake_target_type: Executable,
    cmake_lib_type: Static,
    cmake_config: Debug,
}
2024-10-18 17:57:24.1593385  WARN auto_build_source_tree: 34: scan source dependences with clang ir
2024-10-18 17:57:24.1770021  INFO auto_build_source_tree::clang::visitor: 51: main.cpp
2024-10-18 17:57:24.177632  INFO auto_build_source_tree::clang::visitor: 54:     wrapping.hpp
2024-10-18 17:57:24.1886352  INFO auto_build_source_tree::clang::visitor: 51: wrapping.hpp
2024-10-18 17:57:24.1889601  INFO auto_build_source_tree::clang::visitor: 54:     a/mod.hpp
2024-10-18 17:57:24.1892897  INFO auto_build_source_tree::clang::visitor: 54:     b/mod.hpp
2024-10-18 17:57:24.1897513  INFO auto_build_source_tree::clang::visitor: 54:     c/mod.hpp
2024-10-18 17:57:24.1985675  INFO auto_build_source_tree::clang::visitor: 51: a/mod.hpp
2024-10-18 17:57:24.1990279  INFO auto_build_source_tree::clang::visitor: 54:     a/a.h
2024-10-18 17:57:24.1993075  INFO auto_build_source_tree::clang::visitor: 54:     a/a.hpp
2024-10-18 17:57:24.2113653  INFO auto_build_source_tree::clang::visitor: 51: a/a.h
2024-10-18 17:57:24.2237308  INFO auto_build_source_tree::clang::visitor: 51: a/a.hpp
2024-10-18 17:57:24.2329265  INFO auto_build_source_tree::clang::visitor: 51: b/mod.hpp
2024-10-18 17:57:24.2331731  INFO auto_build_source_tree::clang::visitor: 54:     b/b.h
2024-10-18 17:57:24.2333192  INFO auto_build_source_tree::clang::visitor: 54:     b/b.hpp
2024-10-18 17:57:24.2420526  INFO auto_build_source_tree::clang::visitor: 51: b/b.h
2024-10-18 17:57:24.2516392  INFO auto_build_source_tree::clang::visitor: 51: b/b.hpp
2024-10-18 17:57:24.2622641  INFO auto_build_source_tree::clang::visitor: 51: c/mod.hpp
2024-10-18 17:57:24.2627946  INFO auto_build_source_tree::clang::visitor: 54:     c/c.h
2024-10-18 17:57:24.2631111  INFO auto_build_source_tree::clang::visitor: 54:     c/c.hpp
2024-10-18 17:57:24.2740094  INFO auto_build_source_tree::clang::visitor: 51: c/c.h
2024-10-18 17:57:24.2856737  INFO auto_build_source_tree::clang::visitor: 51: c/c.hpp
2024-10-18 17:57:24.2961053  INFO auto_build_source_tree::clang::visitor: 51: a/a.c
2024-10-18 17:57:24.2965679  INFO auto_build_source_tree::clang::visitor: 54:     a/a.h
2024-10-18 17:57:24.3067543  INFO auto_build_source_tree::clang::visitor: 51: a/a.cpp
2024-10-18 17:57:24.3072764  INFO auto_build_source_tree::clang::visitor: 54:     a/a.hpp
2024-10-18 17:57:24.3181722  INFO auto_build_source_tree::clang::visitor: 51: b/b.c
2024-10-18 17:57:24.3184832  INFO auto_build_source_tree::clang::visitor: 54:     b/b.h
2024-10-18 17:57:24.3284653  INFO auto_build_source_tree::clang::visitor: 51: b/b.cpp
2024-10-18 17:57:24.3290224  INFO auto_build_source_tree::clang::visitor: 54:     b/b.hpp
2024-10-18 17:57:24.3397523  INFO auto_build_source_tree::clang::visitor: 51: c/c.c
2024-10-18 17:57:24.3403655  INFO auto_build_source_tree::clang::visitor: 54:     c/c.h
2024-10-18 17:57:24.3518037  INFO auto_build_source_tree::clang::visitor: 51: c/c.cpp
2024-10-18 17:57:24.3520834  INFO auto_build_source_tree::clang::visitor: 54:     c/c.hpp
2024-10-18 17:57:24.3626187  INFO auto_build_source_tree::clang::visitor: 51: d/d.c
2024-10-18 17:57:24.3629806  INFO auto_build_source_tree::clang::visitor: 54:     d/d.h
2024-10-18 17:57:24.3730385  INFO auto_build_source_tree::clang::visitor: 51: d/d.h
2024-10-18 17:57:24.3826916  INFO auto_build_source_tree::clang::visitor: 51: d/d.cc
2024-10-18 17:57:24.3831235  INFO auto_build_source_tree::clang::visitor: 54:     d/d.hpp
2024-10-18 17:57:24.3933807  INFO auto_build_source_tree::clang::visitor: 51: d/d.hpp
2024-10-18 17:57:24.4026701  INFO auto_build_source_tree::clang::visitor: 51: test.cpp
2024-10-18 17:57:24.4032042  INFO auto_build_source_tree::clang::visitor: 54:     d/mod.hpp
2024-10-18 17:57:24.4125912  INFO auto_build_source_tree::clang::visitor: 51: d/mod.hpp
2024-10-18 17:57:24.4130126  INFO auto_build_source_tree::clang::visitor: 54:     d/d.h
2024-10-18 17:57:24.4134394  INFO auto_build_source_tree::clang::visitor: 54:     d/d.hpp
2024-10-18 17:57:24.4138511  WARN auto_build_source_tree: 37: output mermaid flowchat of source dependences
2024-10-18 17:57:24.4141827  INFO auto_build_source_tree: 39:
flowchart LR;
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
2024-10-18 17:57:24.4148259  WARN auto_build_source_tree: 46: output CMakeLists.txt
2024-10-18 17:57:24.415855  WARN auto_build_source_tree: 65: generate a build system with cmake
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
-- Configuring done (3.2s)
-- Generating done (0.0s)
-- Build files have been written to: D:/__develop__/FutureOrientedGB/auto_build_source_tree/test_c_cpp/build
2024-10-18 17:57:27.6735281  WARN auto_build_source_tree: 73: build with cmake
MSBuild version 17.11.9+a69bbaaf5 for .NET Framework

  1>Checking Build System
  Building Custom Rule D:/__develop__/FutureOrientedGB/auto_build_source_tree/test_c_cpp/CMakeLists.txt
  main.cpp
  a.c
  a.cpp
  b.c
  b.cpp
  c.c
  c.cpp
  test_c_cpp.vcxproj -> D:\__develop__\FutureOrientedGB\auto_build_source_tree\test_c_cpp\build\Debug\test_c_cpp.exe
  Building Custom Rule D:/__develop__/FutureOrientedGB/auto_build_source_tree/test_c_cpp/CMakeLists.txt
```