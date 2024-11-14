use serde::{de::DeserializeOwned, Deserialize, Serialize};
use toml;

use config_file_macros::generate_wrapper_methods;

#[derive(Deserialize, Serialize, Debug)]
pub struct TomlConfigFileWrapper<T> {
    inner: T,
    path: String,
}

generate_wrapper_methods!(
    TomlConfigFileWrapper,
    toml,
    from_str,
    to_string_pretty,
    "toml::from_str",
    "toml::to_string_pretty",
    "TomlDeserializeError",
    "TomlSerializeError"
);
