# Auto Source Compiler
- auto scan source dependencies with clang ir
- auto output mermaid flow chart of source dependencies
- auto output CMakeLists.txt
- auto generate a build system with cmake
- auto build with cmake
- auto install with cmake
- auto clean targets
- auto add c/c++ versioned library
- auto remove library


# Usage
```
# C/C++ Package Manager like Rust Cargo
> asc.exe --help
Usage: asc.exe <COMMAND>

Commands:
  new
  init
  search
  add
  remove
  scan
  build
  install
  run
  clean
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version


# new bin, lib or workspace
> asc.exe new --help
Usage: asc.exe new [OPTIONS] [NAME] [MEMBER]...

Arguments:
  [NAME]
  [MEMBER]...

Options:
      --lib
      --workspace
  -h, --help       Print help
  -V, --version    Print version

# init exists bin, lib or workspace
> asc.exe init --help
Usage: asc.exe init [OPTIONS] [MEMBER]...

Arguments:
  [MEMBER]...

Options:
      --lib
      --workspace
  -h, --help       Print help
  -V, --version    Print version

# scan soruce tree, generate cmake configs
> asc.exe scan --help
Usage: asc.exe scan [OPTIONS] [NAME]

Arguments:
  [NAME]

Options:
      --shared-lib
      --static-lib
      --cmake-minimum-version <CMAKE_MINIMUM_VERSION>  [default: 3.20]
  -h, --help                                           Print help
  -V, --version                                        Print version

# build with cmake
> asc.exe build --help
Usage: asc.exe build [OPTIONS] [NAME]

Arguments:
  [NAME]

Options:
      --config <CONFIG>  [default: debug] [possible values: debug, release]
  -h, --help             Print help
  -V, --version          Print version

# install targets with cmake
> asc.exe install --help
Usage: asc.exe install [OPTIONS] [NAME]

Arguments:
  [NAME]

Options:
      --config <CONFIG>  [default: debug] [possible values: debug, release]
  -h, --help             Print help
  -V, --version          Print version

# clean build targets
> asc.exe clean --help
Usage: asc.exe clean

Options:
  -h, --help     Print help
  -V, --version  Print version
> asc.exe run --help
Usage: asc.exe run [OPTIONS]

Options:
      --config <CONFIG>  [default: Debug] [possible values: debug, release]
  -h, --help             Print help
  -V, --version          Print version

# search library
> asc.exe search --help
Usage: asc.exe search --name <NAME>

Options:
      --name <NAME>
  -h, --help         Print help
  -V, --version      Print version

# add versioned library asc.toml dependencies
> asc.exe add --help
Usage: asc.exe add [OPTIONS] --name <NAME>

Options:
      --name <NAME>
      --features <FEATURES>
  -h, --help                 Print help
  -V, --version              Print version

# remove library from asc.toml dependencies
> asc.exe remove --help
Usage: asc.exe remove --name <NAME>

Options:
      --name <NAME>
  -h, --help         Print help
  -V, --version      Print version
```

# workspace asc.toml
```toml
[workspace]
members = [
    "a",
    "b",
    "c",
]
```


# simple project asc.toml
```toml
[package]
name = "test_c_cpp"
version = "2024.10.22"
edition = "2024"
```

# full project asc.toml
```toml
[package]
name = "test"
version = "2024.10.21"
edition = "2024"


[features]
all = [
    "a",
    "b",
    "c",
]
default = [
    "a",
    "b",
]

[dependencies]
fmt = { version = "11.0.2", find = ["fmt"], link = ["fmt::fmt"] }
spdlog = { version = "1.14.1", find = ["spdlog"], link = ["spdlog::spdlog"] }
ffmpeg = { version = "7.0.2#3", find = ["FFMPEG"], link = ["${FFMPEG_LIBRARIES}"] }
qt5-base = { version = "5.15.14#2", find = ["Qt5", "COMPONENTS", "Core", "Gui", "Widgets"], link = ["Qt5::Core", "Qt5::Widgets", "Qt5::Gui"] }
```



