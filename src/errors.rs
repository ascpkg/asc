use strum_macros::{AsRefStr, FromRepr};

#[derive(Clone, Debug, PartialEq, AsRefStr, FromRepr)]
#[strum(serialize_all = "snake_case")]
pub enum ErrorTag {
    Ok,
    // invalid args
    InvalidCliArgsError,
    // invalid data
    InvalidProjectPackageError,
    InvalidProjectWorkspaceError,
    // invalid filesystem
    FileExistsError,
    DirectoryExistsError,
    PathExistsError,
    FileNotFoundError,
    DirectoryNotFoundError,
    PathNotFoundError,
    ReadFileError,
    WriteFileError,
    CretaeDirectoryError,
    // serialize/seserialize/render error
    TomlSerializeError,
    TomlDeserializeError,
    JsonSerializeError,
    JsonDeserializeError,
    RenderHandlebarsError,
}
