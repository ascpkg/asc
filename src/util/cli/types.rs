use clap::ValueEnum;

use strum_macros::{AsRefStr, FromRepr};

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr, FromRepr)]
#[clap(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ActionType {
    All = 0,
    Scan = 1,
    Configure = 2,
    Build = 3,
    Install = 4,
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr, FromRepr)]
#[clap(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CMakeTargetType {
    Executable = 0,
    Library = 1,
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr, FromRepr)]
#[clap(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CMakeLibraryType {
    Static = 0,
    Shared = 1,
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr, FromRepr)]
#[clap(rename_all = "snake_case")]
pub enum CMakeConfigType {
    Debug = 0,
    Release = 1,
}
