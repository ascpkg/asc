# Auto Source Builder
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
> .\target\debug\asb.exe --source-dir=test_c_cpp/src --entry-point-source=main.cpp --action-type=all --cmake-target-type=library --cmake-lib-type=shared --cmake-config=release
2024-10-19 23:46:38.0400452  WARN asb: 25: parse command lines
2024-10-19 23:46:38.0406013  INFO asb: 28: Options {
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
    cmake_config: Release,
}
2024-10-19 23:46:38.0408491  WARN asb: 39: scan source dependencies with clang ir
2024-10-19 23:46:38.0509476  INFO asb::clang::visitor: 65: main.cpp
2024-10-19 23:46:38.0510096  INFO asb::clang::visitor: 71:     config.h
2024-10-19 23:46:38.0510424  INFO asb::clang::visitor: 71:     version.h
2024-10-19 23:46:38.0510743  INFO asb::clang::visitor: 71:     wrapping.hpp
2024-10-19 23:46:38.057311  INFO asb::clang::visitor: 65: config.h
2024-10-19 23:46:38.0629884  INFO asb::clang::visitor: 65: version.h
2024-10-19 23:46:38.0687801  INFO asb::clang::visitor: 65: wrapping.hpp
2024-10-19 23:46:38.0688328  INFO asb::clang::visitor: 71:     a/mod.hpp
2024-10-19 23:46:38.0688661  INFO asb::clang::visitor: 71:     b/mod.hpp
2024-10-19 23:46:38.0689035  INFO asb::clang::visitor: 71:     c/mod.hpp
2024-10-19 23:46:38.0749937  INFO asb::clang::visitor: 65: a/mod.hpp
2024-10-19 23:46:38.0750437  INFO asb::clang::visitor: 71:     a/a.h
2024-10-19 23:46:38.0750822  INFO asb::clang::visitor: 71:     a/a.hpp
2024-10-19 23:46:38.080913  INFO asb::clang::visitor: 65: a/a.h
2024-10-19 23:46:38.0809763  INFO asb::clang::visitor: 71:     export.h
2024-10-19 23:46:38.0867937  INFO asb::clang::visitor: 65: export.h
2024-10-19 23:46:38.0927918  INFO asb::clang::visitor: 65: a/a.hpp
2024-10-19 23:46:38.092844  INFO asb::clang::visitor: 71:     export.h
2024-10-19 23:46:38.0984851  INFO asb::clang::visitor: 65: b/mod.hpp
2024-10-19 23:46:38.0985426  INFO asb::clang::visitor: 71:     b/b.h
2024-10-19 23:46:38.0985807  INFO asb::clang::visitor: 71:     b/b.hpp
2024-10-19 23:46:38.1046123  INFO asb::clang::visitor: 65: b/b.h
2024-10-19 23:46:38.1046718  INFO asb::clang::visitor: 71:     export.h
2024-10-19 23:46:38.1107037  INFO asb::clang::visitor: 65: b/b.hpp
2024-10-19 23:46:38.1164295  INFO asb::clang::visitor: 65: c/mod.hpp
2024-10-19 23:46:38.1164849  INFO asb::clang::visitor: 71:     c/c.h
2024-10-19 23:46:38.1165231  INFO asb::clang::visitor: 71:     c/c.hpp
2024-10-19 23:46:38.1231419  INFO asb::clang::visitor: 65: c/c.h
2024-10-19 23:46:38.1232195  INFO asb::clang::visitor: 71:     export.h
2024-10-19 23:46:38.1292544  INFO asb::clang::visitor: 65: c/c.hpp
2024-10-19 23:46:38.1360955  INFO asb::clang::visitor: 65: a/a.c
2024-10-19 23:46:38.1361877  INFO asb::clang::visitor: 71:     a/a.h
2024-10-19 23:46:38.145592  INFO asb::clang::visitor: 65: a/a.cpp
2024-10-19 23:46:38.1457023  INFO asb::clang::visitor: 71:     a/a.hpp
2024-10-19 23:46:38.1528431  INFO asb::clang::visitor: 65: b/b.c
2024-10-19 23:46:38.1529164  INFO asb::clang::visitor: 71:     b/b.h
2024-10-19 23:46:38.1591103  INFO asb::clang::visitor: 65: b/b.cpp
2024-10-19 23:46:38.1591634  INFO asb::clang::visitor: 71:     b/b.hpp
2024-10-19 23:46:38.1651264  INFO asb::clang::visitor: 65: c/c.c
2024-10-19 23:46:38.1651816  INFO asb::clang::visitor: 71:     c/c.h
2024-10-19 23:46:38.1712286  INFO asb::clang::visitor: 65: c/c.cpp
2024-10-19 23:46:38.1713034  INFO asb::clang::visitor: 71:     c/c.hpp
2024-10-19 23:46:38.1777866  INFO asb::clang::visitor: 65: d/d.c
2024-10-19 23:46:38.1778505  INFO asb::clang::visitor: 71:     d/d.h
2024-10-19 23:46:38.1839745  INFO asb::clang::visitor: 65: d/d.h
2024-10-19 23:46:38.1905416  INFO asb::clang::visitor: 65: d/d.cc
2024-10-19 23:46:38.190618  INFO asb::clang::visitor: 71:     d/d.hpp
2024-10-19 23:46:38.1969483  INFO asb::clang::visitor: 65: d/d.hpp
2024-10-19 23:46:38.2031687  INFO asb::clang::visitor: 65: test.cpp
2024-10-19 23:46:38.2032319  INFO asb::clang::visitor: 71:     d/mod.hpp
2024-10-19 23:46:38.2094048  INFO asb::clang::visitor: 65: d/mod.hpp
2024-10-19 23:46:38.2094669  INFO asb::clang::visitor: 71:     d/d.h
2024-10-19 23:46:38.2095119  INFO asb::clang::visitor: 71:     d/d.hpp
2024-10-19 23:46:38.2096001  WARN asb: 42: output flow chart test_c_cpp.md
2024-10-19 23:46:38.2097426  INFO asb: 44:
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
    a/a.hpp ---> export.h;
    b/b.h ---> export.h;
    c/c.h ---> export.h;
    main.cpp ---> wrapping.hpp;
