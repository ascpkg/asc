use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::errors::ErrorTag;

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlContainer<T> {
    inner: T,
    path: String,
}

impl<T> TomlContainer<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(data: T, path: &str) -> Self {
        Self {
            inner: data,
            path: path.to_string(),
        }
    }

    pub fn load(path: &str) -> Option<T> {
        match std::fs::read_to_string(path) {
            Ok(text) => Self::loads(&text),
            Err(e) => {
                tracing::error!(
                    func = "std::fs::read_to_string",
                    path = path,
                    error_tag = ErrorTag::ReadFileError.as_ref(),
                    error_str = e.to_string(),
                );
                None
            }
        }
    }

    pub fn loads(text: &str) -> Option<T> {
        match toml::from_str(text) {
            Ok(c) => Some(c),
            Err(e) => {
                tracing::error!(
                    func = "toml::from_str",
                    error_tag = ErrorTag::TomlDeserializeError.as_ref(),
                    error_str = e.to_string(),
                    message = text,
                );
                None
            }
        }
    }

    pub fn dump(&self) -> bool {
        let text = self.dumps();
        if text.is_empty() {
            return false;
        }

        match std::fs::write(&self.path, text.as_bytes()) {
            Ok(_) => true,
            Err(e) => {
                tracing::error!(
                    func = "std::fs::write",
                    path = self.path,
                    error_tag = ErrorTag::WriteFileError.as_ref(),
                    error_str = e.to_string(),
                    messsage = text,
                );
                false
            }
        }
    }

    pub fn dumps(&self) -> String {
        match toml::to_string_pretty(&self.inner) {
            Ok(text) => text,
            Err(e) => {
                tracing::error!(
                    func = "toml::to_string_pretty",
                    error_tag = ErrorTag::TomlSerializeError.as_ref(),
                    error_str = e.to_string(),
                );
                String::new()
            }
        }
    }
}
