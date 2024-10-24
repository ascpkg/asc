use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::errors::ErrorTag;

#[derive(Deserialize, Serialize, Debug)]
pub struct Wrapper<T> {
    inner: T,
    path: String,
}

impl<T> Wrapper<T>
where
    T: DeserializeOwned + Serialize,
{
    pub fn new(data: T, path: &str) -> Self {
        Self {
            inner: data,
            path: path.to_string(),
        }
    }

    pub fn load(path: &str, ignore: bool) -> Option<T> {
        match std::fs::read_to_string(path) {
            Ok(text) => Self::loads(&text, ignore),
            Err(e) => {
                if !ignore {
                    tracing::error!(
                        func = "std::fs::read_to_string",
                        path = path,
                        error_tag = ErrorTag::ReadFileError.as_ref(),
                        error_str = e.to_string(),
                    );
                }
                None
            }
        }
    }

    pub fn loads(text: &str, ignore: bool) -> Option<T> {
        match serde_json::from_str(text) {
            Ok(c) => Some(c),
            Err(e) => {
                if !ignore {
                    tracing::error!(
                        func = "serde_json::from_str",
                        error_tag = ErrorTag::JsonDeserializeError.as_ref(),
                        error_str = e.to_string(),
                        message = text,
                    );
                }
                None
            }
        }
    }

    pub fn dump(&self, ignore: bool) -> bool {
        Self::dump_data(&self.inner, &self.path, ignore)
    }

    pub fn dump_data(data: &T, path: &str, ignore: bool) -> bool {
        let text = Self::dumps_data(data, ignore);
        if text.is_empty() {
            return false;
        }

        match std::fs::write(&path, text.as_bytes()) {
            Ok(_) => true,
            Err(e) => {
                if !ignore {
                    tracing::error!(
                        func = "std::fs::write",
                        path = path,
                        error_tag = ErrorTag::WriteFileError.as_ref(),
                        error_str = e.to_string(),
                        messsage = text,
                    );
                }
                false
            }
        }
    }

    pub fn dumps(&self, ignore: bool) -> String {
        Self::dumps_data(&self.inner, ignore)
    }

    pub fn dumps_data(data: &T, ignore: bool) -> String {
        match serde_json::to_string_pretty(data) {
            Ok(text) => text,
            Err(e) => {
                if !ignore {
                    tracing::error!(
                        func = "serde_json::to_string_pretty",
                        error_tag = ErrorTag::JsonSerializeError.as_ref(),
                        error_str = e.to_string(),
                    );
                }
                String::new()
            }
        }
    }
}
