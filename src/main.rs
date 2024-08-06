use clap::{Parser, Subcommand};
use dialoguer::Password;
use serde::{Deserialize, Serialize};
use std::fs;
use thiserror::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Commands::Define { word } => {}
        Commands::Synonyms { word } => {}
        Commands::All { word } => {}
        Commands::Setup => {
            setup()?;
        }
    }

    Ok(())
}

#[derive(Parser)]
#[clap(name = "werd", version = "0.1.0", author = "laffed")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(visible_alias = "d")]
    Define {
        word: String,
    },
    #[command(visible_alias = "s")]
    Synonyms {
        word: String,
    },
    #[command(visible_alias = "a")]
    All {
        word: String,
    },
    Setup,
}

#[derive(Deserialize, Serialize)]
struct Config {
    key: String,
}

fn get_key_path() -> Result<std::path::PathBuf, Error> {
    let home_dir = dirs::home_dir().ok_or(Error::PathError)?;
    Ok(home_dir.join(".werd.toml"))
}

fn get_key() -> Result<String, Error> {
    let path = get_key_path()?;
    let config = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config)?;
    Ok(config.key)
}

fn setup() -> Result<(), Error> {
    let key = Password::new()
        .with_prompt("Enter your API key")
        .interact()?;

    let config = Config { key };
    let toml = toml::to_string(&config)?;

    let path = get_key_path()?;
    fs::write(path, toml)?;
    Ok(())
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("TOML Deserialization error")]
    TomlDeError(#[from] toml::de::Error),
    #[error("TOML Serialization error")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("Input error")]
    InputError(#[from] dialoguer::Error),
    #[error("Home directory not found")]
    PathError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_key() {
        let key = get_key().unwrap();
        assert_eq!(key, "t");
    }
}
