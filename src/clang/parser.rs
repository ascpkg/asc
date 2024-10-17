use super::callback;

use crate::util;

#[derive(Debug, Clone)]
pub struct SourceMappings {
    // header - sources
    pub header_inclued_by_sources:
        std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    // source - headers
    pub source_include_headers:
        std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    // header - headers
    pub header_include_headers:
        std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    // header - functions
    pub functions_in_header: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    // source - functions
    pub functions_in_source: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
}

impl SourceMappings {
    pub fn parse(options: &util::cli::CommandLines) -> SourceMappings {
        let mut valid_source_mappings = Self::parse_from_main_entry(
            &options.source_dir,
            format!("{}/{}", &options.source_dir, &options.source_main_entry),
            &options.include_dirs,
        );

        let tmp_source_mappings = Self::parse_sources(&options.source_dir, &options.include_dirs);
        for (header, sources) in &mut valid_source_mappings.header_inclued_by_sources {
            if tmp_source_mappings
                .header_inclued_by_sources
                .contains_key(header)
            {
                for source in tmp_source_mappings
                    .header_inclued_by_sources
                    .get(header)
                    .unwrap()
                {
                    sources.insert(source.clone());
                }
            }
        }

        return valid_source_mappings;
    }

    pub fn parse_sources(source_dir: &String, include_dirs: &Vec<String>) -> SourceMappings {
        let mut mappings = SourceMappings {
            header_inclued_by_sources: std::collections::BTreeMap::<
                String,
                std::collections::BTreeSet<String>,
            >::new(),
            source_include_headers: std::collections::BTreeMap::<
                String,
                std::collections::BTreeSet<String>,
            >::new(),
            header_include_headers: std::collections::BTreeMap::<
                String,
                std::collections::BTreeSet<String>,
            >::new(),
            functions_in_header: std::collections::BTreeMap::<
                String,
                std::collections::BTreeSet<String>,
            >::new(),
            functions_in_source: std::collections::BTreeMap::<
                String,
                std::collections::BTreeSet<String>,
            >::new(),
        };

        // record parsed files
        let mut parsed = std::collections::HashSet::<String>::new();

        for source_file in util::fs::find_source_files(source_dir) {
            // skip parsed
            if parsed.contains(&source_file) {
                continue;
            }

            // parse source file
            let headers = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
            let functions = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
            let callbacks = callback::MyParseCallbacks::new(
                source_dir.clone(),
                headers.clone(),
                functions.clone(),
            );

            let mut builder = bindgen::Builder::default().header(source_file.clone());
            for include_dir in include_dirs {
                if !include_dir.is_empty() {
                    builder = builder.clang_arg("-I").clang_arg(include_dir);
                }
            }

            builder
                .parse_callbacks(Box::new(callbacks))
                .ignore_functions()
                .ignore_methods()
                .generate()
                .expect("Unable to generate source dependency tree");

            parsed.insert(source_file.clone());

            // map source to headers
            if !mappings.source_include_headers.contains_key(&source_file) {
                mappings
                    .source_include_headers
                    .insert(source_file.clone(), std::collections::BTreeSet::new());
            }
            let headers_set = mappings
                .source_include_headers
                .get_mut(&source_file)
                .unwrap();
            for header in headers.borrow().iter() {
                headers_set.insert(header.clone());

                // map header to sources
                if !mappings.header_inclued_by_sources.contains_key(header) {
                    mappings
                        .header_inclued_by_sources
                        .insert(header.clone(), std::collections::BTreeSet::new());
                }
                let sources_set = mappings.header_inclued_by_sources.get_mut(header).unwrap();
                sources_set.insert(source_file.clone());
            }
        }

        return mappings;
    }

