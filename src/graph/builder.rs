

pub fn remove_unused_dependency(
    header_sources: &mut std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    source_headers: &mut std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
) {

}

pub fn build_dependency_flowchat(
    header_sources: &std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    source_headers: &std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
) -> String {
    let mut mermaid_code = String::from("```mermaid\nflowchart LR;\n");

    for (header, sources) in header_sources {
        for source in sources {
            mermaid_code.push_str(&format!("    {} -.-> {};\n", header, source));
        }
    }

    for (source, headers) in source_headers {
        for header in headers {
            mermaid_code.push_str(&format!("    {} -- include --> {};\n", source, header));
        }
    }

    mermaid_code.push_str("```");

    mermaid_code
}
