use strum_macros::{AsRefStr, FromRepr};

#[derive(Clone, Debug, PartialEq, AsRefStr, FromRepr)]
#[strum(serialize_all = "snake_case")]
pub enum ErrorTag {
    Ok,
    // invalid args
    InvalidCliArgsError,
    // invalid data
    InvalidProjectError,
    InvalidProjectPackageError,
    InvalidProjectWorkspaceError,
    // invalid filesystem
    // exists
    FileExistsError,
    DirectoryExistsError,
    PathExistsError,
    // not exists
    FileNotFoundError,
    DirectoryNotFoundError,
    PathNotFoundError,
    // read write
    ReadFileError,
    WriteFileError,
    CreateDirectoryError,
    // remove
    RemoveFileError,
    RemoveDirectoryError,
    // serialize/seserialize/render error
    // toml
    TomlSerializeError,
    TomlDeserializeError,
    // json
    JsonSerializeError,
    JsonDeserializeError,
    // handlebars
    RenderHandlebarsError,
}
