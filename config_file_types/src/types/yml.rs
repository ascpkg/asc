use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yml;

use config_file_macros::generate_wrapper_methods;

#[derive(Deserialize, Serialize, Debug)]
pub struct YmlConfigFileWrapper<T> {
    inner: T,
    path: String,
}

generate_wrapper_methods!(
    YmlConfigFileWrapper,
    serde_yml::from_str,
    serde_yml::to_string,
    serde_yml::to_string,
    "YmlDeserializeError",
    "YmlSerializeError"
);
