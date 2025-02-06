# C/C++ package manager and auto source builder (inspired by Rust Cargo)

```
 █████  ███████  ██████     ██████  ██   ██  ██████  
██   ██ ██      ██          ██   ██ ██  ██  ██       
███████ ███████ ██          ██████  █████   ██   ███ 
██   ██      ██ ██          ██      ██  ██  ██    ██ 
██   ██ ███████  ██████     ██      ██   ██  ██████  
```

[简体中文](README.zh-CN.md)

[![Crates.io](https://img.shields.io/crates/d/asc_bin.svg)](https://crates.io/crates/asc_bin)
[![pypi.org](https://img.shields.io/pypi/dm/asc-bin)](https://pypi.org/project/asc-bin)
[![github.com](https://img.shields.io/github/downloads/ascpkg/asc/total.svg)](https://github.com/ascpkg/asc/releases)
[![Crates.io](https://img.shields.io/crates/v/asc_bin.svg)](https://crates.io/crates/asc_bin)
[![release_windows_targets](https://github.com/ascpkg/asc/actions/workflows/release_windows_targets.yml/badge.svg)](https://github.com/ascpkg/asc/actions/workflows/release_windows_targets.yml)
[![release_linux_darwin_targets](https://github.com/ascpkg/asc/actions/workflows/release_linux_darwin_targets.yml/badge.svg)](https://github.com/ascpkg/asc/actions/workflows/release_linux_darwin_targets.yml)

# 1. Features
- new asc package/workspace or init exits sources to asc package/workspace
- search/add/remove/install dependencies with vcpkg
- auto scan source tree with libclang and output mermaid flowchart of source dependencies
- auto generate cmake and vcpkg configurations for building
- build/run/install/uninstall/clean targets



# 2. Usage
> asc --help
```
C/C++'s package manager

Usage: asc.exe <COMMAND>

Commands:
  new        new package/workspace of binary/static library/shared library
  init       init directory as package/workspace of binary/static library/shared library
  vcpkg      update vcpkg source, build vcpkg versions index, set/get vcpkg configurations
  search     search package with exactly matching name or startswith/endswith/contains text
  add        add dependency to package or workspace member's asc.toml
  remove     remove dependency from package or workspace member's asc.toml
  scan       scan necessary sources, generate cmake and vcpkg configurations
  build      build all, package or workspace member
  run        run package or workspace member binary
  clean      clean .asc and target directory
  install    install executable/headers/libraries
  uninstall  uninstall installed executable/headers/libraries
  publish    publish package to vcpkg registry
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```


# 3. Command manual

```mermaid
flowchart TD
    asc_vcpkg_get[asc vcpkg get]-->asc_vcpkg_update[asc vcpkg update]-->asc_vcpkg_index[asc vcpkg index]-->
    asc_new_init[asc new, asc init]-->asc_search_add_remove[asc search, asc add, asc remove]-->asc_scan[asc scan]-->asc_build[asc build]-->asc_run[asc run]-->
    asc_install_uninstall[asc install, asc uninstall, asc clean, asc publish]
```

## 3.1. vcpkg
### 3.1.1. help
> asc vcpkg --help
```
update vcpkg source, build vcpkg versions index, set/get vcpkg configurations

Usage: asc.exe vcpkg [OPTIONS] <ACTION> [ARGS]...

Arguments:
  <ACTION>   update/index/set/get [possible values: update, set, get, index, flatten]
  [ARGS]...  update args

Options:
      --sync
          sync registries
      --threads <THREADS>
          threads [default: 2]
      --push
          git push or not
      --check-point-commit <CHECK_POINT_COMMIT>
          set fallback check point commit hash [default: ]
      --registry <REGISTRY>
          vcpkg registry url?branch=&directory=
      --index-directory <INDEX_DIRECTORY>
          vcpkg.index path
      --env-downloads <ENV_DOWNLOADS>
          vcpkg.downloads path
      --env-default-binary-cache <ENV_DEFAULT_BINARY_CACHE>
          vcpkg.archives path
      --path <PATH>
          [default: ]
  -h, --help
          Print help
```
### 3.1.2. configure vcpkg
> asc vcpkg set --registry="https://github.com/microsoft/vcpkg.git?branch=master&directory=D:/asc/data/vcpkg" --index-directory="D:/asc/data/vcpkg.index" --env-downloads="D:/asc/data/vcpkg.downloads" --env-default-binary-cache="D:/asc/data/vcpkg.archives"
```
2024-12-09 14:43:18.4395436  INFO asc::cli::commands::vcpkg: 56: vcpkg registry="https://github.com/microsoft/vcpkg.git?branch=master&directory=D:/asc/data/vcpkg"
```
### 3.1.3. print vcpkg config
> asc vcpkg get
```
2024-12-09 14:47:04.359041  INFO asc::vcpkg::config: 18: VcpkgArgs {
    action: Get,
    args: [],
    registry: [
        "https://github.com/microsoft/vcpkg.git?branch=master&directory=D:/asc/data/vcpkg"
    ],
    index_directory: Some(
        "D:/asc/data/vcpkg.index",
    ),
    env_downloads: Some(
        "D:/asc/data/vcpkg.downloads",
    ),
    env_default_binary_cache: Some(
        "D:/asc/data/vcpkg.archives",
    ),
    path: "",
}
```
### 3.1.4. update vcpkg source
> asc vcpkg update
```
2024-11-01 11:57:46.6303125  INFO asc::cli::commands::vcpkg: 35: vcpkg
2024-11-01 11:57:46.6336549  INFO asc::util::shell: 9: command: git, args: fetch
remote: Enumerating objects: 391, done.
remote: Counting objects: 100% (294/294), done.
remote: Compressing objects: 100% (27/27), done.
remote: Total 391 (delta 270), reused 282 (delta 267), pack-reused 97 (from 1)
Receiving objects: 100% (391/391), 227.54 KiB | 983.00 KiB/s, done.
Resolving deltas: 100% (271/271), completed with 117 local objects.
From https://github.com/microsoft/vcpkg
   97ce57b37..d221c5d2c  master     -> origin/master
2024-11-01 11:57:49.8008798  INFO asc::util::shell: 9: command: git, args: reset --hard origin/master
HEAD is now at d221c5d2c Bot: Close more low quality issues (#41817)
```
### 3.1.5. build search and version index
> asc vcpkg index
```
2024-11-01 13:11:15.851461  INFO asc::cli::commands::vcpkg: 35: vcpkg
2024-11-01 13:11:15.85506  INFO asc::util::shell: 9: command: git, args: log --reverse --date=iso --pretty=format:{"hash": "%H", "date_time": "%ad"}
2024-11-01 13:11:22.3289549  INFO asc::vcpkg::index: 252: [200] #167# "2016-10-03 16:14:27 -0700"
2024-11-01 13:11:28.6886997  INFO asc::vcpkg::index: 252: [400] #284# "2016-10-19 14:27:58 -0700"
2024-11-01 13:11:35.1546592  INFO asc::vcpkg::index: 252: [600] #437# "2016-10-31 14:09:43 -0700"
```


## 3.2. search
### 3.2.1. help
> asc search --help
```
search package with exactly matching name or startswith/endswith/contains text

Usage: asc.exe search [OPTIONS] <NAME>

Arguments:
  <NAME>  extractly match (spdlog), startswith (log*), endswith (*log), contains (*log*)

Options:
      --list  list all versions
  -h, --help  Print help
```
### 3.2.2. search exact package
> asc search spdlog
```
2024-11-01 11:59:09.8075094  INFO asc::cli::commands::search: 33: search name="spdlog"
2024-11-01 11:59:09.8733259  INFO asc::cli::commands::search: 37: spdlog  1.14.1
```
### 3.2.3. search package startswith
> asc search log*
```
2024-11-01 12:01:16.2747683  INFO asc::cli::commands::search: 33: search name="log*"
2024-11-01 12:01:16.3365334  INFO asc::cli::commands::search: 37: log4cplus  2.1.1
2024-11-01 12:01:16.337238  INFO asc::cli::commands::search: 37: log4cplus  2.1.1
2024-11-01 12:01:16.3377419  INFO asc::cli::commands::search: 37: log4cpp-log4cpp  1.1.4
2024-11-01 12:01:16.338256  INFO asc::cli::commands::search: 37: log4cpp-log4cpp  1.1.4
2024-11-01 12:01:16.3384955  INFO asc::cli::commands::search: 37: log4cxx  1.2.0
2024-11-01 12:01:16.338715  INFO asc::cli::commands::search: 37: log4cxx  1.2.0
2024-11-01 12:01:16.3391528  INFO asc::cli::commands::search: 37: loguru  2.1.0#4
2024-11-01 12:01:16.3393477  INFO asc::cli::commands::search: 37: loguru  2.1.0#4
```
### 3.2.4. search package endswith
> asc search *log
```
2024-11-01 12:01:40.6537929  INFO asc::cli::commands::search: 33: search name="*log"
2024-11-01 12:01:40.7166425  INFO asc::cli::commands::search: 37: aixlog  1.5.0#1
2024-11-01 12:01:40.7171608  INFO asc::cli::commands::search: 37: aixlog  1.5.0#1
2024-11-01 12:01:40.7176153  INFO asc::cli::commands::search: 37: binlog  2021-04-16#1
2024-11-01 12:01:40.718025  INFO asc::cli::commands::search: 37: binlog  2021-04-16#1
2024-11-01 12:01:40.7183569  INFO asc::cli::commands::search: 37: boost-log  1.86.0
2024-11-01 12:01:40.7187329  INFO asc::cli::commands::search: 37: boost-log  1.86.0
2024-11-01 12:01:40.7191264  INFO asc::cli::commands::search: 37: g3log  2.4
2024-11-01 12:01:40.7196137  INFO asc::cli::commands::search: 37: g3log  2.4
2024-11-01 12:01:40.7200388  INFO asc::cli::commands::search: 37: glog  0.7.1
2024-11-01 12:01:40.7204601  INFO asc::cli::commands::search: 37: glog  0.7.1
2024-11-01 12:01:40.7207465  INFO asc::cli::commands::search: 37: plog  1.1.10
2024-11-01 12:01:40.72106  INFO asc::cli::commands::search: 37: plog  1.1.10
2024-11-01 12:01:40.7212863  INFO asc::cli::commands::search: 37: spdlog  1.14.1
2024-11-01 12:01:40.7215186  INFO asc::cli::commands::search: 37: spdlog  1.14.1
```
### 3.2.5. search package contains
> asc search \*log\*
```
2024-11-01 12:02:29.1520598  INFO asc::cli::commands::search: 33: search name="*log*"
2024-11-01 12:02:29.2152304  INFO asc::cli::commands::search: 37: aixlog  1.5.0#1
2024-11-01 12:02:29.2157764  INFO asc::cli::commands::search: 37: boost-log  1.86.0
2024-11-01 12:02:29.2162196  INFO asc::cli::commands::search: 37: loguru  2.1.0#4
2024-11-01 12:02:29.2168018  INFO asc::cli::commands::search: 37: boost-logic  1.86.0
2024-11-01 12:02:29.2170384  INFO asc::cli::commands::search: 37: log4cxx  1.2.0
2024-11-01 12:02:29.2173359  INFO asc::cli::commands::search: 37: log4cpp-log4cpp  1.1.4
2024-11-01 12:02:29.2176674  INFO asc::cli::commands::search: 37: plog  1.1.10
2024-11-01 12:02:29.2180768  INFO asc::cli::commands::search: 37: g3log  2.4
2024-11-01 12:02:29.218328  INFO asc::cli::commands::search: 37: nativefiledialog-extended  1.2.1
2024-11-01 12:02:29.2186326  INFO asc::cli::commands::search: 37: spdlog  1.14.1
2024-11-01 12:02:29.2189039  INFO asc::cli::commands::search: 37: portable-file-dialogs  0.1.0
2024-11-01 12:02:29.2191571  INFO asc::cli::commands::search: 37: easyloggingpp  9.97.1#1
2024-11-01 12:02:29.2193985  INFO asc::cli::commands::search: 37: binlog  2021-04-16#1
2024-11-01 12:02:29.2196422  INFO asc::cli::commands::search: 37: glog  0.7.1
2024-11-01 12:02:29.2200787  INFO asc::cli::commands::search: 37: tinyfiledialogs  3.10.8#4
2024-11-01 12:02:29.2203594  INFO asc::cli::commands::search: 37: log4cplus  2.1.1
```
### 3.2.6. list package versions
> asc search spdlog --list
```
2024-11-01 12:02:57.9698138  INFO asc::cli::commands::search: 33: search name="spdlog"
2024-11-01 12:02:58.1810564  INFO asc::cli::commands::search: 37: 1.14.1  20110b4104f8a8cd0d439b7cdb2dbbebf29df939  2024-05-03 13:04:19 +0800
2024-11-01 12:02:58.1817194  INFO asc::cli::commands::search: 37: 1.14.0  41f185a888400c88c43c845adbe3982f3487e05c  2024-04-29 21:07:48 +0800
2024-11-01 12:02:58.1821729  INFO asc::cli::commands::search: 37: 1.13.0#1  50ca16008cebab427e90a98f8ffc34208b215dba  2024-04-10 00:10:21 +0800
2024-11-01 12:02:58.18262  INFO asc::cli::commands::search: 37: 1.13.0  4803f65e11b94719983a69b65b59fdbeca04cdb4  2024-01-16 19:53:44 +0100
2024-11-01 12:02:58.1831454  INFO asc::cli::commands::search: 37: 1.12.0  12e5fc3aede9bb04650280eff7ed4065f4be8f24  2023-07-10 12:54:41 -0400
2024-11-01 12:02:58.1837114  INFO asc::cli::commands::search: 37: 1.11.0#1  656fcc6ab2b05c6d999b7eaca717027ac3738f71  2023-05-21 21:05:01 -0400
2024-11-01 12:02:58.1840697  INFO asc::cli::commands::search: 37: 1.11.0  d51b969a7db84d56d2083bb22b2f95254bdc4c3f  2022-11-17 05:32:00 +0800
2024-11-01 12:02:58.1842584  INFO asc::cli::commands::search: 37: 1.10.0#1  840f701d83d5019aa5033c9d9d08a4cc0d0ebdce  2022-05-23 23:17:10 +0200
2024-11-01 12:02:58.1847217  INFO asc::cli::commands::search: 37: 1.10.0  e794a09b8871d45f106e192682c9aad627e02e16  2022-04-07 18:58:33 +0200
2024-11-01 12:02:58.1850269  INFO asc::cli::commands::search: 37: 1.9.2  6f9e8964e5bf57f68f1b11b9175db4feb2cc322b  2021-09-24 19:47:08 +0200
2024-11-01 12:02:58.1855436  INFO asc::cli::commands::search: 37: 1.9.0#1  4b317d797e0fb3ca0cfa1b47f2c6741284fe5f5c  2021-07-23 08:08:12 +0300
2024-11-01 12:02:58.1857295  INFO asc::cli::commands::search: 37: 1.8.5#4  ab8067a86b8d6e278b4fc08bdcce94115d84c638  2021-07-16 08:00:28 +0900
2024-11-01 12:02:58.1859165  INFO asc::cli::commands::search: 37: 1.8.5#3  2d44beed8551a8af50d2c7db689b5de21e7ae614  2021-07-01 15:36:27 +0100
2024-11-01 12:02:58.1860251  INFO asc::cli::commands::search: 37: 1.8.5#2  b9cd2a7958dec657fb869ec487d2a98cf39a8d48  2021-04-28 03:44:31 +0800
2024-11-01 12:02:58.1862899  INFO asc::cli::commands::search: 37: 1.8.5#1  4a03e7456939043161cebebab3ead59443cd3d21  2021-04-10 00:47:48 +0800
2024-11-01 12:02:58.1867373  INFO asc::cli::commands::search: 37: 1.8.5  f22705fd158e09d432cc23b36ed249ca99386718  2021-03-31 14:27:55 -0700
2024-11-01 12:02:58.1870457  INFO asc::cli::commands::search: 37: 1.8.0#3  6562225c92ec34b92970ce0e4b680856eb6ae24b  2021-03-24 16:47:26 -0300
2024-11-01 12:02:58.1874421  INFO asc::cli::commands::search: 37: 1.8.0#2  8a95605a7b757d7a66f4f6e972780e2eaf62d67d  2021-02-09 06:01:00 +0800
2024-11-01 12:02:58.1877577  INFO asc::cli::commands::search: 37: 1.8.0#1  4596fed3163064b91d4542690939b3fc113d034e  2020-10-29 15:07:48 +0800
2024-11-01 12:02:58.1879025  INFO asc::cli::commands::search: 37: 1.8.0  ea3c975edb2c2bdf645083a1484c88d13d91ace9  2020-10-14 04:18:33 +0800
2024-11-01 12:02:58.1881897  INFO asc::cli::commands::search: 37: 1.7.0  da839ba61a95a07de889d0bcc5d57c1681281a89  2020-08-07 07:10:15 +0200
2024-11-01 12:02:58.1882974  INFO asc::cli::commands::search: 37: 1.6.1  ffe8f5d9b7818c208058b0c3f6a795fba0707db5  2020-06-11 17:42:04 +0800
2024-11-01 12:02:58.188532  INFO asc::cli::commands::search: 37: 1.4.2-1  705764c63549953c049b34c7a2d67b377d0bd006  2020-03-03 01:35:36 +0800
2024-11-01 12:02:58.1887852  INFO asc::cli::commands::search: 37: 1.4.2  ca1e2ec6b30a0a3830fca950a9dd0b55202fd1ec  2019-11-26 06:35:53 +0800
2024-11-01 12:02:58.1890223  INFO asc::cli::commands::search: 37: 1.3.1-2  514ad6542cc088a23b32e8e17a4defb2ddbed5b9  2019-08-16 02:47:13 +0800
2024-11-01 12:02:58.1891551  INFO asc::cli::commands::search: 37: 1.3.1-1  49d1759ec85c31eb50d07b42d245b989b99df37c  2019-06-19 02:11:06 +0800
2024-11-01 12:02:58.1892912  INFO asc::cli::commands::search: 37: 1.3.1  18b029a5e3997fa4fdc7d3d06d56568a1d6f74ad  2019-06-16 02:54:47 +0300
2024-11-01 12:02:58.1895468  INFO asc::cli::commands::search: 37: 1.3.0  57f7f49aba15b05d3093445bb813c1f5cfbbd6c5  2019-01-14 20:31:58 +0800
2024-11-01 12:02:58.1897321  INFO asc::cli::commands::search: 37: 1.2.1  3d164e3e4c19e54763fe34bcb9fe2e5bdcafc1d0  2018-12-20 15:00:34 -0800
2024-11-01 12:02:58.189849  INFO asc::cli::commands::search: 37: 1.2.0  97e9d96715c6d1ad3501b47c02f76f92c1221a99  2018-10-25 21:07:35 +0300
2024-11-01 12:02:58.1899081  INFO asc::cli::commands::search: 37: 1.0.0  b641590c9155fc38143eddc5897aae8b0b4caa16  2018-08-06 10:08:30 +0200
2024-11-01 12:02:58.189957  INFO asc::cli::commands::search: 37: 1.x-2018-07.04  a382578f19e35e4fec6edd72dde35b1e03c684b7  2018-07-04 18:54:04 -0700
2024-11-01 12:02:58.1900052  INFO asc::cli::commands::search: 37: 0.17.0  3152af9025cc1413d23340a1e70a0eaa4ac453d6  2018-06-13 10:46:28 -0700
2024-11-01 12:02:58.1900524  INFO asc::cli::commands::search: 37: 0.16.3  43172e23188cc8014ac1b73bca8b24f664ff3324  2018-02-23 03:27:49 -0800
2024-11-01 12:02:58.1900997  INFO asc::cli::commands::search: 37: 0.14.0-1  3a6ad750c922cf8d2eec67ad90deb85646078a3e  2017-09-10 02:40:01 +0300
2024-11-01 12:02:58.1901484  INFO asc::cli::commands::search: 37: 0.14.0  3d111fedeac82a36aff98d83b5a64834e5c1b671  2017-08-19 20:00:54 +0200
2024-11-01 12:02:58.1901954  INFO asc::cli::commands::search: 37: 0.13.0  5f5e6b740b7d7f1f1c8b88744068d468fb959870  2017-05-20 13:01:31 +0200
2024-11-01 12:02:58.1902747  INFO asc::cli::commands::search: 37: 0.12.0  b4fa5bca114b30df6086467297cee79a0b6b0826  2017-02-25 16:57:03 +0100
2024-11-01 12:02:58.1903674  INFO asc::cli::commands::search: 37: 0.11.0  0bfa90975b50723ca94fde3c5dd2306db980bff4  2016-10-26 19:44:28 -0700
```

## 3.3. new
### 3.3.1. help
> asc new --help
```
new package/workspace of binary/static library/shared library

Usage: asc.exe new [OPTIONS] [NAME]

Arguments:
  [NAME]  new package/workspace name

Options:
      --lib              new library (default bin)
      --shared           new shared library (default static library)
      --workspace        new workspace (default package)
      --member <MEMBER>  new workspace members (--member=a --member=b --member=c)
  -h, --help             Print help
```
### 3.3.2. new package
#### 3.3.2.1. new binary package
> asc new test_pkg_bin
```
2024-11-01 13:21:43.4893593  INFO asc::cli::commands::new: 39: new bin name="test_pkg_bin"
2024-11-01 13:21:43.4896546  INFO asc::cli::commands::new: 185: new package name="test_pkg_bin"
2024-11-01 13:21:43.5308051  INFO asc::cli::commands::init: 44: init package name="test_pkg_bin"
```
> cd test_pkg_bin

> tree /f
```
│  asc.toml
│
└─src
        main.cpp
```
#### 3.3.2.2. new static or shared library package
> asc new --lib test_pkg_lib

> asc new --lib --shared test_pkg_lib
```
2024-11-01 13:23:24.8902102  INFO asc::cli::commands::new: 60: new lib name="test_pkg_lib"
2024-11-01 13:23:24.8903728  INFO asc::cli::commands::new: 185: new package name="test_pkg_lib"
2024-11-01 13:23:24.9010554  INFO asc::cli::commands::init: 44: init package name="test_pkg_lib"
```
> cd test_pkg_lib

> tree /f
```
│  asc.toml
│
└─src
        export.h
        lib.cpp
        lib.hpp
```
### 3.3.3. new workspace
#### 3.3.3.1. new binary package
> asc new test_ws_bin --workspace --member=a --member=b --member=c
```
2024-11-01 13:25:39.7343758  INFO asc::cli::commands::new: 237: new workspace name="test_ws_bin"
2024-11-01 13:25:39.7510082  INFO asc::cli::commands::new: 39: new bin name="a"
2024-11-01 13:25:39.7512986  INFO asc::cli::commands::new: 185: new package name="a"
2024-11-01 13:25:39.7519888  INFO asc::cli::commands::init: 44: init package name="a"
2024-11-01 13:25:39.7536099  INFO asc::cli::commands::new: 39: new bin name="b"
2024-11-01 13:25:39.7539051  INFO asc::cli::commands::new: 185: new package name="b"
2024-11-01 13:25:39.7543782  INFO asc::cli::commands::init: 44: init package name="b"
2024-11-01 13:25:39.7555566  INFO asc::cli::commands::new: 39: new bin name="c"
2024-11-01 13:25:39.7557864  INFO asc::cli::commands::new: 185: new package name="c"
2024-11-01 13:25:39.7560625  INFO asc::cli::commands::init: 44: init package name="c"
```
> cd test_ws_bin

> tree /f
```
│  asc.toml
│
├─a
│  │  asc.toml
│  │
│  └─src
│          main.cpp
│
├─b
│  │  asc.toml
│  │
│  └─src
│          main.cpp
│
└─c
    │  asc.toml
    │
    └─src
            main.cpp
```
#### 3.3.3.2. new static or shared library package
> asc new --lib test_ws_lib --workspace --member=a --member=b --member=c

> asc new --lib --shared test_ws_lib --workspace --member=a --member=b --member=c
```
2024-11-01 13:26:55.2823825  INFO asc::cli::commands::new: 237: new workspace name="test_ws_lib"
2024-11-01 13:26:55.2828598  INFO asc::cli::commands::new: 60: new lib name="a"
2024-11-01 13:26:55.2829963  INFO asc::cli::commands::new: 185: new package name="a"
2024-11-01 13:26:55.2959096  INFO asc::cli::commands::init: 44: init package name="a"
2024-11-01 13:26:55.2992042  INFO asc::cli::commands::new: 60: new lib name="b"
2024-11-01 13:26:55.2994754  INFO asc::cli::commands::new: 185: new package name="b"
2024-11-01 13:26:55.2998603  INFO asc::cli::commands::init: 44: init package name="b"
2024-11-01 13:26:55.3025409  INFO asc::cli::commands::new: 60: new lib name="c"
2024-11-01 13:26:55.3027336  INFO asc::cli::commands::new: 185: new package name="c"
2024-11-01 13:26:55.3034086  INFO asc::cli::commands::init: 44: init package name="c"
```
> cd test_ws_lib

> tree /f
```
│  asc.toml
│
├─a
│  │  asc.toml
│  │
│  └─src
│          export.h
│          lib.cpp
│          lib.hpp
│
├─b
│  │  asc.toml
│  │
│  └─src
│          export.h
│          lib.cpp
│          lib.hpp
│
└─c
    │  asc.toml
    │
    └─src
            export.h
            lib.cpp
            lib.hpp
```


## 3.4. init
### 3.4.1. help
> asc init --help
```
init directory as package/workspace of binary/static library/shared library

Usage: asc.exe init [OPTIONS]

Options:
      --lib              new library (default bin)
      --shared           new shared library (default static library)
      --workspace        new workspace (default package)
      --member <MEMBER>  new workspace members (--member=a --member=b --member=c)
  -h, --help             Print help
```
### 3.4.2. init package
#### 3.4.2.1. init binary package
> cd exists_pkg_bin

> tree /f
```
└─src
        main.cpp
```
> asc init
```
2024-11-01 13:33:46.5208747  INFO asc::cli::commands::init: 34: init bin name="exists_pkg_bin"
2024-11-01 13:33:46.5213205  INFO asc::cli::commands::init: 44: init package name="exists_pkg_bin"
```
> tree /f
```
│  asc.toml
│
└─src
        main.cpp
```
#### 3.4.2.2. init static or shared library package
> cd exists_pkg_lib

> tree /f
```
└─src
        export.h
        lib.cpp
        lib.hpp
```
> asc init --lib

> asc init --lib --shared
```
2024-11-01 13:35:58.8920565  INFO asc::cli::commands::init: 39: init bin name="exists_pkg_lib"
2024-11-01 13:35:58.892298  INFO asc::cli::commands::init: 44: init package name="exists_pkg_lib"
```
> tree /f
```
│  asc.toml
│
└─src
        export.h
        lib.cpp
        lib.hpp
```
### 3.4.3. init workspace
#### 3.4.3.1. init binary workspace
> cd exists_ws_bin

> tree /f
```
├─a
│  └─src
│          main.cpp
│
├─b
│  └─src
│          main.cpp
│
└─c
    └─src
            main.cpp
```
> asc init --workspace --member=a --member=b --member=c
```
2024-11-01 13:37:23.1650265  INFO asc::cli::commands::init: 71: init workspace name="D:/sources/asc/exists_ws_bin"
2024-11-01 13:37:23.1653402  INFO asc::cli::commands::init: 44: init package name="a"
2024-11-01 13:37:23.166521  INFO asc::cli::commands::init: 44: init package name="b"
2024-11-01 13:37:23.1671192  INFO asc::cli::commands::init: 44: init package name="c"
```
> tree /f
```
│  asc.toml
│
├─a
│  │  asc.toml
│  │
│  └─src
│          main.cpp
│
├─b
│  │  asc.toml
│  │
│  └─src
│          main.cpp
│
└─c
    │  asc.toml
    │
    └─src
            main.cpp
```
#### 3.4.3.2. init static or shared library workspace
> cd exists_ws_lib

> tree /f
```
├─a
│  └─src
│          export.h
│          lib.cpp
│          lib.hpp
│
├─b
│  └─src
│          export.h
│          lib.cpp
│          lib.hpp
│
└─c
    └─src
            export.h
            lib.cpp
            lib.hpp
```
> asc init --lib --workspace --member=a --member=b --member=c

> asc init --lib --shared --workspace --member=a --member=b --member=c
```
2024-11-01 13:38:45.3604913  INFO asc::cli::commands::init: 71: init workspace name="D:/sources/asc/exists_ws_lib"
2024-11-01 13:38:45.3611687  INFO asc::cli::commands::init: 44: init package name="a"
2024-11-01 13:38:45.3625186  INFO asc::cli::commands::init: 44: init package name="b"
2024-11-01 13:38:45.3635302  INFO asc::cli::commands::init: 44: init package name="c"
```
> tree /f
```
│  asc.toml
│
├─a
│  │  asc.toml
│  │
│  └─src
│          export.h
│          lib.cpp
│          lib.hpp
│
├─b
│  │  asc.toml
│  │
│  └─src
│          export.h
│          lib.cpp
│          lib.hpp
│
└─c
    │  asc.toml
    │
    └─src
            export.h
            lib.cpp
            lib.hpp
```


## 3.5. add
### 3.5.1. help
> asc add --help
```
add dependency to package or workspace member's asc.toml

Usage: asc.exe add [OPTIONS] <DEPENDENCY>

Arguments:
  <DEPENDENCY>  dependency name

Options:
      --package <PACKAGE>
          workspace member name
      --version <VERSION>
          dependency version (default latest) [default: ]
      --find-package <FIND_PACKAGE>
          for cmake find_package (--find-package=a --find-package=b@!windows)
      --include-directory <INCLUDE_DIRECTORY>
          for cmake target_include_directories (--include-directory=c -include-directory=d)
      --link-library <LINK_LIBRARY>
          for cmake target_link_libraries (--link-library=e --link-library=f)
      --feature <FEATURE>
          for vcpkg manifest (--feature=g --feature=h)
  -h, --help
          Print help
```
### 3.5.2. add latest version of package dependency to package configuration
> cd test_package

> cat asc.toml
```toml
[package]
name = "test_package"
version = "2024.11.1"
edition = "2024"

[features]
```
> asc add cli11 --find-package="CLI11" --link-library="CLI11::CLI11"
```
2024-11-01 14:05:07.3232681  INFO asc::cli::commands::add: 27: add dependency="cli11"
```
> asc add openssl --find-package="openssl" --link-library="OpenSSL::Crypto" --link-library="OpenSSL::SSL"
```
2024-11-01 14:06:26.3395245  INFO asc::cli::commands::add: 27: add dependency="openssl"
```
> cat asc.toml
```toml
[package]
name = "test_package"
version = "2024.11.1"
edition = "2024"

[features]

[dependencies]
cli11 = { version = "2.4.2#1", find_packages = ["CLI11"], include_directories = [], link_libraries = ["CLI11::CLI11"], features = [] }
openssl = { version = "3.4.0", find_packages = ["openssl"], include_directories = [], link_libraries = ["OpenSSL::Crypto", "OpenSSL::SSL"], features = [] }

```
### 3.5.3. add specific version of package dependency to workspace's sub package configuration
> cd test_workspace

> asc add cli11 --version="2.3.2" --find-package="CLI11" --link-library="CLI11::CLI11" --package=a
```
2024-11-01 14:16:15.6369184  INFO asc::cli::commands::add: 27: add dependency="cli11"
```
> cat a/asc.toml
```
[package]
name = "a"
version = "2024.10.31"
edition = "2024"

cli11 = { version = "2.3.2", find_packages = ["CLI11"], include_directories = [], link_libraries = ["CLI11::CLI11"], features = [] }

```
> asc add fmt --version="10.0.0" --find-package="fmt" --link-library="fmt::fmt" --package=b
```
2024-11-01 14:17:56.0398587  INFO asc::cli::commands::add: 27: add dependency="fmt"
```
> cat b/asc.toml
```
[package]
name = "b"
version = "2024.10.31"
edition = "2024"

[dependencies]
fmt = { version = "10.0.0", find_packages = ["fmt"], include_directories = [], link_libraries = ["fmt::fmt"], features = [] }

```
> asc add spdlog --version="1.11.0#1" --find-package="spdlog" --link-library="spdlog::spdlog" --package=c
```
2024-11-01 14:18:51.9689075  INFO asc::cli::commands::add: 27: add dependency="spdlog"
```
> cat c/asc.toml
```
[package]
name = "c"
version = "2024.10.31"
edition = "2024"

[dependencies]
spdlog = { version = "1.11.0#1", find_packages = ["spdlog"], include_directories = [], link_libraries = ["spdlog::spdlog"], features = [] }

```
### 3.5.6. add package dependency with features to workspace's sub package configuration
> asc add arrow --feature=json --feature=mimalloc@windows --package=c
```
2024-11-01 14:21:32.3342819  INFO asc::cli::commands::add: 27: add dependency="arrow"
```
> cat c/asc.toml
```toml
[package]
name = "c"
version = "2024.10.31"
edition = "2024"

[features]

[dependencies]
arrow = { version = "17.0.0", find_packages = [], link_libraries = [], include_directories = [], features = ["json", "mimalloc@windows"] }
fmt = { version = "10.0.0", find_packages = ["fmt"], include_directories = [], link_libraries = ["fmt::fmt"], features = [] }
spdlog = { version = "1.11.0#1", find_packages = ["spdlog"], include_directories = [], link_libraries = ["spdlog::spdlog"], features = [] }

```

## 3.6. remove
### 3.6.1. help
> asc remove --help
```
remove dependency from package or workspace member's asc.toml

Usage: asc.exe remove [OPTIONS] <DEPENDENCY>

Arguments:
  <DEPENDENCY>  dependency name

Options:
      --package <PACKAGE>  workspace member name
  -h, --help               Print help
```
### 3.6.2. remove specific package dependency
> asc remove cli11
```
2024-11-01 14:08:46.1405413  INFO asc::cli::commands::remove: 18: remove dependency="cli11"
2024-11-01 14:08:46.1410159  INFO asc::dependency::remove: 4: remove dependency="cli11"
```
> asc remove openssl
```
2024-11-01 14:09:19.8021804  INFO asc::cli::commands::remove: 18: remove dependency="openssl"
2024-11-01 14:09:19.8026104  INFO asc::dependency::remove: 4: remove dependency="openssl"
```
> cat asc.toml
```toml
[package]
name = "test_package"
version = "2024.11.1"
edition = "2024"
```


## 3.7. scan
### 3.7.1. help
> asc scan --help
```
scan necessary sources, generate cmake and vcpkg configurations

Usage: asc.exe scan [OPTIONS]

Options:
      --cmake-minimum-version <CMAKE_MINIMUM_VERSION>
          for cmake cmake_minimum_required [default: 3.20]
  -h, --help
          Print help
```
### 3.7.2. scan package
> cd test_package

> asc scan
```
2024-11-01 14:29:14.8536081  INFO asc::cli::commands::scan: 93: scan package name="test_package"
2024-11-01 14:29:14.8540759  INFO asc::cli::commands::scan: 118: ScanOptions {
    project: "test_package",
    project_dir: "D:/sources/asc/test_package",
    target_dir: "D:/sources/asc/test_package/target",
    source_dir: "D:/sources/asc/test_package/src",
    entry_point_source: "D:/sources/asc/test_package/src/main.cpp",
    include_directories: [],
    shared_lib: false,
    static_lib: false,
    cmake_minimum_version: "3.20",
    cmake_config: "",
}
2024-11-01 14:29:14.854853  WARN asc::cli::commands::scan: 125: scan source dependencies with clang ir
2024-11-01 14:29:15.7403074  INFO asc::clang::visitor: 71: main.cpp
2024-11-01 14:29:15.7410044  INFO asc::clang::visitor: 77:     wrapping.hpp
2024-11-01 14:29:15.7415637  INFO asc::clang::visitor: 77:     config.h
2024-11-01 14:29:15.7420946  INFO asc::clang::visitor: 77:     version.h
2024-11-01 14:29:15.7759488  INFO asc::clang::visitor: 71: wrapping.hpp
2024-11-01 14:29:15.776614  INFO asc::clang::visitor: 77:     a/mod.hpp
2024-11-01 14:29:15.7771992  INFO asc::clang::visitor: 77:     b/mod.hpp
2024-11-01 14:29:15.7777168  INFO asc::clang::visitor: 77:     c/mod.hpp
2024-11-01 14:29:15.7975036  INFO asc::clang::visitor: 71: a/mod.hpp
2024-11-01 14:29:15.7980344  INFO asc::clang::visitor: 77:     a/a.h
2024-11-01 14:29:15.7984452  INFO asc::clang::visitor: 77:     a/a.hpp
2024-11-01 14:29:15.8202776  INFO asc::clang::visitor: 71: a/a.h
2024-11-01 14:29:15.8583435  INFO asc::clang::visitor: 71: a/a.hpp
2024-11-01 14:29:15.8590029  INFO asc::clang::visitor: 77:     export.h
2024-11-01 14:29:15.8700152  INFO asc::clang::visitor: 71: export.h
2024-11-01 14:29:15.8823024  INFO asc::clang::visitor: 71: b/mod.hpp
2024-11-01 14:29:15.8826303  INFO asc::clang::visitor: 77:     b/b.h
2024-11-01 14:29:15.8830551  INFO asc::clang::visitor: 77:     b/b.hpp
2024-11-01 14:29:15.8947908  INFO asc::clang::visitor: 71: b/b.h
2024-11-01 14:29:15.9025291  INFO asc::clang::visitor: 71: b/b.hpp
2024-11-01 14:29:15.914299  INFO asc::clang::visitor: 71: c/mod.hpp
2024-11-01 14:29:15.9146903  INFO asc::clang::visitor: 77:     c/c.h
2024-11-01 14:29:15.9150802  INFO asc::clang::visitor: 77:     c/c.hpp
2024-11-01 14:29:15.9268767  INFO asc::clang::visitor: 71: c/c.h
2024-11-01 14:29:15.9380827  INFO asc::clang::visitor: 71: c/c.hpp
2024-11-01 14:29:15.9493361  INFO asc::clang::visitor: 71: config.h
2024-11-01 14:29:15.9593627  INFO asc::clang::visitor: 71: version.h
2024-11-01 14:29:15.9666588  INFO asc::clang::visitor: 71: a/a.c
2024-11-01 14:29:15.9672373  INFO asc::clang::visitor: 77:     a/a.h
2024-11-01 14:29:15.9786484  INFO asc::clang::visitor: 71: a/a.cpp
2024-11-01 14:29:15.9792551  INFO asc::clang::visitor: 77:     a/a.hpp
2024-11-01 14:29:15.9912274  INFO asc::clang::visitor: 71: b/b.c
2024-11-01 14:29:15.9916376  INFO asc::clang::visitor: 77:     b/b.h
2024-11-01 14:29:15.9991088  INFO asc::clang::visitor: 71: b/b.cpp
2024-11-01 14:29:15.9995026  INFO asc::clang::visitor: 77:     b/b.hpp
2024-11-01 14:29:16.0106323  INFO asc::clang::visitor: 71: c/c.c
2024-11-01 14:29:16.0110075  INFO asc::clang::visitor: 77:     c/c.h
2024-11-01 14:29:16.0184897  INFO asc::clang::visitor: 71: c/c.cpp
2024-11-01 14:29:16.0188931  INFO asc::clang::visitor: 77:     c/c.hpp
2024-11-01 14:29:16.027834  INFO asc::clang::visitor: 71: d/d.c
2024-11-01 14:29:16.0284265  INFO asc::clang::visitor: 77:     d/d.h
2024-11-01 14:29:16.0373911  INFO asc::clang::visitor: 71: d/d.h
2024-11-01 14:29:16.0447303  INFO asc::clang::visitor: 71: d/d.cc
2024-11-01 14:29:16.0450196  INFO asc::clang::visitor: 77:     d/d.hpp
2024-11-01 14:29:16.0526325  INFO asc::clang::visitor: 71: d/d.hpp
2024-11-01 14:29:16.0528955  INFO asc::clang::visitor: 77:     export.h
2024-11-01 14:29:16.080282  INFO asc::clang::visitor: 71: lib.cpp
2024-11-01 14:29:16.0807834  INFO asc::clang::visitor: 77:     lib.hpp
2024-11-01 14:29:16.0905869  INFO asc::clang::visitor: 71: lib.hpp
2024-11-01 14:29:16.0912471  INFO asc::clang::visitor: 77:     wrapping.hpp
2024-11-01 14:29:16.1006232  INFO asc::clang::visitor: 71: test.cpp
2024-11-01 14:29:16.1012459  INFO asc::clang::visitor: 77:     d/mod.hpp
2024-11-01 14:29:16.1094027  INFO asc::clang::visitor: 71: d/mod.hpp
2024-11-01 14:29:16.109931  INFO asc::clang::visitor: 77:     d/d.h
2024-11-01 14:29:16.1103609  INFO asc::clang::visitor: 77:     d/d.hpp
2024-11-01 14:29:16.1110395  WARN asc::cli::commands::scan: 128: output flow chart flowchart.md
2024-11-01 14:29:16.1116985  INFO asc::cli::commands::scan: 130:
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
    a/a.hpp ---> export.h;
    d/d.hpp ---> export.h;
    lib.hpp ---> wrapping.hpp;
    main.cpp ---> wrapping.hpp;
    main.cpp ---> config.h;
    main.cpp ---> version.h;
2024-11-01 14:29:16.1121871  WARN asc::cli::commands::scan: 132: output CMakeLists.txt
2024-11-01 14:29:16.1257795  WARN asc::cli::commands::scan: 136: generate vcpkg manifest
2024-11-01 14:29:16.5113435  INFO asc::util::shell: 9: command: git, args: show 656fcc6ab2b05c6d999b7eaca717027ac3738f71:versions/baseline.json
2024-11-01 14:29:16.5636486  INFO asc::vcpkg::json: 117: set baseline to 656fcc6ab2b05c6d999b7eaca717027ac3738f71 @ 2023-05-21 21:05:01 -0400
2024-11-01 14:29:16.5657156  INFO asc::util::shell: 9: command: git, args: log -n 1 --date=iso --pretty=format:{"hash": "%H", "date_time": "%ad"}
2024-11-01 14:29:16.5989139  WARN asc::cli::commands::scan: 139: generate a build system with cmake
2024-11-01 14:29:16.5996138  INFO asc::util::shell: 9: command: cmake, args: -S D:/sources/asc/test_package -B D:/sources/asc/test_package/target -D CMAKE_TOOLCHAIN_FILE=C:/Users/capric/AppData/Roaming/asc/data/vcpkg/scripts/buildsystems/vcpkg.cmake -D VCPKG_TARGET_TRIPLET=x64-windows-static -D VCPKG_HOST_TRIPLET=x64-windows-static
-- Building for: Visual Studio 17 2022
-- Running vcpkg install
Fetching registry information from https://github.com/microsoft/vcpkg.git (HEAD)...
Detecting compiler hash for triplet x64-windows-static...
-- Using %HTTP(S)_PROXY% in environment variables.
Compiler found: D:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.41.34120/bin/Hostx64/x64/cl.exe
The following packages will be built and installed:
    cli11:x64-windows-static@2.3.1 -- C:\Users\capric\AppData\Local\vcpkg\registries\git-trees\47f8293bf52200e08a166ac4e22bee925d63f04a
    fmt:x64-windows-static@10.0.0 -- C:\Users\capric\AppData\Local\vcpkg\registries\git-trees\eedb31bb1318118ec6a2d1bec60ab12e484092fd
    spdlog:x64-windows-static@1.11.0#1 -- C:\Users\capric\AppData\Local\vcpkg\registries\git-trees\67656948712582d93d9096cc08871a3270908d38
  * vcpkg-cmake:x64-windows-static@2024-04-23 -- C:\Users\capric\AppData\Local\vcpkg\registries\git-trees\e74aa1e8f93278a8e71372f1fa08c3df420eb840
  * vcpkg-cmake-config:x64-windows-static@2024-05-23 -- C:\Users\capric\AppData\Local\vcpkg\registries\git-trees\97a63e4bc1a17422ffe4eff71da53b4b561a7841
Additional packages (*) will be modified to complete this operation.
Restored 5 package(s) from C:\Users\capric\AppData\Local\vcpkg\archives in 445 ms. Use --debug to see more details.
Installing 1/5 vcpkg-cmake-config:x64-windows-static@2024-05-23...
Elapsed time to handle vcpkg-cmake-config:x64-windows-static: 7.09 ms
vcpkg-cmake-config:x64-windows-static package ABI: a31f83659e20554d5ca89ceeb4563ce6551b88a5db35806ec45cb493434200e1
Installing 2/5 vcpkg-cmake:x64-windows-static@2024-04-23...
Elapsed time to handle vcpkg-cmake:x64-windows-static: 24.8 ms
vcpkg-cmake:x64-windows-static package ABI: 1e256ea136c3323e6b541f7b0b670c4fb13e8062a69faea28b1717295f0a4bfd
Installing 3/5 cli11:x64-windows-static@2.3.1...
Elapsed time to handle cli11:x64-windows-static: 41.5 ms
cli11:x64-windows-static package ABI: b127f3a824e2fc2976d433a5c7e230c89d5bb0f3c5576d750ae6b043a1687192
Installing 4/5 fmt:x64-windows-static@10.0.0...
Elapsed time to handle fmt:x64-windows-static: 58.5 ms
fmt:x64-windows-static package ABI: df4f2540bcc17e7eb2ff77b319e89587a73a6dbae892d4224b3b4f2202d97c84
Installing 5/5 spdlog:x64-windows-static@1.11.0#1...
Elapsed time to handle spdlog:x64-windows-static: 111 ms
spdlog:x64-windows-static package ABI: 7c403f90c6298d72aee58d95cb76f6621bc16b3865ed7ed9e6f8f0f0f212f3e6
Total install time: 243 ms
cli11 provides CMake targets:

  # this is heuristically generated, and may not be correct
  find_package(CLI11 CONFIG REQUIRED)
  target_link_libraries(main PRIVATE CLI11::CLI11)

cli11 provides pkg-config modules:

  # C++ command line parser
  CLI11

The package fmt provides CMake targets:

    find_package(fmt CONFIG REQUIRED)
    target_link_libraries(main PRIVATE fmt::fmt)

    # Or use the header-only version
    find_package(fmt CONFIG REQUIRED)
    target_link_libraries(main PRIVATE fmt::fmt-header-only)

The package spdlog provides CMake targets:

    find_package(spdlog CONFIG REQUIRED)
    target_link_libraries(main PRIVATE spdlog::spdlog)

    # Or use the header-only version
    find_package(spdlog CONFIG REQUIRED)
    target_link_libraries(main PRIVATE spdlog::spdlog_header_only)

-- Running vcpkg install - done
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
-- Performing Test CMAKE_HAVE_LIBC_PTHREAD
-- Performing Test CMAKE_HAVE_LIBC_PTHREAD - Failed
-- Looking for pthread_create in pthreads
-- Looking for pthread_create in pthreads - not found
-- Looking for pthread_create in pthread
-- Looking for pthread_create in pthread - not found
-- Found Threads: TRUE
-- Configuring done (47.3s)
-- Generating done (0.0s)
-- Build files have been written to: D:/sources/asc/test_package/target
```



## 3.8. build
### 3.8.1. help
> asc build --help
```
build all, package or workspace member

Usage: asc.exe build [OPTIONS]

Options:
      --target <TARGET>  build single target (default all)
      --release          release mode (default false)
  -h, --help             Print help
```
### 3.8.2. build package
> cd test_package

> asc build
```
2024-11-01 14:34:16.0123129  INFO asc::cli::commands::build: 16: build name="test_package"
2024-11-01 14:34:16.0132969  INFO asc::util::shell: 9: command: cmake, args: --build target
MSBuild version 17.11.9+a69bbaaf5 for .NET Framework

  1>Checking Build System
  Building Custom Rule D:/sources/asc/test_package/CMakeLists.txt
  main.cpp
  a.c
  a.cpp
  b.c
  b.cpp
  c.c
  c.cpp
LINK : warning LNK4075: ignoring '/INCREMENTAL' due to '/OPT:ICF' specification [D:\__develop__\FutureOrientedGB\asc\test_package\target\test_package.vcxproj]
  test_package.vcxproj -> D:\__develop__\FutureOrientedGB\asc\test_package\target\Debug\test_package.exe
  Building Custom Rule D:/sources/asc/test_package/CMakeLists.txt
```
### 3.8.3. build workspace
> cd test_workspace

> asc build
```
2024-11-01 14:34:59.9977919  INFO asc::cli::commands::build: 16: build name="test_workspace"
2024-11-01 14:34:59.9983509  INFO asc::util::shell: 9: command: cmake, args: --build target
MSBuild version 17.11.9+a69bbaaf5 for .NET Framework

  1>Checking Build System
  Building Custom Rule D:/sources/asc/test_workspace/a/CMakeLists.txt
  lib.cpp
LINK : warning LNK4075: ignoring '/INCREMENTAL' due to '/OPT:ICF' specification [D:\__develop__\FutureOrientedGB\asc\test_workspace\target\a\a.vcxproj]
     Creating library D:/sources/asc/test_workspace/target/a/Debug/a.lib and object D:/sources/asc/test_workspace/target/a/Debug/a.exp
  a.vcxproj -> D:\__develop__\FutureOrientedGB\asc\test_workspace\target\a\Debug\a.dll
  Building Custom Rule D:/sources/asc/test_workspace/b/CMakeLists.txt
  lib.cpp
LINK : warning LNK4075: ignoring '/INCREMENTAL' due to '/OPT:ICF' specification [D:\__develop__\FutureOrientedGB\asc\test_workspace\target\b\b.vcxproj]
     Creating library D:/sources/asc/test_workspace/target/b/Debug/b.lib and object D:/sources/asc/test_workspace/target/b/Debug/b.exp
  b.vcxproj -> D:\__develop__\FutureOrientedGB\asc\test_workspace\target\b\Debug\b.dll
  Building Custom Rule D:/sources/asc/test_workspace/c/CMakeLists.txt
  lib.cpp
LINK : warning LNK4075: ignoring '/INCREMENTAL' due to '/OPT:ICF' specification [D:\__develop__\FutureOrientedGB\asc\test_workspace\target\c\c.vcxproj]
     Creating library D:/sources/asc/test_workspace/target/c/Debug/c.lib and object D:/sources/asc/test_workspace/target/c/Debug/c.exp
  c.vcxproj -> D:\__develop__\FutureOrientedGB\asc\test_workspace\target\c\Debug\c.dll
  Building Custom Rule D:/sources/asc/test_workspace/CMakeLists.txt
```
### 3.8.4. build release mode
> asc build --config=Release
```
2024-11-01 14:35:47.3051576  INFO asc::cli::commands::build: 16: build name="test_workspace"
2024-11-01 14:35:47.3056953  INFO asc::util::shell: 9: command: cmake, args: --build target --release
MSBuild version 17.11.9+a69bbaaf5 for .NET Framework

  1>Checking Build System
  Building Custom Rule D:/sources/asc/test_workspace/a/CMakeLists.txt
  lib.cpp
     Creating library D:/sources/asc/test_workspace/target/a/Release/a.lib and object D:/sources/asc/test_workspace/target/a/Release/a.exp
  a.vcxproj -> D:\__develop__\FutureOrientedGB\asc\test_workspace\target\a\Release\a.dll
  Building Custom Rule D:/sources/asc/test_workspace/b/CMakeLists.txt
  lib.cpp
     Creating library D:/sources/asc/test_workspace/target/b/Release/b.lib and object D:/sources/asc/test_workspace/target/b/Release/b.exp
  b.vcxproj -> D:\__develop__\FutureOrientedGB\asc\test_workspace\target\b\Release\b.dll
  Building Custom Rule D:/sources/asc/test_workspace/c/CMakeLists.txt
  lib.cpp
     Creating library D:/sources/asc/test_workspace/target/c/Release/c.lib and object D:/sources/asc/test_workspace/target/c/Release/c.exp
  c.vcxproj -> D:\__develop__\FutureOrientedGB\asc\test_workspace\target\c\Release\c.dll
  Building Custom Rule D:/sources/asc/test_workspace/CMakeLists.txt
```


## 3.9. run
### 3.9.1. help
> asc vcpkg run --help
```
run package or workspace member binary

Usage: asc.exe run [OPTIONS]

Options:
      --bin <BIN>    binary name
      --args <ARGS>  command line arguments
      --release      release mode (default false)
  -h, --help         Print help
```
### 3.9.2. run package target
> asc run --config=Debug
```
2024-11-01 14:37:41.1921068  INFO asc::cli::commands::run: 19: run
2024-11-01 14:37:41.1933882  INFO asc::util::shell: 9: command: target/Debug/test_package, args:
2024.11.1
NOT HAVE_GETTIMEOFDAY
```
### 3.9.3. run workspace member package
> asc new abc --workspace --member=a --member=b --member=c

> cd abc

> asc scan

> asc build

> asc run --bin a --confg=Debug
```
2024-11-01 14:41:39.6262125  INFO asc::cli::commands::run: 19: run
2024-11-01 14:41:39.6272723  INFO asc::util::shell: 9: command: target/a/Debug/a, args:
Hello, world!
```

## 3.10. install
### 3.10.1. help
> asc install --help
```
install executable/headers/libraries

Usage: asc.exe install [OPTIONS]

Options:
      --prefix <PREFIX>      install prefix [default: target/installed]
      --release              release mode (default false)
      --pack-cli <PACK_CLI>  package cli (7z, tar, iscc, auto .7z on windows .tar.xz on others) [default: ]
  -h, --help                 Print help
```
### 3.10.2. install files to default directory
> asc install
```
2024-11-01 14:43:13.4602646  INFO asc::cli::commands::install: 17: install name="test_workspace"
2024-11-01 14:43:13.460973  INFO asc::util::shell: 9: command: cmake, args: --install target --prefix target/installed/x64-windows-static
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/lib/a.lib
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/bin/a.dll
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/include/a/export.h
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/include/a/lib.hpp
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/share/a/a-config.cmake
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/lib/b.lib
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/bin/b.dll
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/include/b/export.h
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/include/b/lib.hpp
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/share/b/b-config.cmake
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/lib/c.lib
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/bin/c.dll
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/include/c/export.h
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/include/c/lib.hpp
-- Installing: D:/sources/asc/test_workspace/target/installed/x64-windows-static/share/c/c-config.cmake
```
### 3.10.2. install files to specific directory
> asc install --prefix=d:/test_dir
```
2024-11-01 14:44:34.2393421  INFO asc::cli::commands::install: 17: install name="test_workspace"
2024-11-01 14:44:34.2400124  INFO asc::util::shell: 9: command: cmake, args: --install target --prefix d:/test_dir/x64-windows-static
-- Installing: d:/test_dir/x64-windows-static/lib/a.lib
-- Installing: d:/test_dir/x64-windows-static/bin/a.dll
-- Installing: d:/test_dir/x64-windows-static/include/a/export.h
-- Installing: d:/test_dir/x64-windows-static/include/a/lib.hpp
-- Installing: d:/test_dir/x64-windows-static/share/a/a-config.cmake
-- Installing: d:/test_dir/x64-windows-static/lib/b.lib
-- Installing: d:/test_dir/x64-windows-static/bin/b.dll
-- Installing: d:/test_dir/x64-windows-static/include/b/export.h
-- Installing: d:/test_dir/x64-windows-static/include/b/lib.hpp
-- Installing: d:/test_dir/x64-windows-static/share/b/b-config.cmake
-- Installing: d:/test_dir/x64-windows-static/lib/c.lib
-- Installing: d:/test_dir/x64-windows-static/bin/c.dll
-- Installing: d:/test_dir/x64-windows-static/include/c/export.h
-- Installing: d:/test_dir/x64-windows-static/include/c/lib.hpp
-- Installing: d:/test_dir/x64-windows-static/share/c/c-config.cmake
```

## 3.11. uninstall
### 3.11.1. help
> asc uninstall --help
```
uninstall installed executable/headers/libraries

Usage: asc.exe uninstall

Options:
  -h, --help  Print help
```
### 3.11.2. uninstall installed files
> asc uninstall
```
2024-11-01 14:46:06.0361699  INFO asc::cli::commands::uninstall: 10: uninstall name="test_workspace"
2024-11-01 14:46:06.0374127  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/lib/a.lib"
2024-11-01 14:46:06.0378967  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/bin/a.dll"
2024-11-01 14:46:06.0383549  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/include/a/export.h"
2024-11-01 14:46:06.0387121  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/include/a/lib.hpp"
2024-11-01 14:46:06.0392287  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/share/a/a-config.cmake"
2024-11-01 14:46:06.0396755  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/lib/b.lib"
2024-11-01 14:46:06.040064  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/bin/b.dll"
2024-11-01 14:46:06.0403592  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/include/b/export.h"
2024-11-01 14:46:06.040635  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/include/b/lib.hpp"
2024-11-01 14:46:06.0410633  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/share/b/b-config.cmake"
2024-11-01 14:46:06.0414438  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/lib/c.lib"
2024-11-01 14:46:06.0420534  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/bin/c.dll"
2024-11-01 14:46:06.0426158  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/include/c/export.h"
2024-11-01 14:46:06.0428833  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/include/c/lib.hpp"
2024-11-01 14:46:06.0432325  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="d:/test_dir/x64-windows-static/share/c/c-config.cmake"
2024-11-01 14:46:06.0434244  INFO asc::util::fs::dir: 83: func="std::fs::remove_dir_all" path="d:/test_dir/x64-windows-static/include/b"
2024-11-01 14:46:06.0437334  INFO asc::config::relative_paths::asc: 61: func="util::fs::remove_dirs" path="d:/test_dir/x64-windows-static/include/b"
2024-11-01 14:46:06.0440796  INFO asc::util::fs::dir: 83: func="std::fs::remove_dir_all" path="d:/test_dir/x64-windows-static/include/c"
2024-11-01 14:46:06.0443341  INFO asc::config::relative_paths::asc: 61: func="util::fs::remove_dirs" path="d:/test_dir/x64-windows-static/include/c"
2024-11-01 14:46:06.0446436  INFO asc::util::fs::dir: 83: func="std::fs::remove_dir_all" path="d:/test_dir/x64-windows-static/include/a"
2024-11-01 14:46:06.0448609  INFO asc::config::relative_paths::asc: 61: func="util::fs::remove_dirs" path="d:/test_dir/x64-windows-static/include/a"
2024-11-01 14:46:06.0451718  INFO asc::util::fs::dir: 83: func="std::fs::remove_dir_all" path="d:/test_dir/x64-windows-static/share/a"
2024-11-01 14:46:06.0454253  INFO asc::config::relative_paths::asc: 61: func="util::fs::remove_dirs" path="d:/test_dir/x64-windows-static/share/a"
2024-11-01 14:46:06.0458083  INFO asc::util::fs::dir: 83: func="std::fs::remove_dir_all" path="d:/test_dir/x64-windows-static/share/c"
2024-11-01 14:46:06.0460582  INFO asc::config::relative_paths::asc: 61: func="util::fs::remove_dirs" path="d:/test_dir/x64-windows-static/share/c"
2024-11-01 14:46:06.0464013  INFO asc::util::fs::dir: 83: func="std::fs::remove_dir_all" path="d:/test_dir/x64-windows-static/share/b"
2024-11-01 14:46:06.0466544  INFO asc::config::relative_paths::asc: 61: func="util::fs::remove_dirs" path="d:/test_dir/x64-windows-static/share/b"
2024-11-01 14:46:06.0471102  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path=".asc/installed_files.toml"
```

## 3.12. clean
### 3.12.1. help
> asc vcpkg clean --help
```
clean .asc and target directory

Usage: asc.exe clean

Options:
  -h, --help  Print help
```
### 3.12.2. remove target and generated configurations
```
2024-11-01 14:46:40.2707949  INFO asc::cli::commands::clean: 45: clean workspace name="test_workspace"
2024-11-01 14:46:40.2715091  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="CMakeLists.txt"
2024-11-01 14:46:40.2719851  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="vcpkg.json"
2024-11-01 14:46:40.2723839  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="vcpkg-configuration.json"
2024-11-01 14:46:40.2731009 ERROR asc::config::types::toml: 13: func="std::fs::read_to_string" path=".asc/installed_files.toml" error_tag="read_file_error" error_str="The system cannot find the file specified. (os error 2)"
2024-11-01 14:46:40.3164334  INFO asc::util::fs::dir: 83: func="std::fs::remove_dir_all" path="target"
2024-11-01 14:46:40.3166662  INFO asc::cli::commands::clean: 33: clean package name="a"
2024-11-01 14:46:40.3169021  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="CMakeLists.txt"
2024-11-01 14:46:40.3170947  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="version.h.in"
2024-11-01 14:46:40.3172906  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="a-config.cmake.in"
2024-11-01 14:46:40.3175241  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="config.h.cm"
2024-11-01 14:46:40.3177088  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="user.cmake"
2024-11-01 14:46:40.3178819  INFO asc::cli::commands::clean: 33: clean package name="b"
2024-11-01 14:46:40.3180823  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="CMakeLists.txt"
2024-11-01 14:46:40.3182399  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="version.h.in"
2024-11-01 14:46:40.3184088  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="b-config.cmake.in"
2024-11-01 14:46:40.3185954  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="config.h.cm"
2024-11-01 14:46:40.3187811  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="user.cmake"
2024-11-01 14:46:40.318922  INFO asc::cli::commands::clean: 33: clean package name="c"
2024-11-01 14:46:40.3191079  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="CMakeLists.txt"
2024-11-01 14:46:40.319262  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="version.h.in"
2024-11-01 14:46:40.3194202  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="c-config.cmake.in"
2024-11-01 14:46:40.3195899  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="config.h.cm"
2024-11-01 14:46:40.3197923  INFO asc::util::fs::file: 51: func="std::fs::remove_file" path="user.cmake"
```

# 4. asc.toml manual
## 4.1. workspace
```toml
[workspace]
members = [
    "a",
    "b",
    "c",
]
```
## 4.2. package
```toml
[package]
name = "ast"
version = "2024.11.20"
edition = "2024"
description = "scan necessary source files"
license = "GPL-3.0-or-later"
repository = ""
branch = "main"

[[lib]]
name = "ast"
source_dir = "src"
source_file = "lib.cpp"
shared = false
std_c = "11"
std_cxx = "17"

[dependencies]
fmt = { version = "11.0.2#1", find_packages = ["fmt"], include_directories = [], link_libraries = ["fmt::fmt"], features = [] }
llvm = { version = "18.1.6#1", find_packages = ["Clang"], include_directories = ["${CLANG_INCLUDE_DIRS}"], link_libraries = ["libclang"], features = ["clang"] }

[std_dependencies]
std_cxx = { name = "stdc++", check = "HAVE_CXX_LIBRARY OR HAVE_STD_CXX_LIBRARY"}
std_cxx_fs = { name = "stdc++fs", check = "NOT HAVE_STD_CXX_FS_LIBRARY"}

```

# 5. install
## 5.1. cargo
> cargo install asc_bin

## 5.2. pip
> python3 -m pip install asc_bin

# 6. build
## 6.1. cargo
> git clone

> cargo build --release

## 6.2. environment

### 6.2.1. git
* install git >= 2.30 and it's bin directory to PATH
> git --version
```
git version 2.45.2
```

### 6.2.2. cmake
* install cmake >= 3.20 add it's bin directory to PATH
> cmake --version
```
cmake version 3.29.6

CMake suite maintained and supported by Kitware (kitware.com/cmake).
```