    fn parse_from_main_entry(
        source_dir: &String,
        source_file: String,
        include_dirs: &Vec<String>,
    ) -> SourceMappings {
        let mut mappings = SourceMappings {
            header_inclued_by_sources: std::collections::BTreeMap::<
                String,
                std::collections::BTreeSet<String>,
            >::new(),
            source_include_headers: std::collections::BTreeMap::<
                String,
                std::collections::BTreeSet<String>,
            >::new(),
            header_include_headers: std::collections::BTreeMap::<
                String,
                std::collections::BTreeSet<String>,
            >::new(),
            functions_in_header: std::collections::BTreeMap::<
                String,
                std::collections::BTreeSet<String>,
            >::new(),
            functions_in_source: std::collections::BTreeMap::<
                String,
                std::collections::BTreeSet<String>,
            >::new(),
        };

        // record parsed files
        let mut parsed = std::collections::HashSet::<String>::new();

        // parse source file
        let headers = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let functions = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let callbacks =
            callback::MyParseCallbacks::new(source_dir.clone(), headers.clone(), functions.clone());

        let mut builder = bindgen::Builder::default().header(source_file.clone());
        for include_dir in include_dirs {
            if !include_dir.is_empty() {
                builder = builder.clang_arg("-I").clang_arg(include_dir);
            }
        }

        builder
            .parse_callbacks(Box::new(callbacks))
            .ignore_functions()
            .ignore_methods()
            .generate()
            .expect("Unable to generate source dependency tree");

        parsed.insert(source_file.clone());

        // map source to headers
        if !mappings.source_include_headers.contains_key(&source_file) {
            mappings
                .source_include_headers
                .insert(source_file.clone(), std::collections::BTreeSet::new());
        }
        let headers_set = mappings
            .source_include_headers
            .get_mut(&source_file)
            .unwrap();
        for header in headers.borrow().iter() {
            headers_set.insert(header.clone());

            // map header to sources
            if !mappings.header_inclued_by_sources.contains_key(header) {
                mappings
                    .header_inclued_by_sources
                    .insert(header.clone(), std::collections::BTreeSet::new());
            }
            let sources_set = mappings.header_inclued_by_sources.get_mut(header).unwrap();
            sources_set.insert(source_file.clone());
        }

        // map source to functions
        if !mappings.functions_in_source.contains_key(&source_file) {
            mappings
                .functions_in_source
                .insert(source_file.clone(), std::collections::BTreeSet::new());
        }
        let functions_set = mappings.functions_in_source.get_mut(&source_file).unwrap();
        for function in functions.borrow().iter() {
            functions_set.insert(function.clone());
        }

        // parse header
        for header in headers.borrow().iter() {
            // skip parsed
            if parsed.contains(&header.clone()) {
                continue;
            }

            let inner_headers = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
            let inner_functions = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
            let inner_callbacks = callback::MyParseCallbacks::new(
                source_dir.clone(),
                inner_headers.clone(),
                inner_functions.clone(),
            );

            let mut inner_builder = bindgen::Builder::default().header(header.clone());
            for include_dir in include_dirs {
                if !include_dir.is_empty() {
                    inner_builder = inner_builder.clang_arg("-I").clang_arg(include_dir);
                }
            }

            inner_builder
                .parse_callbacks(Box::new(inner_callbacks))
                .ignore_functions()
                .ignore_methods()
                .generate()
                .expect("Unable to generate source dependency tree");

            parsed.insert(header.clone());

            // map header to headers
            if !mappings.header_include_headers.contains_key(header) {
                mappings
                    .header_include_headers
                    .insert(header.clone(), std::collections::BTreeSet::new());
            }
            let headers_set = mappings.header_include_headers.get_mut(header).unwrap();
            for header in inner_headers.borrow().iter() {
                headers_set.insert(header.clone());
            }

            // map header to functions
            if !mappings.functions_in_header.contains_key(header) {
                mappings
                    .functions_in_header
                    .insert(header.clone(), std::collections::BTreeSet::new());
            }
            let functions_set = mappings.functions_in_header.get_mut(header).unwrap();
            for function in inner_functions.borrow().iter() {
                functions_set.insert(function.clone());
            }
        }

        return mappings;
    }
}
