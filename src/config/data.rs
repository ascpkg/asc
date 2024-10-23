use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct InstalledFiles {
    pub prefix: String,
    pub files: Vec<String>,
}
