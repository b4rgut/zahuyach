use crate::error::{Result, ZahuyachError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub site: SiteConfig,
    pub build: BuildConfig,
    pub markdown: Option<MarkdownConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SiteConfig {
    pub title: String,
    pub description: String,
    pub author: String,
    pub base_url: String,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildConfig {
    pub output_dir: String,
    pub content_dir: String,
    pub static_dir: String,
    pub templates_dir: String,
    pub clean_output: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarkdownConfig {
    pub enable_tables: Option<bool>,
    pub enable_footnotes: Option<bool>,
    pub enable_strikethrough: Option<bool>,
    pub syntax_highlighting: Option<bool>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| ZahuyachError::InvalidInput(format!("Failed to parse config: {}", e)))?;
        Ok(config)
    }
}
