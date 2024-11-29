use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    ffi::CString,
};

use crate::util;

use super::c_source_parser_ffi;

#[derive(Debug, Default, Clone)]
pub struct SourceMappings {
    pub parsed_files: BTreeSet<String>,
    pub source_symbols: BTreeMap<String, BTreeSet<String>>,
    pub source_include_headers: BTreeMap<String, BTreeSet<String>>,
    pub header_include_by_sources: BTreeMap<String, BTreeSet<String>>,
}

impl SourceMappings {
    pub fn scan_necessary_sources(
        &mut self,
        entry_point_source: &String,
        source_dir: &String,
        target_dir: &String,
    ) {
        // collect from entry point file
        let result = unsafe {
            c_source_parser_ffi::scan_source_and_symbols(
                CString::new(entry_point_source.clone()).unwrap().into_raw(),
                CString::new(source_dir.clone()).unwrap().into_raw(),
                CString::new(target_dir.clone()).unwrap().into_raw(),
                Box::into_raw(Box::new(BTreeSet::<String>::new())) as *mut std::ffi::c_void,
            )
        };
        let error_code = c_source_parser_ffi::AstCErrorCode::from(result.error_code);
        if error_code != c_source_parser_ffi::AstCErrorCode::AstCErrorNone {
            tracing::error!(
                "c_source_parser_ffi::scan_source_and_symbols error, code: {} ({})",
                std::any::type_name_of_val(&error_code),
                result.error_code,
            );
            return;
        }

        // snapshot header include by sources collected from entry point file
        let necessaries = self.collect_symbols_and_sources(result);

        // collect from other sources
        for src_path in &util::fs::find_source_files(&source_dir) {
            if src_path != entry_point_source {
                let r = unsafe {
                    c_source_parser_ffi::scan_source_and_symbols(
                        CString::new(src_path.clone()).unwrap().into_raw(),
                        CString::new(source_dir.clone()).unwrap().into_raw(),
                        CString::new(target_dir.clone()).unwrap().into_raw(),
                        Box::into_raw(Box::new(self.parsed_files.clone())) as *mut std::ffi::c_void,
                    )
                };
                let error_code = c_source_parser_ffi::AstCErrorCode::from(r.error_code);
                if error_code != c_source_parser_ffi::AstCErrorCode::AstCErrorNone {
                    tracing::error!(
                        "c_source_parser_ffi::scan_source_and_symbols error, code: {} ({})",
                        std::any::type_name_of_val(&error_code),
                        r.error_code,
                    );
                    continue;
                }

                self.collect_symbols_and_sources(r);
            }
        }

        self.append_implemented_sources(necessaries);
    }

    fn collect_symbols_and_sources(
        &mut self,
        result: c_source_parser_ffi::ClangParsedResult,
    ) -> Box<BTreeMap<String, BTreeSet<String>>> {
        // convert from raw pointer and take ownership
        let current_parsed_files =
            unsafe { Box::from_raw(result.current_parsed_files as *mut BTreeSet<String>) };
        let _last_parsed_files =
            unsafe { Box::from_raw(result.last_parsed_files as *mut BTreeSet<String>) };
        let source_symbols = unsafe {
            Box::from_raw(result.source_symbols as *mut BTreeMap<String, BTreeSet<String>>)
        };
        let source_include_headers = unsafe {
            Box::from_raw(result.source_include_headers as *mut BTreeMap<String, BTreeSet<String>>)
        };
        let header_include_by_sources = unsafe {
            Box::from_raw(
                result.header_include_by_sources as *mut BTreeMap<String, BTreeSet<String>>,
            )
        };

        // extend parsed results
        self.parsed_files.extend(current_parsed_files.into_iter());
        for (source, symbols) in source_symbols.into_iter() {
            self.source_symbols
                .entry(source)
                .or_insert_with(BTreeSet::new)
                .extend(symbols);
        }
        for (source, headers) in source_include_headers.into_iter() {
            self.source_include_headers
                .entry(source)
                .or_insert_with(BTreeSet::new)
                .extend(headers);
        }
        for (header, sources) in header_include_by_sources.clone().into_iter() {
            self.header_include_by_sources
                .entry(header)
                .or_insert_with(BTreeSet::new)
                .extend(sources);
        }

        return header_include_by_sources;
    }

    fn append_implemented_sources(
        &mut self,
        mut necessaries: Box<BTreeMap<String, BTreeSet<String>>>,
    ) {
        let mut parsed_files = HashSet::new();
        loop {
            let mut header_sources_to_insert = HashMap::new();
            let necessary_headers = necessaries
                .keys()
                .map(|k| k.clone())
                .collect::<Vec<String>>();
            for header in &necessary_headers {
                // skip parsed
                if parsed_files.contains(header) {
                    continue;
                }
                parsed_files.insert(header.clone());

                // find header symbols
                if let Some(header_symbols) = self.source_symbols.get(header) {
                    if let Some(sources) = self.header_include_by_sources.get(header) {
                        for source in sources {
                            // find source symbols
                            if let Some(source_symbols) = self.source_symbols.get(source) {
                                // find implemented source files
                                if header_symbols.intersection(source_symbols).next().is_some() {
                                    // add source file which implement symbols
                                    necessaries.get_mut(header).unwrap().insert(source.clone());

                                    // add headers which include by implemented sources
                                    if let Some(headers) = self.source_include_headers.get(source) {
                                        for h in headers {
                                            header_sources_to_insert
                                                .entry(h.clone())
                                                .or_insert_with(HashSet::new)
                                                .insert(source.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if header_sources_to_insert.is_empty() {
                break;
            }
            for (header, sources) in header_sources_to_insert {
                necessaries
                    .entry(header)
                    .or_insert_with(BTreeSet::new)
                    .extend(sources);
            }
        }

        // flatten necessary sources
        let mut flatten_necessaries = HashSet::new();
        for (header, sources) in necessaries.iter() {
            flatten_necessaries.insert(header);
            flatten_necessaries.extend(sources);
        }
        // clean unnecessary source and symbols
        self.source_symbols
            .retain(|source, _| flatten_necessaries.contains(source));

        self.parsed_files.clear();
        self.source_include_headers.clear();

        // store necessary sources
        self.header_include_by_sources = *necessaries;
    }
}
