// pub mod parser;
// pub mod util;
// pub mod visitor;

pub mod ast;

pub mod parser {
    use std::{
        collections::{BTreeMap, BTreeSet},
        ffi::{c_char, c_int, CStr, CString},
    };

    use crate::cli;

    use super::ast;

    #[derive(Debug, Default, Clone)]
    pub struct SourceMappings {
        // header - sources
        pub header_include_by_sources: BTreeMap<String, BTreeSet<String>>,
    }

    impl SourceMappings {
        pub fn scan(options: &cli::commands::scan::ScanOptions) -> Self {
            let mut mappings = SourceMappings::default();

            let mut result_buf = Vec::<c_char>::new();
            result_buf.resize(64 * 1024, 0);
            unsafe {
                let result_len = ast::scan_necessary_sources(
                    CString::new(options.entry_point_source.as_bytes())
                        .unwrap()
                        .into_raw(),
                    CString::new(options.source_dir.as_bytes())
                        .unwrap()
                        .into_raw(),
                    CString::new(options.target_dir.as_bytes())
                        .unwrap()
                        .into_raw(),
                    result_buf.as_mut_ptr(),
                    result_buf.len() as c_int,
                    0,
                );
                if result_len <= 0 {
                    tracing::error!("ast::scan_necessary_sources error");
                    return mappings;
                }
            };

            let result_text = unsafe { CStr::from_ptr(result_buf.as_ptr()).to_str().unwrap() };
            for line in result_text.split("\n") {
                if line.is_empty() || line == "\n" {
                    continue;
                }

                if let Some((header, source)) = line.split_once("\t\t") {
                    mappings
                        .header_include_by_sources
                        .entry(header.trim().to_string())
                        .or_insert_with(BTreeSet::new)
                        .insert(source.trim().to_string());
                }
            }

            return mappings;
        }
    }
}
