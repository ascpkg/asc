# Wrap struct to file configuration

```rust
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;
use config_file_types;

#[derive(Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("toml")]
pub struct MyTomlConfig {
    pub files: Vec<String>,

    #[serde(skip)]
    pub path: String,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
pub struct MyJsonConfig {
    pub dependencies: BTreeMap<String, BTreeSet<String>>,

    #[serde(skip)]
    pub path: String,
}

fn main() {
    // load from file or default
    let mut my_toml = MyTomlConfig::load("test.toml", true).unwrap();
    // update
    my_toml.files.push(String::from("a.txt"));
    // serialize to string (pretty & output error)
    let my_toml_text = my_toml.dumps(true, false);
    // serialize to file
    my_toml.dump(true, false);

    // load from file or panic
    let my_json = MyTomlConfig::load("test.json", false).unwrap();
    // serialize to string (no indent & ignore error)
    let my_json_text = my_json.dumps(true, true);
    // serialize to file
    my_json.dump(true, false);
}
```