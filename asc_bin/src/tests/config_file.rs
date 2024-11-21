use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;
use config_file_types;

#[derive(Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("toml")]
pub struct MyTomlConfig {
    pub dependencies: BTreeMap<String, BTreeSet<String>>,

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

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("xml")]
pub struct MyXmlConfig {
    pub dependencies: BTreeMap<String, BTreeSet<String>>,

    #[serde(skip)]
    pub path: String,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("yml")]
pub struct MyYmlConfig {
    pub dependencies: BTreeMap<String, BTreeSet<String>>,

    #[serde(skip)]
    pub path: String,
}

fn main() {
    // load from file or default
    let mut my_toml = MyTomlConfig::load("target/debug/test.toml", true).unwrap();
    // update
    my_toml.dependencies.insert(String::from("a"), BTreeSet::from([String::from("b"), String::from("c")]));
    // serialize to string (pretty & output error)
    println!("{}\n", my_toml.dumps(true, false));
    // serialize to file (pretty & output error)
    my_toml.dump(true, false);
    // load from file or panic
    println!("{:#?}\n\n", MyTomlConfig::load("target/debug/test.toml", false).unwrap());

    // load from file or default
    let mut my_json = MyJsonConfig::load("target/debug/test.json", true).unwrap();
    // update
    my_json.dependencies.insert(String::from("a"), BTreeSet::from([String::from("b"), String::from("c")]));
    // serialize to string (pretty & output error)
    println!("{}\n", my_toml.dumps(true, false));
    // serialize to file (pretty & output error)
    my_json.dump(true, false);
    // load from file or panic
    println!("{:#?}\n\n", MyJsonConfig::load("target/debug/test.json", false).unwrap());

    // load from file or default
    let mut my_xml = MyXmlConfig::load("target/debug/test.xml", true).unwrap();
    // update
    my_xml.dependencies.insert(String::from("a"), BTreeSet::from([String::from("b"), String::from("c")]));
    // serialize to string (no indent & output error)
    println!("{}\n", my_xml.dumps(false, false));
    // serialize to file (no indent & output error)
    my_xml.dump(false, false);
    // load from file or panic
    println!("{:#?}\n\n", MyXmlConfig::load("target/debug/test.xml", false).unwrap());

    // load from file or default
    let mut my_yml = MyYmlConfig::load("target/debug/test.yml", true).unwrap();
    // update
    my_yml.dependencies.insert(String::from("a"), BTreeSet::from([String::from("b"), String::from("c")]));
    // serialize to string (no indent & output error)
    println!("{}\n", my_yml.dumps(false, false));
    // serialize to file (no indent & output error)
    my_yml.dump(false, false);
    // load from file or panic
    println!("{:#?}\n\n", MyYmlConfig::load("target/debug/test.yml", false).unwrap());
}