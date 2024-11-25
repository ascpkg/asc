use hcl;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use config_file_macros::generate_wrapper_methods;

#[derive(Deserialize, Serialize, Debug)]
pub struct HclConfigFileWrapper<T> {
    inner: T,
    path: String,
}

generate_wrapper_methods!(
    HclConfigFileWrapper,
    hcl::from_str,
    hcl::to_string,
    hcl::to_string,
    "HclDeserializeError",
    "HclSerializeError"
);
