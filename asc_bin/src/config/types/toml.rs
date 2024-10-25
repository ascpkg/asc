use serde::{de::DeserializeOwned, Deserialize, Serialize};

use config_file_macros::generate_wrapper_methods;

use crate::errors::ErrorTag;

#[derive(Deserialize, Serialize, Debug)]
pub struct Wrapper<T> {
    inner: T,
    path: String,
}

generate_wrapper_methods!(
    toml,
    from_str,
    to_string_pretty,
    "toml::from_str",
    "toml::to_string_pretty",
    ErrorTag::TomlDeserializeError.as_ref(),
    ErrorTag::TomlSerializeError.as_ref()
);
