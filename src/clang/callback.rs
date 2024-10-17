use bindgen;

use tracing;

#[derive(Debug, Clone)]
pub struct MyParseCallbacks {
    source_dir: String,
    headers: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    functions: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
}

impl MyParseCallbacks {
    pub fn new(
        source_dir: String,
        headers_container: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
        functions_container: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ) -> Self {
        Self {
            source_dir: source_dir,
            headers: headers_container,
            functions: functions_container,
        }
    }
}

impl bindgen::callbacks::ParseCallbacks for MyParseCallbacks {
    fn header_file(&self, filename: &str) {
        tracing::error!("{}", filename);
    }

    fn include_file(&self, filename: &str) {
        if filename.starts_with(&self.source_dir) {
            let path = filename.replace(r"\", "/");
            tracing::error!("    {}", path);

            self.headers.borrow_mut().push(path);
        }
    }

    fn generated_name_override(
        &self,
        item_info: bindgen::callbacks::ItemInfo<'_>,
    ) -> Option<String> {
        match item_info.kind {
            bindgen::callbacks::ItemKind::Function => {
                // tracing::error!("        function: {}", item_info.name);

                self.functions.borrow_mut().push(item_info.name.to_string());
            }
            _ => {}
        };

        None
    }
}
