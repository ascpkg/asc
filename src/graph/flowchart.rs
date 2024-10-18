use crate::clang;

use crate::util;

pub fn path(options: &util::cli::CommandLines) -> String {
    format!("{}.md", options.project)
}

pub fn gen(
    options: &util::cli::CommandLines,
    source_mappings: &clang::parser::SourceMappings,
) -> String {
    let prefix_length = options.source_dir.len() + 1;

    let mut mermaid_flow_chart = String::from("flowchart LR;");
    for (header, sources) in &source_mappings.header_include_by_sources {
        for source in sources {
            mermaid_flow_chart.push_str(&format!(
                "\n    {} ---> {};",
                source.clone().split_off(prefix_length),
                header.clone().split_off(prefix_length)
            ));
        }
    }

    std::fs::write(
        path(&options),
        format!("```mermaid\n{}\n```", mermaid_flow_chart).as_bytes(),
    )
    .unwrap();

    return mermaid_flow_chart;
}