2024-10-19 23:46:38.2098558  WARN asb: 46: output D:/sources/FutureOrientedGB/asb/test_c_cpp/CMakeLists.txt
2024-10-19 23:46:38.2168755  WARN asb: 54: generate a build system with cmake
2024-10-19 23:46:38.2169406  INFO asb::cmake::project: 14: command="cmake" args="-S D:/sources/FutureOrientedGB/asb/test_c_cpp -B D:/sources/FutureOrientedGB/asb/test_c_cpp/build -D BUILD_SHARED_LIBS=1"
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
-- Configuring done (3.2s)
CMake Error: INSTALL(EXPORT) given unknown export "test_c_cpp-targets"
-- Generating done (0.0s)
CMake Generate step failed.  Build files cannot be regenerated correctly.
2024-10-19 23:46:41.4549851  WARN asb: 62: build with cmake
2024-10-19 23:46:41.4551238  INFO asb::cmake::build: 11: command="cmake" args="--build D:/sources/FutureOrientedGB/asb/test_c_cpp/build --config Release"
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
     Creating library D:/sources/FutureOrientedGB/asb/test_c_cpp/build/Release/test_c_cpp.lib and object D:/sources/Fut
  ureOrientedGB/asb/test_c_cpp/build/Release/test_c_cpp.exp
  test_c_cpp.vcxproj -> D:\sources\FutureOrientedGB\asb\test_c_cpp\build\Release\test_c_cpp.dll
  Building Custom Rule D:/sources/FutureOrientedGB/asb/test_c_cpp/CMakeLists.txt
2024-10-19 23:46:42.6339611  WARN asb: 70: install with cmake
2024-10-19 23:46:42.6340349  INFO asb::cmake::install: 11: command="cmake" args="--install D:/sources/FutureOrientedGB/asb/test_c_cpp/build --config Release"
-- Installing: C:/Program Files (x86)/test_c_cpp/lib/test_c_cpp.lib
-- Installing: C:/Program Files (x86)/test_c_cpp/bin/test_c_cpp.dll
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/config.h
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/version.h
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/a/a.h
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/a/a.hpp
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/a/mod.hpp
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/b/b.h
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/b/b.hpp
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/b/mod.hpp
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/c/c.h
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/c/c.hpp
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/c/mod.hpp
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/export.h
-- Installing: C:/Program Files (x86)/test_c_cpp/include/test_c_cpp/wrapping.hpp
-- Installing: C:/Program Files (x86)/test_c_cpp/share/test_c_cpp/test_c_cpp-config.cmake
```
