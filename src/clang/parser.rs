use super::visitor;

use crate::util;

type StringSet = std::collections::BTreeSet<String>;
type StringMapSet = std::collections::BTreeMap<String, std::collections::BTreeSet<String>>;
type RcRefCellStringMapSet = std::rc::Rc<std::cell::RefCell<StringMapSet>>;

#[derive(Debug, Clone)]
pub struct SourceMappings {
    // header - sources
    pub header_include_by_sources: StringMapSet,
    // source - headers
    pub source_include_headers: StringMapSet,
}

impl SourceMappings {
    pub fn scan(options: &util::cli::CommandLines) -> SourceMappings {
        let mut parsed_files = std::collections::BTreeSet::new();

        let (source_to_headers_from_entry_point, header_to_sources_from_entry_point) =
            Self::get_includes_from_entry_point(
                &options,
                &mut parsed_files,
                format!("{}/{}", &options.source_dir, &options.entry_point_source),
            );

        let (_source_to_headers_from_source_files, header_to_sources_from_sources_files) =
            Self::get_includes_from_source_files(
                &options,
                &mut parsed_files,
            );

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
        options: &util::cli::CommandLines,
        parsed_files: &mut StringSet,
    ) -> (RcRefCellStringMapSet, RcRefCellStringMapSet) {
        let source_to_headers =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));
        let header_to_sources =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));

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
        options: &util::cli::CommandLines,
        parsed_files: &mut StringSet,
        source_file: String,
    ) -> (RcRefCellStringMapSet, RcRefCellStringMapSet) {
        let source_to_headers =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));
        let header_to_sources =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));

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
        options: &util::cli::CommandLines,
        parsed_files: &mut StringSet,
        source_file: &String,
        source_include_headers: RcRefCellStringMapSet,
        header_include_by_sources: RcRefCellStringMapSet,
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
                .or_insert_with(std::collections::BTreeSet::new)
                .insert(include.clone());

            // map header to sources
            let header_include_by_sources_cloned = header_include_by_sources.clone();
            header_include_by_sources_cloned
                .borrow_mut()
                .entry(include.clone())
                .or_insert_with(std::collections::BTreeSet::new)
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
