use std::collections::{BTreeMap, BTreeSet};

use super::visitor;

use crate::cli;
use crate::util;

#[derive(Debug, Default, Clone)]
pub struct SourceMappings {
    // prefix
    pub source_dir: String,
    pub target_dir: String,

    // include dirs
    pub include_dirs: Vec<String>,

    // header - sources
    pub header_include_by_sources: BTreeMap<String, BTreeSet<String>>,
    // source - headers
    pub source_include_headers: BTreeMap<String, BTreeSet<String>>,
    // file - symbols
    pub source_symbols: BTreeMap<String, BTreeSet<String>>,

    // parsed
    parsed_files: BTreeSet<String>,
}

impl SourceMappings {
    pub fn scan(options: &cli::commands::scan::ScanOptions) -> Self {
        // init
        let mut mappings = SourceMappings::default();
        mappings.source_dir = options.source_dir.clone();
        mappings.target_dir = options.target_dir.clone();
        mappings.include_dirs = options.include_dirs.clone();

        // parse entry point source file
        mappings.get_symbols_and_inclusions(&options.entry_point_source);

        // parse all source files
        for source_file in util::fs::find_source_files(&mappings.source_dir) {
            mappings.get_symbols_and_inclusions(&source_file);
        }

        // clear temp data
        mappings.parsed_files.clear();

        // remove source files that do not implement any symbols declared in the header file
        for (header, sources) in mappings.header_include_by_sources.iter_mut() {
            if let Some(header_symbols) = mappings.source_symbols.get(header) {
                sources.retain(|source| {
                    if source.ends_with(&options.entry_point_source) {
                        return true;
                    }
                    if let Some(source_symbols) = mappings.source_symbols.get(source) {
                        return (header_symbols.is_empty() && source_symbols.is_empty())
                            || !header_symbols
                                .intersection(source_symbols)
                                .cloned()
                                .collect::<Vec<String>>()
                                .is_empty();
                    }
                    return false;
                });
            }
        }

        // remove isolated island headers and sources

        return mappings;
    }

    fn get_symbols_and_inclusions(&mut self, source_file: &String) {
        // skip parsed file
        if self.parsed_files.contains(source_file) {
            return;
        }
        self.parsed_files.insert(source_file.clone());

        // visit
        let (include_files, source_symbols) =
            visitor::get_symbols_and_inclusions(source_file, &self);

        // collect symbols
        for (source, symbols) in source_symbols {
            if !self.source_symbols.contains_key(&source) {
                self.source_symbols.insert(source, symbols);
            } else {
                let values = self.source_symbols.get_mut(&source).unwrap();
                for symbol in symbols.iter() {
                    values.insert(symbol.clone());
                }
            }
        }

        // collect sources
        for include in include_files {
            // map source to headers
            self.source_include_headers
                .entry(source_file.clone())
                .or_insert_with(BTreeSet::new)
                .insert(include.clone());

            // map header to sources
            self.header_include_by_sources
                .entry(include.clone())
                .or_insert_with(BTreeSet::new)
                .insert(source_file.clone());

            // recurse
            self.get_symbols_and_inclusions(&include);
        }
    }
}
