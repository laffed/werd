use clap::{Parser, Subcommand};
use dialoguer::Password;
use serde::{Deserialize, Serialize};
use std::fs;

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

fn get_key() -> Result<String, Box<dyn std::error::Error>> {
    let config = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config)?;
    Ok(config.key)
}

fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let key = Password::new()
        .with_prompt("Enter your API key")
        .interact()?;
    let config = Config { key };
    let toml = toml::to_string(&config)?;
    fs::write("config.toml", toml)?;
    Ok(())
}
