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
> .\target\debug\asb.exe --source-dir=test_c_cpp/src --entry-point-source=main.cpp --action-type=all --cmake-target-type=library --cmake-lib-type=shared
2024-10-19 16:50:03.487011  WARN asb: 25: parse command lines
2024-10-19 16:50:03.4877332  INFO asb: 28: CommandLines {
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
2024-10-19 16:50:03.4878053  WARN asb: 34: scan source dependencies with clang ir
2024-10-19 16:50:03.4979419  INFO asb::clang::visitor: 51: main.cpp
2024-10-19 16:50:03.4979916  INFO asb::clang::visitor: 54:     wrapping.hpp
2024-10-19 16:50:03.5041168  INFO asb::clang::visitor: 51: wrapping.hpp
2024-10-19 16:50:03.5041617  INFO asb::clang::visitor: 54:     a/mod.hpp
2024-10-19 16:50:03.5041957  INFO asb::clang::visitor: 54:     b/mod.hpp
2024-10-19 16:50:03.5042293  INFO asb::clang::visitor: 54:     c/mod.hpp
2024-10-19 16:50:03.5101703  INFO asb::clang::visitor: 51: a/mod.hpp
2024-10-19 16:50:03.510212  INFO asb::clang::visitor: 54:     a/a.h
2024-10-19 16:50:03.5102447  INFO asb::clang::visitor: 54:     a/a.hpp
2024-10-19 16:50:03.5160382  INFO asb::clang::visitor: 51: a/a.h
2024-10-19 16:50:03.5218975  INFO asb::clang::visitor: 51: a/a.hpp
2024-10-19 16:50:03.5277055  INFO asb::clang::visitor: 51: b/mod.hpp
2024-10-19 16:50:03.5277725  INFO asb::clang::visitor: 54:     b/b.h
2024-10-19 16:50:03.5278198  INFO asb::clang::visitor: 54:     b/b.hpp
2024-10-19 16:50:03.5334636  INFO asb::clang::visitor: 51: b/b.h
2024-10-19 16:50:03.539597  INFO asb::clang::visitor: 51: b/b.hpp
2024-10-19 16:50:03.5456173  INFO asb::clang::visitor: 51: c/mod.hpp
2024-10-19 16:50:03.5456873  INFO asb::clang::visitor: 54:     c/c.h
2024-10-19 16:50:03.5457275  INFO asb::clang::visitor: 54:     c/c.hpp
2024-10-19 16:50:03.5513872  INFO asb::clang::visitor: 51: c/c.h
2024-10-19 16:50:03.5574755  INFO asb::clang::visitor: 51: c/c.hpp
2024-10-19 16:50:03.5634882  INFO asb::clang::visitor: 51: a/a.c
2024-10-19 16:50:03.5635466  INFO asb::clang::visitor: 54:     a/a.h
2024-10-19 16:50:03.5694862  INFO asb::clang::visitor: 51: a/a.cpp
2024-10-19 16:50:03.5695488  INFO asb::clang::visitor: 54:     a/a.hpp
2024-10-19 16:50:03.5756702  INFO asb::clang::visitor: 51: b/b.c
2024-10-19 16:50:03.5757331  INFO asb::clang::visitor: 54:     b/b.h
2024-10-19 16:50:03.5815405  INFO asb::clang::visitor: 51: b/b.cpp
2024-10-19 16:50:03.5816006  INFO asb::clang::visitor: 54:     b/b.hpp
2024-10-19 16:50:03.5874834  INFO asb::clang::visitor: 51: c/c.c
2024-10-19 16:50:03.5875601  INFO asb::clang::visitor: 54:     c/c.h
2024-10-19 16:50:03.5938677  INFO asb::clang::visitor: 51: c/c.cpp
2024-10-19 16:50:03.5939349  INFO asb::clang::visitor: 54:     c/c.hpp
2024-10-19 16:50:03.5997381  INFO asb::clang::visitor: 51: d/d.c
2024-10-19 16:50:03.5998071  INFO asb::clang::visitor: 54:     d/d.h
2024-10-19 16:50:03.6060957  INFO asb::clang::visitor: 51: d/d.h
2024-10-19 16:50:03.6122322  INFO asb::clang::visitor: 51: d/d.cc
2024-10-19 16:50:03.6122917  INFO asb::clang::visitor: 54:     d/d.hpp
2024-10-19 16:50:03.6191008  INFO asb::clang::visitor: 51: d/d.hpp
2024-10-19 16:50:03.6255936  INFO asb::clang::visitor: 51: test.cpp
2024-10-19 16:50:03.6256711  INFO asb::clang::visitor: 54:     d/mod.hpp
2024-10-19 16:50:03.6316226  INFO asb::clang::visitor: 51: d/mod.hpp
2024-10-19 16:50:03.631695  INFO asb::clang::visitor: 54:     d/d.h
2024-10-19 16:50:03.6317519  INFO asb::clang::visitor: 54:     d/d.hpp
2024-10-19 16:50:03.6318627  WARN asb: 37: output flow chart test_c_cpp.md
2024-10-19 16:50:03.6320113  INFO asb: 39:
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
2024-10-19 16:50:03.6321785  WARN asb: 41: output D:/sources/FutureOrientedGB/asb/test_c_cpp/CMakeLists.txt
2024-10-19 16:50:03.6389684  WARN asb: 49: generate a build system with cmake
2024-10-19 16:50:03.6390528  INFO asb::cmake::project: 14: command="cmake" args="-S D:/sources/FutureOrientedGB/asb/test_c_cpp -B D:/sources/FutureOrientedGB/asb/test_c_cpp/build -D BUILD_SHARED_LIBS=1"
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
-- Configuring done (2.6s)
CMake Error: INSTALL(EXPORT) given unknown export "test_c_cpp-targets"
-- Generating done (0.0s)
CMake Generate step failed.  Build files cannot be regenerated correctly.
2024-10-19 16:50:06.2844181  WARN asb: 57: build with cmake
2024-10-19 16:50:06.284507  INFO asb::cmake::build: 15: command="cmake" args="--build D:/sources/FutureOrientedGB/asb/test_c_cpp/build --config Debug"
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
2024-10-19 16:50:07.4618952  WARN asb: 65: install with cmake
2024-10-19 16:50:07.4619892  INFO asb::cmake::install: 15: command="cmake" args="--install D:/sources/FutureOrientedGB/asb/test_c_cpp/build --config Debug"
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/lib/test_c_cpp.lib
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/bin/test_c_cpp.dll
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/include/test_c_cpp/a/a.h
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/include/test_c_cpp/a/a.hpp
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/include/test_c_cpp/a/mod.hpp
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/include/test_c_cpp/b/b.h
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/include/test_c_cpp/b/b.hpp
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/include/test_c_cpp/b/mod.hpp
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/include/test_c_cpp/c/c.h
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/include/test_c_cpp/c/c.hpp
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/include/test_c_cpp/c/mod.hpp
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/include/test_c_cpp/wrapping.hpp
-- Installing: D:/sources/FutureOrientedGB/asb/test_c_cpp/build/target/share/test_c_cpp/test_c_cpp-config.cmake
```
