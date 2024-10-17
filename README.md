[test_c_cpp_bin.md](./test_c_cpp_bin.md)

```
Usage: source_dependency_tree.exe [OPTIONS] --project <PROJECT> --source-dir <SOURCE_DIR> --entry-point-source <ENTRY_POINT_SOURCE>

Options:
      --project <PROJECT>
      --source-dir <SOURCE_DIR>
      --entry-point-source <ENTRY_POINT_SOURCE>
      --include-dirs <INCLUDE_DIRS>              [default: ]
  -h, --help                                     Print help
  -V, --version                                  Print version
```

```
> .\target\debug\source_dependency_tree.exe --source-dir=test_c_cpp_bin/src --entry-point-source=main.cpp --project=test_c_cpp_bin
2024-10-17T06:49:13.922456Z ERROR source_dependency_tree: 17: parse command line options
2024-10-17T06:49:13.923402Z ERROR source_dependency_tree: 20: CommandLines {
    project: "test_c_cpp_bin",
    source_dir: "test_c_cpp_bin/src",
    entry_point_source: "main.cpp",
    include_dirs: [
        "",
    ],
}
2024-10-17T06:49:13.923811Z ERROR source_dependency_tree: 28: generate source dependences
2024-10-17T06:49:13.924382Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/main.cpp
2024-10-17T06:49:14.458634Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/wrapping.hpp
2024-10-17T06:49:14.459340Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/mod.hpp
2024-10-17T06:49:14.459597Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/a.h
2024-10-17T06:49:14.459954Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/a.hpp
2024-10-17T06:49:14.460453Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/mod.hpp
2024-10-17T06:49:14.460841Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/b.h
2024-10-17T06:49:14.461077Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/b.hpp
2024-10-17T06:49:14.461271Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/mod.hpp
2024-10-17T06:49:14.461498Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/c.h
2024-10-17T06:49:14.461714Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/c.hpp
2024-10-17T06:49:31.760438Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/wrapping.hpp
2024-10-17T06:49:31.898276Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/mod.hpp
2024-10-17T06:49:31.898469Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/a.h
2024-10-17T06:49:31.898780Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/a.hpp
2024-10-17T06:49:31.899033Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/mod.hpp
2024-10-17T06:49:31.899213Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/b.h
2024-10-17T06:49:31.899488Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/b.hpp
2024-10-17T06:49:31.899755Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/mod.hpp
2024-10-17T06:49:31.899865Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/c.h
2024-10-17T06:49:31.899987Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/c.hpp
2024-10-17T06:49:31.906381Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/a/mod.hpp
2024-10-17T06:49:32.041182Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/a.h
2024-10-17T06:49:32.041447Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/a.hpp
2024-10-17T06:49:32.044102Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/a/a.h
2024-10-17T06:49:32.173799Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/a/a.hpp
2024-10-17T06:49:32.310896Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/b/mod.hpp
2024-10-17T06:49:32.445954Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/b.h
2024-10-17T06:49:32.446605Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/b.hpp
2024-10-17T06:49:32.449537Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/b/b.h
2024-10-17T06:49:32.583941Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/b/b.hpp
2024-10-17T06:49:32.714083Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/c/mod.hpp
2024-10-17T06:49:32.842396Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/c.h
2024-10-17T06:49:32.842818Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/c.hpp
2024-10-17T06:49:32.846263Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/c/c.h
2024-10-17T06:49:32.978204Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/c/c.hpp
2024-10-17T06:49:33.110243Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/a/a.c
2024-10-17T06:49:33.248545Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/a.h
2024-10-17T06:49:33.249856Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/a/a.cpp
2024-10-17T06:49:33.379152Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/a.hpp
2024-10-17T06:49:33.381578Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/b/b.c
2024-10-17T06:49:33.518205Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/b.h
2024-10-17T06:49:33.519367Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/b/b.cpp
2024-10-17T06:49:33.655310Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/b.hpp
2024-10-17T06:49:33.657999Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/c/c.c
2024-10-17T06:49:33.790524Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/c.h
2024-10-17T06:49:33.792226Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/c/c.cpp
2024-10-17T06:49:33.924868Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/c.hpp
2024-10-17T06:49:33.927623Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/d/d.c
2024-10-17T06:49:34.060822Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/d/d.h
2024-10-17T06:49:34.061735Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/d/d.cc
2024-10-17T06:49:34.194137Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/d/d.hpp
2024-10-17T06:49:34.197203Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/main.cpp
2024-10-17T06:49:34.723480Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/wrapping.hpp
2024-10-17T06:49:34.723770Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/mod.hpp
2024-10-17T06:49:34.724097Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/a.h
2024-10-17T06:49:34.724546Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/a/a.hpp
2024-10-17T06:49:34.724814Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/mod.hpp
2024-10-17T06:49:34.725072Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/b.h
2024-10-17T06:49:34.725547Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/b/b.hpp
2024-10-17T06:49:34.725988Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/mod.hpp
2024-10-17T06:49:34.726360Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/c.h
2024-10-17T06:49:34.727006Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/c/c.hpp
2024-10-17T06:49:51.672511Z ERROR source_dependency_tree::clang::callback: 28: test_c_cpp_bin/src/test.cpp
2024-10-17T06:49:51.812580Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/d/mod.hpp
2024-10-17T06:49:51.812822Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/d/d.h
2024-10-17T06:49:51.813117Z ERROR source_dependency_tree::clang::callback: 34:     test_c_cpp_bin/src/d/d.hpp
2024-10-17T06:49:51.815982Z ERROR source_dependency_tree: 31: generate markdown mermaid flowchat
2024-10-17T06:49:51.816600Z ERROR source_dependency_tree: 34:

flowchart LR;
    a/a.c ---> a/a.h;
    main.cpp ---> a/a.h;
    a/a.cpp ---> a/a.hpp;
    main.cpp ---> a/a.hpp;
    main.cpp ---> a/mod.hpp;
    b/b.c ---> b/b.h;
    main.cpp ---> b/b.h;
    b/b.cpp ---> b/b.hpp;
    main.cpp ---> b/b.hpp;
    main.cpp ---> b/mod.hpp;
    c/c.c ---> c/c.h;
    main.cpp ---> c/c.h;
    c/c.cpp ---> c/c.hpp;
    main.cpp ---> c/c.hpp;
    main.cpp ---> c/mod.hpp;
    main.cpp ---> wrapping.hpp;


2024-10-17T06:49:51.817276Z ERROR source_dependency_tree: 41: generate CMakeLists.txt
2024-10-17T06:49:51.817928Z ERROR source_dependency_tree: 46: cmake generate project
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
-- Configuring done (3.5s)
-- Generating done (0.0s)
-- Build files have been written to: D:/__develop__/FutureOrientedGB/source_dependency_tree/test_c_cpp_bin/build
2024-10-17T06:49:55.333240Z ERROR source_dependency_tree: 51: cmake build
MSBuild version 17.11.9+a69bbaaf5 for .NET Framework

  1>Checking Build System
  Building Custom Rule D:/__develop__/FutureOrientedGB/source_dependency_tree/test_c_cpp_bin/CMakeLists.txt
  a.c
  a.cpp
  b.c
  b.cpp
  c.c
  c.cpp
  main.cpp
  test_c_cpp_bin.vcxproj -> D:\__develop__\FutureOrientedGB\source_dependency_tree\test_c_cpp_bin\build\Debug\test_c_cpp_bin.exe
  Building Custom Rule D:/__develop__/FutureOrientedGB/source_dependency_tree/test_c_cpp_bin/CMakeLists.txt
```