# asc exists sources
```
> cd test_c_cpp
> asc.exe init --lib
2024-10-22 15:45:59.2468421  INFO asc::cli::commands::init: 37: init bin
2024-10-22 15:45:59.2469928  INFO asc::cli::commands::init: 42: init package
> asc.exe init
2024-10-22 15:46:13.8842823  INFO asc::cli::commands::init: 32: init bin
2024-10-22 15:46:13.884457  INFO asc::cli::commands::init: 42: init package
> asc.exe scan
2024-10-22 15:46:17.8208018  WARN asc::config::method: 132: func="util::fs::is_file_exists" path="asc.toml" error_tag="file_exists_error" skip
2024-10-22 15:46:17.8216048  INFO asc::cli::commands::scan: 74: scan package
2024-10-22 15:46:17.8218736  INFO asc::cli::commands::scan: 90: ScanOptions {
    project: "test_c_cpp",
    project_dir: "D:/__develop__/FutureOrientedGB/asc/test_c_cpp",
    target_dir: "D:/__develop__/FutureOrientedGB/asc/test_c_cpp/target",
    source_dir: "D:/__develop__/FutureOrientedGB/asc/test_c_cpp/src",
    entry_point_source: "D:/__develop__/FutureOrientedGB/asc/test_c_cpp/src/main.cpp",
    include_dirs: [],
    shared_lib: false,
    static_lib: false,
    cmake_minimum_version: "3.20",
    cmake_config: "",
}
2024-10-22 15:46:17.8223685  WARN asc::cli::commands::scan: 97: scan source dependencies with clang ir
2024-10-22 15:46:17.8380284  INFO asc::clang::visitor: 69: main.cpp
2024-10-22 15:46:17.8383455  INFO asc::clang::visitor: 75:     wrapping.hpp
2024-10-22 15:46:17.8476211  INFO asc::clang::visitor: 69: wrapping.hpp
2024-10-22 15:46:17.8479422  INFO asc::clang::visitor: 75:     a/mod.hpp
2024-10-22 15:46:17.8482555  INFO asc::clang::visitor: 75:     b/mod.hpp
2024-10-22 15:46:17.8485553  INFO asc::clang::visitor: 75:     c/mod.hpp
2024-10-22 15:46:17.8577806  INFO asc::clang::visitor: 69: a/mod.hpp
2024-10-22 15:46:17.8579687  INFO asc::clang::visitor: 75:     a/a.h
2024-10-22 15:46:17.8581642  INFO asc::clang::visitor: 75:     a/a.hpp
2024-10-22 15:46:17.8692644  INFO asc::clang::visitor: 69: a/a.h
2024-10-22 15:46:17.8695134  INFO asc::clang::visitor: 75:     export.h
2024-10-22 15:46:17.8809259  INFO asc::clang::visitor: 69: export.h
2024-10-22 15:46:17.8909568  INFO asc::clang::visitor: 69: a/a.hpp
2024-10-22 15:46:17.9007367  INFO asc::clang::visitor: 69: b/mod.hpp
2024-10-22 15:46:17.9009714  INFO asc::clang::visitor: 75:     b/b.h
2024-10-22 15:46:17.9011558  INFO asc::clang::visitor: 75:     b/b.hpp
2024-10-22 15:46:17.9104193  INFO asc::clang::visitor: 69: b/b.h
2024-10-22 15:46:17.9210741  INFO asc::clang::visitor: 69: b/b.hpp
2024-10-22 15:46:17.9295348  INFO asc::clang::visitor: 69: c/mod.hpp
2024-10-22 15:46:17.9298242  INFO asc::clang::visitor: 75:     c/c.h
2024-10-22 15:46:17.9301553  INFO asc::clang::visitor: 75:     c/c.hpp
2024-10-22 15:46:17.9404044  INFO asc::clang::visitor: 69: c/c.h
2024-10-22 15:46:17.9512715  INFO asc::clang::visitor: 69: c/c.hpp
2024-10-22 15:46:17.9603022  INFO asc::clang::visitor: 69: a/a.c
2024-10-22 15:46:17.9607352  INFO asc::clang::visitor: 75:     a/a.h
2024-10-22 15:46:17.9708013  INFO asc::clang::visitor: 69: a/a.cpp
2024-10-22 15:46:17.9710239  INFO asc::clang::visitor: 75:     a/a.hpp
2024-10-22 15:46:17.9825016  INFO asc::clang::visitor: 69: b/b.c
2024-10-22 15:46:17.9828086  INFO asc::clang::visitor: 75:     b/b.h
2024-10-22 15:46:17.9934932  INFO asc::clang::visitor: 69: b/b.cpp
2024-10-22 15:46:17.9940362  INFO asc::clang::visitor: 75:     b/b.hpp
2024-10-22 15:46:18.0057471  INFO asc::clang::visitor: 69: c/c.c
2024-10-22 15:46:18.0059435  INFO asc::clang::visitor: 75:     c/c.h
2024-10-22 15:46:18.015812  INFO asc::clang::visitor: 69: c/c.cpp
2024-10-22 15:46:18.0164069  INFO asc::clang::visitor: 75:     c/c.hpp
2024-10-22 15:46:18.0262992  INFO asc::clang::visitor: 69: d/d.c
2024-10-22 15:46:18.0265581  INFO asc::clang::visitor: 75:     d/d.h
2024-10-22 15:46:18.0385764  INFO asc::clang::visitor: 69: d/d.h
2024-10-22 15:46:18.0476206  INFO asc::clang::visitor: 69: d/d.cc
2024-10-22 15:46:18.048091  INFO asc::clang::visitor: 75:     d/d.hpp
2024-10-22 15:46:18.0586779  INFO asc::clang::visitor: 69: d/d.hpp
2024-10-22 15:46:18.0591915  INFO asc::clang::visitor: 75:     export.h
2024-10-22 15:46:18.0710639  INFO asc::clang::visitor: 69: test.cpp
2024-10-22 15:46:18.0715635  INFO asc::clang::visitor: 75:     d/mod.hpp
2024-10-22 15:46:18.0815909  INFO asc::clang::visitor: 69: d/mod.hpp
2024-10-22 15:46:18.0819787  INFO asc::clang::visitor: 75:     d/d.h
2024-10-22 15:46:18.0824376  INFO asc::clang::visitor: 75:     d/d.hpp
2024-10-22 15:46:18.08289  WARN asc::cli::commands::scan: 100: output flow chart flowchart.md
2024-10-22 15:46:18.0835353  INFO asc::cli::commands::scan: 102:
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
    a/a.h ---> export.h;
    d/d.hpp ---> export.h;
    main.cpp ---> wrapping.hpp;
2024-10-22 15:46:18.0840313  WARN asc::cli::commands::scan: 104: output CMakeLists.txt
2024-10-22 15:46:18.0942631  WARN asc::cli::commands::scan: 108: generate a build system with cmake
2024-10-22 15:46:18.0948647  INFO asc::cmake::project: 12: command="cmake" args="-S D:/__develop__/FutureOrientedGB/asc/test_c_cpp -B D:/__develop__/FutureOrientedGB/asc/test_c_cpp/target"
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
-- Looking for include file stdlib.h
-- Looking for include file stdlib.h - found
-- Looking for gettimeofday
-- Looking for gettimeofday - not found
-- Looking for O_BINARY
-- Looking for O_BINARY - found
-- Configuring done (5.1s)
-- Generating done (0.0s)
-- Build files have been written to: D:/__develop__/FutureOrientedGB/asc/test_c_cpp/target

> asc.exe build
2024-10-22 15:46:28.0284024  INFO asc::cli::commands::build: 15: build
2024-10-22 15:46:28.0289255  WARN asc::config::method: 132: func="util::fs::is_file_exists" path="asc.toml" error_tag="file_exists_error" skip
2024-10-22 15:46:28.0293884  INFO asc::cmake::build: 11: command="cmake" args="--build target --config Debug"
MSBuild version 17.11.9+a69bbaaf5 for .NET Framework

  1>Checking Build System
  Building Custom Rule D:/__develop__/FutureOrientedGB/asc/test_c_cpp/CMakeLists.txt
  main.cpp
  a.c
  a.cpp
  b.c
  b.cpp
  c.c
  c.cpp
LINK : warning LNK4075: ignoring '/INCREMENTAL' due to '/OPT:ICF' specification [D:\__develop__\FutureOrientedGB\asc\test_c_cpp\target\test_c_cpp.vcxproj]
  test_c_cpp.vcxproj -> D:\__develop__\FutureOrientedGB\asc\test_c_cpp\target\Debug\test_c_cpp.exe
  Building Custom Rule D:/__develop__/FutureOrientedGB/asc/test_c_cpp/CMakeLists.txt
  
> asc.exe install
2024-10-22 15:46:34.0671238  INFO asc::cli::commands::install: 15: install
2024-10-22 15:46:34.0674419  WARN asc::config::method: 132: func="util::fs::is_file_exists" path="asc.toml" error_tag="file_exists_error" skip
2024-10-22 15:46:34.0678842  INFO asc::cmake::install: 11: command="cmake" args="--install target --config Debug"
-- Installing: C:/Program Files (x86)/test_c_cpp/bin/test_c_cpp.exe
```

