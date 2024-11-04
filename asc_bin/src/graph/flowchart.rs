use crate::clang;
use crate::cli;
use crate::config::relative_paths;
use crate::util;

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
        relative_paths::FLOW_CHART_MD_FILE_NAME,
        format!("```mermaid\n{}\n```", mermaid_flow_chart).as_bytes(),
    )
    .unwrap();

    return mermaid_flow_chart;
}
