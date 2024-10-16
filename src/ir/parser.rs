use super::callback;

use crate::util;

pub fn parse(options: util::cli::CommandLines) -> (
    std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>,
    std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>,
    std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>,
    std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>
) {
    // record parsed files
    let mut parsed = std::collections::HashSet::<String>::new();

    // header - sources
    let mut header_sources =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    // header - functions
    let mut header_functions =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    // source - headers
    let mut source_headers =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    // source - functions
    let mut source_functions =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();

    for source_dir in &options.source_dirs {
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
            for include_dir in &options.include_dirs {
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
            if !source_headers.contains_key(&source_file) {
                source_headers.insert(source_file.clone(), std::collections::BTreeSet::new());
            }
            let headers_set = source_headers.get_mut(&source_file).unwrap();
            for header in headers.borrow().iter() {
                headers_set.insert(header.clone());

                // map header to sources
                if !header_sources.contains_key(header) {
                    header_sources.insert(header.clone(), std::collections::BTreeSet::new());
                }
                let sources_set = header_sources.get_mut(header).unwrap();
                sources_set.insert(source_file.clone());
            }

            // map source to functions
            if !source_functions.contains_key(&source_file) {
                source_functions.insert(source_file.clone(), std::collections::BTreeSet::new());
            }
            let functions_set = source_functions.get_mut(&source_file).unwrap();
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
                for include_dir in &options.include_dirs {
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

                // map header to functions
                if !header_functions.contains_key(header) {
                    header_functions.insert(header.clone(), std::collections::BTreeSet::new());
                }
                let functions_set = header_functions.get_mut(header).unwrap();
                for function in inner_functions.borrow().iter() {
                    functions_set.insert(function.clone());
                }
            }
        }
    }

   (source_headers, source_functions, header_sources, header_functions)
}
