use crate::error::{Result, ZahuyachError};
use pulldown_cmark::{Parser, html};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub front_matter: FrontMatter,
    pub content: String,
    pub html_content: String,
    pub slug: String,
    pub file_path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub description: Option<String>,
    pub draft: Option<bool>,
    pub featured: Option<bool>,
}

impl Post {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;

        // Разделяем front matter и содержимое
        let (front_matter_str, markdown_content) = Self::parse_front_matter(&content)?;

        // Парсим front matter
        let front_matter: FrontMatter = serde_yaml::from_str(&front_matter_str).map_err(|e| {
            ZahuyachError::InvalidInput(format!("Failed to parse front matter: {}", e))
        })?;

        // Конвертируем Markdown в HTML
        let parser = Parser::new(&markdown_content);
        let mut html_content = String::new();
        html::push_html(&mut html_content, parser);

        // Создаем slug из имени файла
        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled")
            .to_string();

        Ok(Post {
            front_matter,
            content: markdown_content,
            html_content,
            slug,
            file_path: path.to_path_buf(),
        })
    }

    fn parse_front_matter(content: &str) -> Result<(String, String)> {
        if !content.starts_with("---") {
            return Err(ZahuyachError::InvalidInput(
                "No front matter found".to_string(),
            ));
        }

        let mut parts = content.splitn(3, "---");
        parts.next(); // Пропускаем первую пустую часть

        let front_matter = parts.next().ok_or_else(|| {
            ZahuyachError::InvalidInput("Invalid front matter format".to_string())
        })?;

        let content = parts.next().ok_or_else(|| {
            ZahuyachError::InvalidInput("No content after front matter".to_string())
        })?;

        Ok((front_matter.to_string(), content.to_string()))
    }

    pub fn is_draft(&self) -> bool {
        self.front_matter.draft.unwrap_or(false)
    }
}
