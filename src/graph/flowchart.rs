use crate::clang;
use crate::cli;
use crate::util;

pub fn path(_options: &cli::commands::scan::ScanOptions) -> String {
    format!("flowchart.md")
}

pub fn gen(
    options: &cli::commands::scan::ScanOptions,
    source_mappings: &clang::parser::SourceMappings,
) -> String {
    let mut mermaid_flow_chart = String::from("flowchart LR;");
    for (header, sources) in &source_mappings.header_include_by_sources {
        let h = util::fs::remove_prefix(header, &options.source_dir, &options.target_dir);
        for source in sources {
            let s = util::fs::remove_prefix(source, &options.source_dir, &options.target_dir);
            mermaid_flow_chart.push_str(&format!("\n    {} ---> {};", s, h));
        }
    }

    std::fs::write(
        path(&options),
        format!("```mermaid\n{}\n```", mermaid_flow_chart).as_bytes(),
    )
    .unwrap();

    return mermaid_flow_chart;
}
