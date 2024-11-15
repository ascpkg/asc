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
    serde_json,
    from_str,
    to_string,
    to_string_pretty,
    "from_str",
    "to_string",
    "JsonDeserializeError",
    "JsonSerializeError"
);
