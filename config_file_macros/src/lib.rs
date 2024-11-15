#[macro_export]
macro_rules! generate_wrapper_methods {
    ($wrapper:ident, $format:ident, $deserialize_method:ident, $serialize_method:ident, $serialize_pretty_method:ident, $deserialize_func_str:expr, $serialize_func_str:expr, $deserialize_error:expr, $serialize_error:expr) => {
        impl<T> $wrapper<T>
        where
            T: DeserializeOwned + Serialize,
        {
            // read from file
            pub fn load(path: &str, ignore_error: bool) -> Option<T> {
                match std::fs::read_to_string(path) {
                    Ok(text) => Self::loads(&text, ignore_error),
                    Err(e) => {
                        if !ignore_error {
                            tracing::error!(
                                func = "std::fs::read_to_string",
                                path = path,
                                error_tag = "ReadFileError",
                                error_str = e.to_string(),
                            );
                        }
                        None
                    }
                }
            }

            // read from str
            pub fn loads(text: &str, ignore_error: bool) -> Option<T> {
                match $format::$deserialize_method(text) {
                    Ok(c) => Some(c),
                    Err(e) => {
                        if !ignore_error {
                            tracing::error!(
                                func = $deserialize_func_str,
                                error_tag = $deserialize_error,
                                error_str = e.to_string(),
                                message = text,
                            );
                        }
                        None
                    }
                }
            }

            // write to file
            pub fn dump(&self, pretty: bool, ignore_error: bool) -> bool {
                Self::dump_data(&self.inner, &self.path, pretty, ignore_error)
            }

            // write to file
            pub fn dump_data(data: &T, path: &str, pretty: bool, ignore_error: bool) -> bool {
                let text = Self::dumps_data(data, pretty, ignore_error);
                if text.is_empty() {
                    return false;
                }

                match std::fs::write(path, text.as_bytes()) {
                    Ok(_) => true,
                    Err(e) => {
                        if !ignore_error {
                            tracing::error!(
                                func = "std::fs::write",
                                path = path,
                                error_tag = "WriteFileError",
                                error_str = e.to_string(),
                                message = text,
                            );
                        }
                        false
                    }
                }
            }

            // write to str
            pub fn dumps(&self, pretty: bool, ignore_error: bool) -> String {
                Self::dumps_data(&self.inner, pretty, ignore_error)
            }

            // write to str
            pub fn dumps_data(data: &T, pretty: bool, ignore_error: bool) -> String {
                let f = if !pretty {
                    $format::$serialize_method
                } else {
                    $format::$serialize_pretty_method
                };
                match f(data) {
                    Ok(text) => {
                        if !text.contains("[dependencies.") {
                            text
                        } else {
                            let mut dependencies = std::collections::BTreeMap::new();
                            let mut current_dep = String::new();
                            let mut output = String::new();
                            let mut feed = true;
                            for line in text.lines() {
                                let line = line.trim();
                                if line.starts_with("[dependencies.") {
                                    if feed {
                                        output.push_str("[dependencies]\n");
                                    }
                                    feed = false;
                                    current_dep =
                                        line["[dependencies.".len()..line.len() - 1].to_string();
                                } else if !feed && line.starts_with("version") {
                                    let version =
                                        line.split('=').nth(1).unwrap().trim().to_string();
                                    dependencies.insert(
                                        current_dep.clone(),
                                        format!(r#"version = {}"#, version),
                                    );
                                } else if !feed
                                    && (line.starts_with("find_packages")
                                        || line.starts_with("include_directories")
                                        || line.starts_with("link_libraries")
                                        || line.starts_with("features"))
                                {
                                    let key = line.split('=').next().unwrap().trim();
                                    let value = line.split('=').nth(1).unwrap().trim();
                                    dependencies
                                        .entry(current_dep.clone())
                                        .or_insert(String::new())
                                        .push_str(&format!(r#", {} = {}"#, key, value));
                                } else {
                                    if feed {
                                        output.push_str(&line);
                                        output.push_str("\n");
                                    }
                                }
                            }
                            for (dep, props) in dependencies {
                                output.push_str(&format!("{} = {{ {} }}\n", dep, props));
                            }
                            output.push_str("\n");
                            output
                        }
                    }
                    Err(e) => {
                        if !ignore_error {
                            tracing::error!(
                                func = $serialize_func_str,
                                error_tag = $serialize_error,
                                error_str = e.to_string(),
                            );
                        }
                        String::new()
                    }
                }
            }
        }
    };
}
