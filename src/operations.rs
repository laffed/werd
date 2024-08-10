use serde::Deserialize;
use std::fmt::Display;

#[derive(strum::IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum Modus {
    Synonyms,
    Definitions,
}

#[derive(Deserialize)]
pub struct SynonymsResponse {
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

#[derive(Deserialize)]
pub struct DefinitionsResponse {
    word: String,
    definitions: Vec<Definition>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
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
