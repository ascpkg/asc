use crate::clang;

use crate::util;

pub fn path(options: &util::cli::Options) -> String {
    format!("{}.md", options.project)
}

pub fn gen(
    options: &util::cli::Options,
    source_mappings: &clang::parser::SourceMappings,
) -> String {
    let mut mermaid_flow_chart = String::from("flowchart LR;");
    for (header, sources) in &source_mappings.header_include_by_sources {
        let h = util::fs::remove_prefix(header, &options.source_dir, &options.build_dir);
        for source in sources {
            let s = util::fs::remove_prefix(source, &options.source_dir, &options.build_dir);
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
