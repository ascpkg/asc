use crate::ir;

pub fn remove_unused_dependency(source_mappings: &mut ir::parser::SourceMappings) {
 
}

pub fn build_dependency_flowchat(source_mappings: &ir::parser::SourceMappings) -> String {
    let mut mermaid_code = String::from("\n```mermaid\nflowchart LR;\n");
    for (header, sources) in &source_mappings.header_inclued_by_sources {
        for source in sources {
            mermaid_code.push_str(&format!("    {} -.-> {};\n", header, source));
        }
    }
    mermaid_code.push_str("```\n");

    mermaid_code.push_str("\n```mermaid\nflowchart LR;\n");
    for (source, headers) in &source_mappings.source_include_headers {
        for header in headers {
            mermaid_code.push_str(&format!("    {} ---> {};\n", source, header));
        }
    }

    for (header, headers) in &source_mappings.header_include_headers {
        for h in headers {
            mermaid_code.push_str(&format!("    {} ---> {};\n", header, h));
        }
    }
    mermaid_code.push_str("```\n");

    mermaid_code
}
