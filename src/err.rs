use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Network error")]
    Network(#[from] reqwest::Error),
    #[error("TOML Deserialization error")]
    TomlDe(#[from] toml::de::Error),
    #[error("TOML Serialization error")]
    TomlSer(#[from] toml::ser::Error),
    #[error("Input error")]
    Input(#[from] dialoguer::Error),
    #[error("Home directory not found")]
    Path,
    #[error("API key not found.\nPlease run 'werd setup'")]
    ConfigFileNotFound,
    #[error("Could not read API key from config.\nPlease run 'werd setup' again")]
    ConfigFile,
}
