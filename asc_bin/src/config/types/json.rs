use serde::{de::DeserializeOwned, Deserialize, Serialize};

use config_file_macros::generate_wrapper_methods;

use crate::errors::ErrorTag;

#[derive(Deserialize, Serialize, Debug)]
pub struct Wrapper<T> {
    inner: T,
    path: String,
}

generate_wrapper_methods!(
    serde_json,
    from_str,
    to_string_pretty,
    "from_str",
    "to_string_pretty",
    ErrorTag::JsonDeserializeError.as_ref(),
    ErrorTag::JsonSerializeError.as_ref()
);
