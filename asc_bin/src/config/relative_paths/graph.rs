use crate::util;

pub static FLOW_CHART_MD_FILE_NAME: &str = "flowchart.md";

pub fn clean_graph_files() -> bool {
    let mut has_error = false;

    for path in [FLOW_CHART_MD_FILE_NAME] {
        if util::fs::is_file_exists(path) {
            has_error &= util::fs::remove_file(path);
        }
    }

    return has_error;
}
