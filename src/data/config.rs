use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: General,
    pub filetype: HashMap<String, Filetype>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub selenium_browser: String,
    pub default_filetype: Option<String>,
    pub editor_command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filetype {
    pub language: String,
    pub main: String,
    pub run: String,
    pub source_templates: Option<Vec<String>>,
    pub root_templates: Option<Vec<String>>,
    pub compile: Option<String>,
    pub after: Option<String>,
}

#[allow(dead_code)]
fn default_archive_dir() -> String {
    "archives".to_string()
}

impl Config {
    #[allow(dead_code)]
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    #[allow(dead_code)]
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
