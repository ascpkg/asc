use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

use config_file_macros::generate_wrapper_methods;

#[derive(Deserialize, Serialize, Debug)]
pub struct JsonConfigFileWrapper<T> {
    inner: T,
    path: String,
}

generate_wrapper_methods!(
    JsonConfigFileWrapper,
    serde_json::from_str,
    serde_json::to_string,
    serde_json::to_string_pretty,
    "JsonDeserializeError",
    "JsonSerializeError"
);
