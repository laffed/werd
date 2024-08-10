use dialoguer::Password;
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::{DefinitionsResponse, Error, Modus, SynonymsResponse};

const BASE_URL: &str = "https://wordsapiv1.p.rapidapi.com/words/";

pub fn synonyms(word: &str) -> Result<SynonymsResponse, Error> {
    let client = get_client(word, Modus::Synonyms)?;
    let res = client.send()?.json()?;
    Ok(res)
}

pub fn definitions(word: &str) -> Result<DefinitionsResponse, Error> {
    let client = get_client(word, Modus::Definitions)?;
    let res = client.send()?.json()?;
    Ok(res)
}

pub fn setup() -> Result<(), Error> {
    let key = Password::new()
        .with_prompt("Enter your API key")
        .interact()?;

    let config = Config { key };
    let toml = toml::to_string(&config)?;

    let path = get_key_path()?;
    fs::write(path, toml)?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
struct Config {
    key: String,
}

fn get_client(word: &str, modus: Modus) -> Result<RequestBuilder, Error> {
    let url = BASE_URL.to_string() + word + "/" + modus.into();
    let client = reqwest::blocking::Client::new();
    Ok(client
        .get(url)
        .header("x-rapidapi-key", get_key()?)
        .header("x-rapidapi-host", "wordsapiv1.p.rapidapi.com"))
}

fn get_key_path() -> Result<std::path::PathBuf, Error> {
    let home_dir = dirs::home_dir().ok_or(Error::Path)?;
    Ok(home_dir.join(".werd.toml"))
}

fn get_key() -> Result<String, Error> {
    let path = get_key_path()?;
    let config = fs::read_to_string(path).map_err(|_| Error::ConfigFileNotFound)?;
    let config: Config = toml::from_str(&config).map_err(|_| Error::ConfigFile)?;
    Ok(config.key)
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
