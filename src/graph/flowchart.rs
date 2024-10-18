use crate::clang;

pub fn gen(
    source_dir: &String,
    source_mappings: &clang::parser::SourceMappings,
) -> String {
    let prefix_length = source_dir.len() + 1;

    let mut mermaid_code = String::from("flowchart LR;");
    for (header, sources) in &source_mappings.header_inclued_by_sources {
        for source in sources {
            mermaid_code.push_str(&format!(
                "\n    {} ---> {};",
                source.clone().split_off(prefix_length),
                header.clone().split_off(prefix_length)
            ));
        }
    }

    return mermaid_code;
}
