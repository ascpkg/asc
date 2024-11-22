use quick_xml;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use config_file_macros::generate_wrapper_methods;

#[derive(Deserialize, Serialize, Debug)]
pub struct XmlConfigFileWrapper<T> {
    inner: T,
    path: String,
}

generate_wrapper_methods!(
    XmlConfigFileWrapper,
    quick_xml::de::from_str,
    quick_xml::se::to_string,
    quick_xml::se::to_string,
    "XmlDeserializeError",
    "XmlSerializeError"
);
