## C/C++ package manager and auto source builder (inspired Rust Cargo)

```
 █████  ███████  ██████     ██████  ██   ██  ██████  
██   ██ ██      ██          ██   ██ ██  ██  ██       
███████ ███████ ██          ██████  █████   ██   ███ 
██   ██      ██ ██          ██      ██  ██  ██    ██ 
██   ██ ███████  ██████     ██      ██   ██  ██████  
```                                                     

### **crates dependency graph**
```mermaid
graph LR;
    asc--depend-->config_file_derives
    asc--depend-->config_file_types
    config_file_types--depend-->config_file_macros
    asc--depend-->c_source_parser_ffi
    c_source_parser_ffi--depend-->rs_container_ffi
```


---


### asc
[English](asc_bin/README.md)
&nbsp;&nbsp;&nbsp;&nbsp;
[简体中文](asc_bin/README.zh-CN.md)

[![Crates.io](https://img.shields.io/crates/d/asc_bin.svg)](https://crates.io/crates/asc_bin)
[![pypi.org](https://img.shields.io/pypi/dm/asc-bin)](https://pypi.org/project/asc-bin)
[![github.com](https://img.shields.io/github/downloads/ascpkg/asc/total.svg)](https://github.com/ascpkg/asc/releases)
[![Crates.io](https://img.shields.io/crates/v/asc_bin.svg)](https://crates.io/crates/asc_bin)


---


### config_file_derives

[![Docs](https://docs.rs/config_file_derives/badge.svg)](https://docs.rs/config_file_derives)
[![Crates.io](https://img.shields.io/crates/d/config_file_derives.svg)](https://crates.io/crates/config_file_derives)
[![Crates.io](https://img.shields.io/crates/v/config_file_derives.svg)](https://crates.io/crates/config_file_derives)


---


### config_file_macros

[![Docs](https://docs.rs/config_file_macros/badge.svg)](https://docs.rs/config_file_macros)
[![Crates.io](https://img.shields.io/crates/d/config_file_macros.svg)](https://crates.io/crates/config_file_macros)
[![Crates.io](https://img.shields.io/crates/v/config_file_macros.svg)](https://crates.io/crates/config_file_macros)


---


### config_file_types

[![Docs](https://docs.rs/config_file_types/badge.svg)](https://docs.rs/config_file_types)
[![Crates.io](https://img.shields.io/crates/d/config_file_types.svg)](https://crates.io/crates/config_file_types)
[![Crates.io](https://img.shields.io/crates/v/config_file_types.svg)](https://crates.io/crates/config_file_types)


---


### c_source_parser_ffi

[![Docs](https://docs.rs/c_source_parser_ffi/badge.svg)](https://docs.rs/c_source_parser_ffi)
[![Crates.io](https://img.shields.io/crates/d/c_source_parser_ffi.svg)](https://crates.io/crates/c_source_parser_ffi)
[![Crates.io](https://img.shields.io/crates/v/c_source_parser_ffi.svg)](https://crates.io/crates/c_source_parser_ffi)


---


### rs_container_ffi

[![Docs](https://docs.rs/rs_container_ffi/badge.svg)](https://docs.rs/rs_container_ffi)
[![Crates.io](https://img.shields.io/crates/d/rs_container_ffi.svg)](https://crates.io/crates/rs_container_ffi)
[![Crates.io](https://img.shields.io/crates/v/rs_container_ffi.svg)](https://crates.io/crates/rs_container_ffi)