asc new workspace
```
> asc.exe new --lib abc --workspace a b c
2024-10-22 15:49:02.728014  INFO asc::cli::commands::new: 227: new workspace
2024-10-22 15:49:02.7285986  INFO asc::cli::commands::new: 57: new lib
2024-10-22 15:49:02.7289951  INFO asc::cli::commands::new: 182: new package
2024-10-22 15:49:02.7298364  INFO asc::cli::commands::init: 42: init package
2024-10-22 15:49:02.732464  INFO asc::cli::commands::new: 57: new lib
2024-10-22 15:49:02.7326835  INFO asc::cli::commands::new: 182: new package
2024-10-22 15:49:02.7331759  INFO asc::cli::commands::init: 42: init package
2024-10-22 15:49:02.7346465  INFO asc::cli::commands::new: 57: new lib
2024-10-22 15:49:02.7348504  INFO asc::cli::commands::new: 182: new package
2024-10-22 15:49:02.7351948  INFO asc::cli::commands::init: 42: init package

> cd abc
> .asc.exe scan --shared-lib
2024-10-22 15:49:22.1727416  WARN asc::config::method: 132: func="util::fs::is_file_exists" path="asc.toml" error_tag="file_exists_error" skip
2024-10-22 15:49:22.1736892  INFO asc::cli::commands::scan: 115: scan workspace
2024-10-22 15:49:22.1740476  INFO asc::cli::commands::scan: 74: scan package
2024-10-22 15:49:22.1743151  INFO asc::cli::commands::scan: 90: ScanOptions {
    project: "a",
    project_dir: "D:/__develop__/FutureOrientedGB/asc/abc/a",
    target_dir: "D:/__develop__/FutureOrientedGB/asc/abc/a/target",
    source_dir: "D:/__develop__/FutureOrientedGB/asc/abc/a/src",
    entry_point_source: "D:/__develop__/FutureOrientedGB/asc/abc/a/src/lib.cpp",
    include_dirs: [],
    shared_lib: true,
    static_lib: false,
    cmake_minimum_version: "3.20",
    cmake_config: "",
}
2024-10-22 15:49:22.1749078  WARN asc::cli::commands::scan: 97: scan source dependencies with clang ir
2024-10-22 15:49:22.1906495  INFO asc::clang::visitor: 69: lib.cpp
2024-10-22 15:49:22.1910229  INFO asc::clang::visitor: 75:     lib.hpp
2024-10-22 15:49:22.2008269  INFO asc::clang::visitor: 69: lib.hpp
2024-10-22 15:49:22.201235  INFO asc::clang::visitor: 75:     export.h
2024-10-22 15:49:22.2138418  INFO asc::clang::visitor: 69: export.h
2024-10-22 15:49:22.2141412  WARN asc::cli::commands::scan: 100: output flow chart flowchart.md
2024-10-22 15:49:22.2144491  INFO asc::cli::commands::scan: 102:
flowchart LR;
    lib.hpp ---> export.h;
    lib.cpp ---> lib.hpp;
2024-10-22 15:49:22.2145171  WARN asc::cli::commands::scan: 104: output CMakeLists.txt
2024-10-22 15:49:22.2250303  INFO asc::cli::commands::scan: 74: scan package
2024-10-22 15:49:22.2252  INFO asc::cli::commands::scan: 90: ScanOptions {
    project: "b",
    project_dir: "D:/__develop__/FutureOrientedGB/asc/abc/b",
    target_dir: "D:/__develop__/FutureOrientedGB/asc/abc/b/target",
    source_dir: "D:/__develop__/FutureOrientedGB/asc/abc/b/src",
    entry_point_source: "D:/__develop__/FutureOrientedGB/asc/abc/b/src/lib.cpp",
    include_dirs: [],
    shared_lib: true,
    static_lib: false,
    cmake_minimum_version: "3.20",
    cmake_config: "",
}
2024-10-22 15:49:22.2257887  WARN asc::cli::commands::scan: 97: scan source dependencies with clang ir
2024-10-22 15:49:22.2349889  INFO asc::clang::visitor: 69: lib.cpp
2024-10-22 15:49:22.23527  INFO asc::clang::visitor: 75:     lib.hpp
2024-10-22 15:49:22.2446716  INFO asc::clang::visitor: 69: lib.hpp
2024-10-22 15:49:22.2452423  INFO asc::clang::visitor: 75:     export.h
2024-10-22 15:49:22.2566026  INFO asc::clang::visitor: 69: export.h
2024-10-22 15:49:22.2568766  WARN asc::cli::commands::scan: 100: output flow chart flowchart.md
2024-10-22 15:49:22.2571462  INFO asc::cli::commands::scan: 102:
flowchart LR;
    lib.hpp ---> export.h;
    lib.cpp ---> lib.hpp;
2024-10-22 15:49:22.257274  WARN asc::cli::commands::scan: 104: output CMakeLists.txt
2024-10-22 15:49:22.2678211  INFO asc::cli::commands::scan: 74: scan package
2024-10-22 15:49:22.2680657  INFO asc::cli::commands::scan: 90: ScanOptions {
    project: "c",
    project_dir: "D:/__develop__/FutureOrientedGB/asc/abc/c",
    target_dir: "D:/__develop__/FutureOrientedGB/asc/abc/c/target",
    source_dir: "D:/__develop__/FutureOrientedGB/asc/abc/c/src",
    entry_point_source: "D:/__develop__/FutureOrientedGB/asc/abc/c/src/lib.cpp",
    include_dirs: [],
    shared_lib: true,
    static_lib: false,
    cmake_minimum_version: "3.20",
    cmake_config: "",
}
2024-10-22 15:49:22.2687295  WARN asc::cli::commands::scan: 97: scan source dependencies with clang ir
2024-10-22 15:49:22.2784264  INFO asc::clang::visitor: 69: lib.cpp
2024-10-22 15:49:22.2788197  INFO asc::clang::visitor: 75:     lib.hpp
2024-10-22 15:49:22.2893219  INFO asc::clang::visitor: 69: lib.hpp
2024-10-22 15:49:22.2896065  INFO asc::clang::visitor: 75:     export.h
2024-10-22 15:49:22.3003818  INFO asc::clang::visitor: 69: export.h
2024-10-22 15:49:22.3007625  WARN asc::cli::commands::scan: 100: output flow chart flowchart.md
2024-10-22 15:49:22.3015296  INFO asc::cli::commands::scan: 102:
flowchart LR;
    lib.hpp ---> export.h;
    lib.cpp ---> lib.hpp;
2024-10-22 15:49:22.3018198  WARN asc::cli::commands::scan: 104: output CMakeLists.txt
2024-10-22 15:49:22.3130039  WARN asc::cli::commands::scan: 158: generate a build system with cmake
2024-10-22 15:49:22.3134289  INFO asc::cmake::project: 12: command="cmake" args="-S D:/__develop__/FutureOrientedGB/asc/abc -B D:/__develop__/FutureOrientedGB/asc/abc/target -D BUILD_SHARED_LIBS=1"
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
-- Looking for include file stdlib.h
-- Looking for include file stdlib.h - found
-- Looking for gettimeofday
-- Looking for gettimeofday - not found
-- Looking for O_BINARY
-- Looking for O_BINARY - found
-- Configuring done (5.3s)
CMake Error: INSTALL(EXPORT) given unknown export "a-targets"
CMake Error: INSTALL(EXPORT) given unknown export "b-targets"
CMake Error: INSTALL(EXPORT) given unknown export "c-targets"
-- Generating done (0.0s)
CMake Generate step failed.  Build files cannot be regenerated correctly.

> .asc.exe build
2024-10-22 15:49:37.6684246  INFO asc::cli::commands::build: 15: build
2024-10-22 15:49:37.6686842  WARN asc::config::method: 132: func="util::fs::is_file_exists" path="asc.toml" error_tag="file_exists_error" skip
2024-10-22 15:49:37.6688586  INFO asc::cmake::build: 11: command="cmake" args="--build target --config Debug"
MSBuild version 17.11.9+a69bbaaf5 for .NET Framework

  1>Checking Build System
  Building Custom Rule D:/__develop__/FutureOrientedGB/asc/abc/a/CMakeLists.txt
  lib.cpp
LINK : warning LNK4075: ignoring '/INCREMENTAL' due to '/OPT:ICF' specification [D:\__develop__\FutureOrientedGB\asc\abc\target\a\a.vcxproj]
     Creating library D:/__develop__/FutureOrientedGB/asc/abc/target/a/Debug/a.lib and object D:/__develop__/FutureOrientedGB/asc/abc/target/a/Debug/a.exp
  a.vcxproj -> D:\__develop__\FutureOrientedGB\asc\abc\target\a\Debug\a.dll
  Building Custom Rule D:/__develop__/FutureOrientedGB/asc/abc/b/CMakeLists.txt
  lib.cpp
LINK : warning LNK4075: ignoring '/INCREMENTAL' due to '/OPT:ICF' specification [D:\__develop__\FutureOrientedGB\asc\abc\target\b\b.vcxproj]
     Creating library D:/__develop__/FutureOrientedGB/asc/abc/target/b/Debug/b.lib and object D:/__develop__/FutureOrientedGB/asc/abc/target/b/Debug/b.exp
  b.vcxproj -> D:\__develop__\FutureOrientedGB\asc\abc\target\b\Debug\b.dll
  Building Custom Rule D:/__develop__/FutureOrientedGB/asc/abc/c/CMakeLists.txt
  lib.cpp
LINK : warning LNK4075: ignoring '/INCREMENTAL' due to '/OPT:ICF' specification [D:\__develop__\FutureOrientedGB\asc\abc\target\c\c.vcxproj]
     Creating library D:/__develop__/FutureOrientedGB/asc/abc/target/c/Debug/c.lib and object D:/__develop__/FutureOrientedGB/asc/abc/target/c/Debug/c.exp
  c.vcxproj -> D:\__develop__\FutureOrientedGB\asc\abc\target\c\Debug\c.dll
  Building Custom Rule D:/__develop__/FutureOrientedGB/asc/abc/CMakeLists.txt

> .asc.exe install
2024-10-22 15:49:57.4386968  INFO asc::cli::commands::install: 15: install
2024-10-22 15:49:57.4390542  WARN asc::config::method: 132: func="util::fs::is_file_exists" path="asc.toml" error_tag="file_exists_error" skip
2024-10-22 15:49:57.4394497  INFO asc::cmake::install: 11: command="cmake" args="--install target --config Debug"
-- Installing: C:/Program Files (x86)/abc/lib/a.lib
-- Installing: C:/Program Files (x86)/abc/bin/a.dll
-- Installing: C:/Program Files (x86)/abc/include/a/export.h
-- Installing: C:/Program Files (x86)/abc/include/a/lib.hpp
-- Installing: C:/Program Files (x86)/abc/share/a/a-config.cmake
-- Installing: C:/Program Files (x86)/abc/lib/b.lib
-- Installing: C:/Program Files (x86)/abc/bin/b.dll
-- Installing: C:/Program Files (x86)/abc/include/b/export.h
-- Installing: C:/Program Files (x86)/abc/include/b/lib.hpp
-- Installing: C:/Program Files (x86)/abc/share/b/b-config.cmake
-- Installing: C:/Program Files (x86)/abc/lib/c.lib
-- Installing: C:/Program Files (x86)/abc/bin/c.dll
-- Installing: C:/Program Files (x86)/abc/include/c/export.h
-- Installing: C:/Program Files (x86)/abc/include/c/lib.hpp
-- Installing: C:/Program Files (x86)/abc/share/c/c-config.cmake
```