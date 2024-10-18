use super::visitor;

use crate::util;

type StringSet = std::collections::BTreeSet<String>;
type StringMapSet = std::collections::BTreeMap<String, std::collections::BTreeSet<String>>;
type RcRefCellStringMapSet = std::rc::Rc<std::cell::RefCell<StringMapSet>>;

#[derive(Debug, Clone)]
pub struct SourceMappings {
    // header - sources
    pub header_inclued_by_sources: StringMapSet,
    // source - headers
    pub source_include_headers: StringMapSet,
}

impl SourceMappings {
    pub fn scan(options: &util::cli::CommandLines) -> SourceMappings {
        let mut parsed_files = std::collections::BTreeSet::new();

        let (source_to_headers_from_entry_point, header_to_sources_from_entry_point) =
            Self::get_includes_from_entry_point(
                &mut parsed_files,
                &options.source_dir,
                format!("{}/{}", &options.source_dir, &options.entry_point_source),
                &options.include_dirs,
            );

        let (_source_to_headers_from_source_files, header_to_sources_from_sources_files) =
            Self::get_includes_from_source_files(
                &mut parsed_files,
                &options.source_dir,
                &options.include_dirs,
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
            header_inclued_by_sources: header_to_sources_from_entry_point.borrow().clone(),
            source_include_headers: source_to_headers_from_entry_point.borrow().clone(),
        };
    }

    fn get_includes_from_source_files(
        parsed_files: &mut StringSet,
        source_dir: &String,
        include_dirs: &Vec<String>,
    ) -> (RcRefCellStringMapSet, RcRefCellStringMapSet) {
        let source_to_headers =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));
        let header_to_sources =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));

        for source_file in util::fs::find_source_files(source_dir) {
            Self::get_include_files_in_source_dir(
                parsed_files,
                &source_file,
                source_dir,
                source_to_headers.clone(),
                header_to_sources.clone(),
                include_dirs,
            );
        }

        return (source_to_headers, header_to_sources);
    }

    fn get_includes_from_entry_point(
        parsed_files: &mut StringSet,
        source_dir: &String,
        source_file: String,
        include_dirs: &Vec<String>,
    ) -> (RcRefCellStringMapSet, RcRefCellStringMapSet) {
        let source_to_headers =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));
        let header_to_sources =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));

        Self::get_include_files_in_source_dir(
            parsed_files,
            &source_file,
            source_dir,
            source_to_headers.clone(),
            header_to_sources.clone(),
            include_dirs,
        );

        return (source_to_headers, header_to_sources);
    }

    fn get_include_files_in_source_dir(
        parsed_files: &mut StringSet,
        source_file: &String,
        source_dir: &String,
        source_include_headers: RcRefCellStringMapSet,
        header_inclued_by_sources: RcRefCellStringMapSet,
        include_dirs: &Vec<String>,
    ) {
        // skip parsed
        if parsed_files.contains(source_file) {
            return;
        }
        parsed_files.insert(source_file.clone());

        for include in visitor::get_include_files(source_file, source_dir) {
            // skip third-party
            if !include.starts_with(source_dir) {
                continue;
            }

            // map source to headers
            source_include_headers
                .borrow_mut()
                .entry(source_file.clone())
                .or_insert_with(std::collections::BTreeSet::new)
                .insert(include.clone());

            // map header to sources
            let header_inclued_by_sources_cloned = header_inclued_by_sources.clone();
            header_inclued_by_sources_cloned
                .borrow_mut()
                .entry(include.clone())
                .or_insert_with(std::collections::BTreeSet::new)
                .insert(source_file.clone());

            // recurse
            Self::get_include_files_in_source_dir(
                parsed_files,
                &include,
                source_dir,
                source_include_headers.clone(),
                header_inclued_by_sources.clone(),
                include_dirs,
            );
        }
    }
}
