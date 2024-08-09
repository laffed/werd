use clap::{Parser, Subcommand};
use dialoguer::Password;
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs};
use thiserror::Error;

const BASE_URL: &str = "https://wordsapiv1.p.rapidapi.com/words/";

fn main() -> Result<(), Error> {
    let args = Args::parse();
    match args.command {
        Commands::Define { word } => {
            let d = definitions(word.as_str())?;
            println!("{}", d);
        }
        Commands::Synonyms { word } => {
            let s = synonyms(word.as_str())?;
            println!("{}", s);
        }
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

#[derive(strum::IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
enum Modus {
    Synonyms,
    Definitions,
}

#[derive(Deserialize)]
struct SynonymsResponse {
    word: String,
    synonyms: Vec<String>,
}

impl Display for SynonymsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let synonyms = self
            .synonyms
            .iter()
            .fold(String::new(), |acc, x| acc + x + "\n");

        write!(f, "Synonyms for '{}':\n\n{}", self.word, synonyms)
    }
}

fn synonyms(word: &str) -> Result<SynonymsResponse, Error> {
    let client = get_client(word, Modus::Synonyms)?;
    let res = client.send()?.json()?;
    Ok(res)
}

#[derive(Deserialize)]
struct DefinitionsResponse {
    word: String,
    definitions: Vec<Definition>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Definition {
    definition: String,
    part_of_speech: String,
}

impl Display for DefinitionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let definitions = self.definitions.iter().fold(String::new(), |acc, x| {
            format!("{}({}) {}\n", acc, x.part_of_speech, x.definition)
        });

        write!(f, "Definitions for '{}':\n\n{}", self.word, definitions)
    }
}

fn definitions(word: &str) -> Result<DefinitionsResponse, Error> {
    let client = get_client(word, Modus::Definitions)?;
    let res = client.send()?.json()?;
    Ok(res)
}

fn get_client(word: &str, modus: Modus) -> Result<RequestBuilder, Error> {
    let url = BASE_URL.to_string() + word + "/" + modus.into();
    let client = reqwest::blocking::Client::new();
    Ok(client
        .get(url)
        .header("x-rapidapi-key", get_key()?)
        .header("x-rapidapi-host", "wordsapiv1.p.rapidapi.com"))
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
    #[error("Network error")]
    NetworkError(#[from] reqwest::Error),
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
