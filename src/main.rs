mod client;
mod err;
mod operations;

use crate::operations::{DefinitionsResponse, Modus, SynonymsResponse};
use clap::{Parser, Subcommand};
use client::{definitions, setup, synonyms};
use err::Error;

fn main() -> Result<(), Error> {
    let args = Args::parse();
    match args.command {
        Commands::Define { word } => match definitions(word.as_str()) {
            Ok(d) => println!("{}", d),
            Err(e) => eprintln!("{}", e),
        },
        Commands::Synonyms { word } => match synonyms(word.as_str()) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e),
        },
        Commands::Setup => {
            setup()?;
        }
        _ => {
            eprintln!("Command not implemented");
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
