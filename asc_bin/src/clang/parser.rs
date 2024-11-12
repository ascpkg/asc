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

    // file - symbols
    pub source_symbols: BTreeMap<String, BTreeSet<String>>,

    // parsed
    pub parsed_files: BTreeSet<String>,
}

impl SourceMappings {
    pub fn scan(options: &cli::commands::scan::ScanOptions) -> Self {
        // init
        let mut self_ = SourceMappings {
            source_dir: options.source_dir.clone(),
            target_dir: options.target_dir.clone(),
            include_dirs: options.include_dirs.clone(),
            ..Default::default()
        };

        // parse entry point source file
        self_.collect_symbols_and_inclusions(&options.entry_point_source);
        let header_include_by_sources = self_.header_include_by_sources.clone();

        // parse other source files
        let other_source_files = util::fs::find_source_files(&self_.source_dir)
            .into_iter()
            .filter(|path| path != &options.entry_point_source)
            .collect();
        for source_file in &other_source_files {
            self_.collect_symbols_and_inclusions(source_file);
        }

        // clear temp data
        self_.parsed_files.clear();

        // add source files which implement symbols declared in header files
        self_.add_implementation_sources(other_source_files, header_include_by_sources);

        return self_;
    }

    fn add_implementation_sources(
        &mut self,
        other_sources: Vec<String>,
        header_include_by_sources: BTreeMap<String, BTreeSet<String>>,
    ) {
        self.header_include_by_sources = header_include_by_sources.clone();
        for source in &other_sources {
            if let Some(source_symbols) = self.source_symbols.get(source) {
                for (header, sources) in self.header_include_by_sources.iter_mut() {
                    if let Some(header_symbols) = self.source_symbols.get(header) {
                        if header_symbols.intersection(source_symbols).next().is_some() {
                            sources.insert(source.clone());
                        }
                    }
                }
            }
        }
    }

    fn collect_symbols_and_inclusions(&mut self, source_file: &String) {
        // visit
        let result = visitor::collect_symbols_and_inclusions(source_file, &self);

        // collect symbols
        for (source, symbols) in result.source_symbols {
            self.source_symbols
                .entry(source)
                .or_default()
                .extend(symbols);
        }

        // collect sources
        for (include, source) in &result.include_files {
            self.header_include_by_sources
                .entry(include.clone())
                .or_default()
                .extend(source.clone());
        }

        // collect parsed files
        self.parsed_files.extend(result.parsed_files);
    }
}
