use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use super::visitor;

use crate::util;

type StringSet = BTreeSet<String>;
type StringSetMap = BTreeMap<String, BTreeSet<String>>;
type RcRefCellStringSetMap = Rc<RefCell<StringSetMap>>;

#[derive(Debug, Clone)]
pub struct SourceMappings {
    // header - sources
    pub header_include_by_sources: StringSetMap,
    // source - headers
    pub source_include_headers: StringSetMap,
}

impl SourceMappings {
    pub fn scan(options: &util::cli::Options) -> SourceMappings {
        let mut parsed_files = BTreeSet::new();

        let (source_to_headers_from_entry_point, header_to_sources_from_entry_point) =
            Self::get_includes_from_entry_point(
                &options,
                &mut parsed_files,
                format!("{}/{}", &options.source_dir, &options.entry_point_source),
            );

        let (_source_to_headers_from_source_files, header_to_sources_from_sources_files) =
            Self::get_includes_from_source_files(&options, &mut parsed_files);

        for (header, sources) in header_to_sources_from_entry_point.borrow_mut().iter_mut() {
            if header_to_sources_from_sources_files
                .borrow()
                .contains_key(header)
            {
                for source in header_to_sources_from_sources_files
                    .borrow()
                    .get(header)
                    .unwrap()
                {
                    sources.insert(source.clone());
                }
            }
        }

        for (header, sources) in source_to_headers_from_entry_point.borrow_mut().iter_mut() {
            if header_to_sources_from_sources_files
                .borrow()
                .contains_key(header)
            {
                for source in header_to_sources_from_sources_files
                    .borrow()
                    .get(header)
                    .unwrap()
                {
                    sources.insert(source.clone());
                }
            }
        }

        return SourceMappings {
            header_include_by_sources: header_to_sources_from_entry_point.borrow().clone(),
            source_include_headers: source_to_headers_from_entry_point.borrow().clone(),
        };
    }

    fn get_includes_from_source_files(
        options: &util::cli::Options,
        parsed_files: &mut StringSet,
    ) -> (RcRefCellStringSetMap, RcRefCellStringSetMap) {
        let source_to_headers = Rc::new(RefCell::new(BTreeMap::new()));
        let header_to_sources = Rc::new(RefCell::new(BTreeMap::new()));

        for source_file in util::fs::find_source_files(&options.source_dir) {
            Self::get_include_files_in_source_dir(
                options,
                parsed_files,
                &source_file,
                source_to_headers.clone(),
                header_to_sources.clone(),
            );
        }

        return (source_to_headers, header_to_sources);
    }

    fn get_includes_from_entry_point(
        options: &util::cli::Options,
        parsed_files: &mut StringSet,
        source_file: String,
    ) -> (RcRefCellStringSetMap, RcRefCellStringSetMap) {
        let source_to_headers = Rc::new(RefCell::new(BTreeMap::new()));
        let header_to_sources = Rc::new(RefCell::new(BTreeMap::new()));

        Self::get_include_files_in_source_dir(
            options,
            parsed_files,
            &source_file,
            source_to_headers.clone(),
            header_to_sources.clone(),
        );

        return (source_to_headers, header_to_sources);
    }

    fn get_include_files_in_source_dir(
        options: &util::cli::Options,
        parsed_files: &mut StringSet,
        source_file: &String,
        source_include_headers: RcRefCellStringSetMap,
        header_include_by_sources: RcRefCellStringSetMap,
    ) {
        // skip parsed
        if parsed_files.contains(source_file) {
            return;
        }
        parsed_files.insert(source_file.clone());

        for include in visitor::get_include_files(source_file, &options) {
            // map source to headers
            source_include_headers
                .borrow_mut()
                .entry(source_file.clone())
                .or_insert_with(BTreeSet::new)
                .insert(include.clone());

            // map header to sources
            let header_include_by_sources_cloned = header_include_by_sources.clone();
            header_include_by_sources_cloned
                .borrow_mut()
                .entry(include.clone())
                .or_insert_with(BTreeSet::new)
                .insert(source_file.clone());

            // recurse
            Self::get_include_files_in_source_dir(
                options,
                parsed_files,
                &include,
                source_include_headers.clone(),
                header_include_by_sources.clone(),
            );
        }
    }
}
