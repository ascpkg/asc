# Auto Souce Builder
- scan source dependencies with clang ir
- output mermaid flow chat of source dependencies

  for example: [test_c_cpp](./test_c_cpp.md)

- output CMakeLists.txt
- generate a build system with cmake
- build with cmake
- install with cmake

# Usage
```
Usage: asb.exe [OPTIONS] --source-dir <SOURCE_DIR> --entry-point-source <ENTRY_POINT_SOURCE>

Options:
      --action-type <ACTION_TYPE>
          [default: scan] [possible values: all, scan, configure, build, install]
      --project <PROJECT>
          default to parent dir name of source_dir [default: ]
      --project-dir <PROJECT_DIR>
          default to parent dir of source_dir [default: ]
      --build-dir <BUILD_DIR>
          default to project_dir/build [default: ]
      --source-dir <SOURCE_DIR>
          source_dir must have a sub dir named src to store source files
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
> .\target\debug\asb.exe --source-dir=test_c_cpp/src --entry-point-source=main.cpp --action-type=all --cmake-target-type=library --cmake-lib-type=shared
2024-10-19 22:48:12.5306943  WARN asb: 25: parse command lines
2024-10-19 22:48:12.5315333  INFO asb: 28: CommandLines {
    action_type: All,
    project: "test_c_cpp",
    project_dir: "D:/sources/FutureOrientedGB/asb/test_c_cpp",
    build_dir: "D:/sources/FutureOrientedGB/asb/test_c_cpp/build",
    source_dir: "D:/sources/FutureOrientedGB/asb/test_c_cpp/src",
    entry_point_source: "main.cpp",
    include_dirs: [],
    cmake_minimum_version: "3.20",
    cmake_target_type: Library,
    cmake_lib_type: Shared,
    cmake_config: Debug,
}
2024-10-19 22:48:12.5318202  WARN asb: 39: scan source dependencies with clang ir
2024-10-19 22:48:12.542089  INFO asb::clang::visitor: 65: main.cpp
2024-10-19 22:48:12.5421721  INFO asb::clang::visitor: 71:     config.h
2024-10-19 22:48:12.5422074  INFO asb::clang::visitor: 71:     version.h
2024-10-19 22:48:12.5422409  INFO asb::clang::visitor: 71:     wrapping.hpp
2024-10-19 22:48:12.5482344  INFO asb::clang::visitor: 65: config.h
2024-10-19 22:48:12.5538848  INFO asb::clang::visitor: 65: version.h
2024-10-19 22:48:12.5600149  INFO asb::clang::visitor: 65: wrapping.hpp
2024-10-19 22:48:12.5600741  INFO asb::clang::visitor: 71:     a/mod.hpp
2024-10-19 22:48:12.560107  INFO asb::clang::visitor: 71:     b/mod.hpp
2024-10-19 22:48:12.5601666  INFO asb::clang::visitor: 71:     c/mod.hpp
2024-10-19 22:48:12.5661424  INFO asb::clang::visitor: 65: a/mod.hpp
2024-10-19 22:48:12.5662021  INFO asb::clang::visitor: 71:     a/a.h
2024-10-19 22:48:12.5662523  INFO asb::clang::visitor: 71:     a/a.hpp
2024-10-19 22:48:12.5722322  INFO asb::clang::visitor: 65: a/a.h
2024-10-19 22:48:12.5723001  INFO asb::clang::visitor: 71:     export.h
2024-10-19 22:48:12.5783986  INFO asb::clang::visitor: 65: export.h
2024-10-19 22:48:12.5843815  INFO asb::clang::visitor: 65: a/a.hpp
2024-10-19 22:48:12.5908645  INFO asb::clang::visitor: 65: b/mod.hpp
2024-10-19 22:48:12.5909754  INFO asb::clang::visitor: 71:     b/b.h
2024-10-19 22:48:12.591025  INFO asb::clang::visitor: 71:     b/b.hpp
2024-10-19 22:48:12.5972114  INFO asb::clang::visitor: 65: b/b.h
2024-10-19 22:48:12.603439  INFO asb::clang::visitor: 65: b/b.hpp
2024-10-19 22:48:12.6099299  INFO asb::clang::visitor: 65: c/mod.hpp
2024-10-19 22:48:12.610022  INFO asb::clang::visitor: 71:     c/c.h
2024-10-19 22:48:12.6100743  INFO asb::clang::visitor: 71:     c/c.hpp
2024-10-19 22:48:12.616439  INFO asb::clang::visitor: 65: c/c.h
2024-10-19 22:48:12.6165011  INFO asb::clang::visitor: 71:     export.h
2024-10-19 22:48:12.6227562  INFO asb::clang::visitor: 65: c/c.hpp
2024-10-19 22:48:12.6291795  INFO asb::clang::visitor: 65: a/a.c
2024-10-19 22:48:12.6292588  INFO asb::clang::visitor: 71:     a/a.h
2024-10-19 22:48:12.6381847  INFO asb::clang::visitor: 65: a/a.cpp
2024-10-19 22:48:12.6383164  INFO asb::clang::visitor: 71:     a/a.hpp
2024-10-19 22:48:12.6532949  INFO asb::clang::visitor: 65: b/b.c
2024-10-19 22:48:12.6534047  INFO asb::clang::visitor: 71:     b/b.h
2024-10-19 22:48:12.6620727  INFO asb::clang::visitor: 65: b/b.cpp
2024-10-19 22:48:12.6621612  INFO asb::clang::visitor: 71:     b/b.hpp
2024-10-19 22:48:12.6682808  INFO asb::clang::visitor: 65: c/c.c
2024-10-19 22:48:12.668341  INFO asb::clang::visitor: 71:     c/c.h
2024-10-19 22:48:12.6743971  INFO asb::clang::visitor: 65: c/c.cpp
2024-10-19 22:48:12.6744724  INFO asb::clang::visitor: 71:     c/c.hpp
2024-10-19 22:48:12.6806194  INFO asb::clang::visitor: 65: d/d.c
2024-10-19 22:48:12.6806902  INFO asb::clang::visitor: 71:     d/d.h
2024-10-19 22:48:12.6864934  INFO asb::clang::visitor: 65: d/d.h
2024-10-19 22:48:12.6865448  INFO asb::clang::visitor: 71:     export.h
2024-10-19 22:48:12.6927401  INFO asb::clang::visitor: 65: d/d.cc
2024-10-19 22:48:12.6928257  INFO asb::clang::visitor: 71:     d/d.hpp
2024-10-19 22:48:12.6992606  INFO asb::clang::visitor: 65: d/d.hpp
2024-10-19 22:48:12.7051503  INFO asb::clang::visitor: 65: test.cpp
2024-10-19 22:48:12.7052149  INFO asb::clang::visitor: 71:     d/mod.hpp
2024-10-19 22:48:12.7117829  INFO asb::clang::visitor: 65: d/mod.hpp
2024-10-19 22:48:12.7118628  INFO asb::clang::visitor: 71:     d/d.h
2024-10-19 22:48:12.71193  INFO asb::clang::visitor: 71:     d/d.hpp
2024-10-19 22:48:12.7120313  WARN asb: 42: output flow chart test_c_cpp.md
2024-10-19 22:48:12.7121884  INFO asb: 44:
flowchart LR;
    main.cpp ---> config.h;
    main.cpp ---> version.h;
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
    a/a.h ---> export.h;
    c/c.h ---> export.h;
    d/d.h ---> export.h;
    main.cpp ---> wrapping.hpp;
2024-10-19 22:48:12.712347  WARN asb: 46: output D:/sources/FutureOrientedGB/asb/test_c_cpp/CMakeLists.txt
2024-10-19 22:48:12.7195249  WARN asb: 54: generate a build system with cmake
2024-10-19 22:48:12.7196073  INFO asb::cmake::project: 14: command="cmake" args="-S D:/sources/FutureOrientedGB/asb/test_c_cpp -B D:/sources/FutureOrientedGB/asb/test_c_cpp/build -D BUILD_SHARED_LIBS=1"
-- Building for: Visual Studio 17 2022
-- Selecting Windows SDK version 10.0.22621.0 to target Windows 10.0.22631.
-- The C compiler identification is MSVC 19.41.34123.0
-- The CXX compiler identification is MSVC 19.41.34123.0
-- Detecting C compiler ABI info
-- Detecting C compiler ABI info - done
-- Check for working C compiler: C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.41.34120/bin/Hostx64/x64/cl.exe - skipped
-- Detecting C compile features
-- Detecting C compile features - done
-- Detecting CXX compiler ABI info
-- Detecting CXX compiler ABI info - done
-- Check for working CXX compiler: C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.41.34120/bin/Hostx64/x64/cl.exe - skipped
-- Detecting CXX compile features
-- Detecting CXX compile features - done
-- Looking for include file stdlib.h
-- Looking for include file stdlib.h - found
-- Looking for gettimeofday
-- Looking for gettimeofday - not found
-- Looking for O_BINARY
-- Looking for O_BINARY - found
-- Configuring done (3.7s)
CMake Error: INSTALL(EXPORT) given unknown export "test_c_cpp-targets"
-- Generating done (0.0s)
CMake Generate step failed.  Build files cannot be regenerated correctly.
2024-10-19 22:48:16.4244406  WARN asb: 62: build with cmake
2024-10-19 22:48:16.4245197  INFO asb::cmake::build: 15: command="cmake" args="--build D:/sources/FutureOrientedGB/asb/test_c_cpp/build --config Debug"
MSBuild version 17.11.9+a69bbaaf5 for .NET Framework

  1>Checking Build System
  Building Custom Rule D:/sources/FutureOrientedGB/asb/test_c_cpp/CMakeLists.txt
  main.cpp
  a.c
  a.cpp
  b.c
  b.cpp
  c.c
  c.cpp
LINK : warning LNK4075: ignoring '/INCREMENTAL' due to '/OPT:ICF' specification [D:\sources\FutureOrientedGB\asb\test_c_cpp\build\test_c_cpp.vcxproj]
     Creating library D:/sources/FutureOrientedGB/asb/test_c_cpp/build/Debug/test_c_cpp.lib and object D:/sources/FutureOrientedGB/asb/test_c_cpp/build/Debug/test_c_cpp.exp
  test_c_cpp.vcxproj -> D:\sources\FutureOrientedGB\asb\test_c_cpp\build\Debug\test_c_cpp.dll
  Building Custom Rule D:/sources/FutureOrientedGB/asb/test_c_cpp/CMakeLists.txt
2024-10-19 22:48:17.6467584  WARN asb: 70: install with cmake
2024-10-19 22:48:17.6468591  INFO asb::cmake::install: 15: command="cmake" args="--install D:/sources/FutureOrientedGB/asb/test_c_cpp/build --config Debug"
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/lib/test_c_cpp.lib
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/bin/test_c_cpp.dll
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/config.h
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/version.h
-- Up-to-date: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/a/a.h
-- Up-to-date: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/a/a.hpp
-- Up-to-date: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/a/mod.hpp
-- Up-to-date: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/b/b.h
-- Up-to-date: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/b/b.hpp
-- Up-to-date: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/b/mod.hpp
-- Up-to-date: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/c/c.h
-- Up-to-date: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/c/c.hpp
-- Up-to-date: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/c/mod.hpp
-- Up-to-date: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/export.h
-- Up-to-date: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/include/test_c_cpp/wrapping.hpp
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/test_c_cpp/share/test_c_cpp/test_c_cpp-config.cmake
```
