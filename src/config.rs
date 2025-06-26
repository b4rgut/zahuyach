use crate::error::{Result, ZahuyachError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub site: SiteConfig,
    pub build: BuildConfig,
    pub markdown: Option<MarkdownConfig>,
    pub content: Option<ContentConfig>,
    pub rss: Option<RssConfig>,
    pub pagination: Option<PaginationConfig>,
    pub features: Option<FeaturesConfig>,
    pub date_format: Option<DateFormatConfig>,
    pub taxonomy: Option<TaxonomyConfig>,
    pub display: Option<DisplayConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SiteConfig {
    pub title: String,
    pub description: String,
    pub author: String,
    pub base_url: String,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub email: Option<String>,
    pub social: Option<SocialConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildConfig {
    pub output_dir: String,
    pub content_dir: String,
    pub static_dir: String,
    pub templates_dir: String,
    pub clean_output: Option<bool>,
    pub generate_rss: Option<bool>,
    pub rss_limit: Option<usize>,
    pub generate_sitemap: Option<bool>,
    pub clean_urls: Option<bool>,
    pub include_drafts: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarkdownConfig {
    pub enable_tables: Option<bool>,
    pub enable_footnotes: Option<bool>,
    pub enable_strikethrough: Option<bool>,
    pub enable_tasklists: Option<bool>,
    pub enable_smart_punctuation: Option<bool>,
    pub enable_heading_attributes: Option<bool>,
    pub syntax_highlighting: Option<bool>,
    pub syntax_theme: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentConfig {
    pub enable_smart_quotes: Option<bool>,
    pub enable_emoji: Option<bool>,
    pub external_links_new_tab: Option<bool>,
    pub add_anchor_links: Option<bool>,
    pub auto_excerpt: Option<bool>,
    pub excerpt_length: Option<usize>,
    pub excerpt_separator: Option<String>,
    pub reading_speed: Option<usize>, // words per minute
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RssConfig {
    pub enabled: Option<bool>,
    pub limit: Option<usize>,
    pub full_content: Option<bool>,
    pub filename: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaginationConfig {
    pub posts_per_page: Option<usize>,
    pub pages_in_nav: Option<usize>,
    pub infinite_scroll: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FeaturesConfig {
    pub dark_mode: Option<bool>,
    pub light_mode: Option<bool>,
    pub auto_theme: Option<bool>,
    pub responsive: Option<bool>,
    pub seo_optimized: Option<bool>,
    pub rss_feed: Option<bool>,
    pub syntax_highlighting: Option<bool>,
    pub reading_time: Option<bool>,
    pub social_links: Option<bool>,
    pub code_copy: Option<bool>,
    pub back_to_top: Option<bool>,
    pub reading_progress: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DateFormatConfig {
    pub posts: Option<String>,
    pub archive: Option<String>,
    pub full: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaxonomyConfig {
    pub enable_tags: Option<bool>,
    pub enable_categories: Option<bool>,
    pub min_tag_count: Option<usize>,
    pub max_tags_in_cloud: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DisplayConfig {
    pub popular_tags_limit: Option<usize>,
    pub recent_posts_limit: Option<usize>,
    pub related_posts_limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SocialConfig {
    pub github: Option<String>,
    pub twitter: Option<String>,
    pub linkedin: Option<String>,
    pub email: Option<String>,
    pub mastodon: Option<String>,
    pub youtube: Option<String>,
    pub instagram: Option<String>,
    pub facebook: Option<String>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| ZahuyachError::InvalidInput(format!("Failed to parse config: {}", e)))?;
        Ok(config)
    }

    // Helper methods to get values with defaults
    pub fn get_excerpt_separator(&self) -> &str {
        self.content
            .as_ref()
            .and_then(|c| c.excerpt_separator.as_deref())
            .unwrap_or("<!-- more -->")
    }

    pub fn get_excerpt_length(&self) -> usize {
        self.content
            .as_ref()
            .and_then(|c| c.excerpt_length)
            .unwrap_or(3) // 3 paragraphs by default
    }

    pub fn get_reading_speed(&self) -> usize {
        self.content
            .as_ref()
            .and_then(|c| c.reading_speed)
            .unwrap_or(200) // 200 words per minute
    }

    pub fn get_rss_limit(&self) -> usize {
        self.rss
            .as_ref()
            .and_then(|r| r.limit)
            .or(self.build.rss_limit)
            .unwrap_or(20)
    }

    pub fn get_posts_per_page(&self) -> usize {
        self.pagination
            .as_ref()
            .and_then(|p| p.posts_per_page)
            .unwrap_or(10)
    }

    pub fn get_rss_filename(&self) -> &str {
        self.rss
            .as_ref()
            .and_then(|r| r.filename.as_deref())
            .unwrap_or("feed.xml")
    }

    pub fn is_rss_enabled(&self) -> bool {
        self.rss
            .as_ref()
            .and_then(|r| r.enabled)
            .or(self.build.generate_rss)
            .unwrap_or(true)
    }

    pub fn should_clean_output(&self) -> bool {
        self.build.clean_output.unwrap_or(true)
    }

    pub fn include_drafts(&self) -> bool {
        self.build.include_drafts.unwrap_or(false)
    }

    pub fn get_max_tags_in_cloud(&self) -> usize {
        self.taxonomy
            .as_ref()
            .and_then(|t| t.max_tags_in_cloud)
            .unwrap_or(50)
    }

    pub fn get_min_tag_count(&self) -> usize {
        self.taxonomy
            .as_ref()
            .and_then(|t| t.min_tag_count)
            .unwrap_or(1)
    }

    pub fn is_tags_enabled(&self) -> bool {
        self.taxonomy
            .as_ref()
            .and_then(|t| t.enable_tags)
            .unwrap_or(true)
    }

    pub fn is_categories_enabled(&self) -> bool {
        self.taxonomy
            .as_ref()
            .and_then(|t| t.enable_categories)
            .unwrap_or(false)
    }

    pub fn get_popular_tags_limit(&self) -> usize {
        self.display
            .as_ref()
            .and_then(|d| d.popular_tags_limit)
            .unwrap_or(20)
    }

    pub fn get_recent_posts_limit(&self) -> usize {
        self.display
            .as_ref()
            .and_then(|d| d.recent_posts_limit)
            .unwrap_or(5)
    }

    pub fn get_related_posts_limit(&self) -> usize {
        self.display
            .as_ref()
            .and_then(|d| d.related_posts_limit)
            .unwrap_or(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = Config {
            site: SiteConfig {
                title: "Test".to_string(),
                description: "Test".to_string(),
                author: "Test".to_string(),
                base_url: "http://test.com".to_string(),
                language: None,
                timezone: None,
                email: None,
                social: None,
            },
            build: BuildConfig {
                output_dir: "dist".to_string(),
                content_dir: "content".to_string(),
                static_dir: "static".to_string(),
                templates_dir: "templates".to_string(),
                clean_output: None,
                generate_rss: None,
                rss_limit: None,
                generate_sitemap: None,
                clean_urls: None,
                include_drafts: None,
            },
            markdown: None,
            content: None,
            rss: None,
            pagination: None,
            features: None,
            date_format: None,
            taxonomy: None,
            display: None,
        };

        assert_eq!(config.get_excerpt_separator(), "<!-- more -->");
        assert_eq!(config.get_excerpt_length(), 3);
        assert_eq!(config.get_reading_speed(), 200);
        assert_eq!(config.get_rss_limit(), 20);
        assert_eq!(config.get_posts_per_page(), 10);
        assert_eq!(config.get_rss_filename(), "feed.xml");
        assert!(config.is_rss_enabled());
        assert!(config.should_clean_output());
        assert!(!config.include_drafts());
    }

    #[test]
    fn test_config_from_toml() {
        let toml_str = r#"
[site]
title = "Test Blog"
description = "A test blog"
author = "Test Author"
base_url = "https://test.com"
language = "en"
timezone = "Europe/Moscow"
email = "test@test.com"

[site.social]
github = "https://github.com/test"
twitter = "https://twitter.com/test"
linkedin = "https://linkedin.com/in/test"
email = "social@test.com"

[build]
output_dir = "public"
content_dir = "posts"
static_dir = "assets"
templates_dir = "themes"
clean_output = false
generate_rss = true
rss_limit = 30
generate_sitemap = true
clean_urls = false
include_drafts = true

[markdown]
enable_tables = true
enable_footnotes = false
enable_strikethrough = true
enable_tasklists = true
enable_smart_punctuation = false
enable_heading_attributes = true
syntax_highlighting = true
syntax_theme = "monokai"

[content]
enable_smart_quotes = false
enable_emoji = true
external_links_new_tab = false
add_anchor_links = true
auto_excerpt = false
excerpt_length = 5
excerpt_separator = "<!--break-->"
reading_speed = 250

[rss]
enabled = false
limit = 15
full_content = false
filename = "atom.xml"

[pagination]
posts_per_page = 20
pages_in_nav = 7
infinite_scroll = true

[features]
dark_mode = true
light_mode = false
auto_theme = true
reading_time = false

[date_format]
posts = "%d/%m/%Y"
archive = "%d.%m"
full = "%d/%m/%Y %H:%M"

[taxonomy]
enable_tags = false
enable_categories = true
min_tag_count = 2
max_tags_in_cloud = 100

[display]
popular_tags_limit = 30
recent_posts_limit = 10
related_posts_limit = 5
"#;

        let config: Config = toml::from_str(toml_str).unwrap();

        // Test site config
        assert_eq!(config.site.title, "Test Blog");
        assert_eq!(config.site.description, "A test blog");
        assert_eq!(config.site.author, "Test Author");
        assert_eq!(config.site.base_url, "https://test.com");
        assert_eq!(config.site.language.as_ref().unwrap(), "en");
        assert_eq!(config.site.timezone.as_ref().unwrap(), "Europe/Moscow");
        assert_eq!(config.site.email.as_ref().unwrap(), "test@test.com");

        // Test social config
        let social = config.site.social.as_ref().unwrap();
        assert_eq!(social.github.as_ref().unwrap(), "https://github.com/test");
        assert_eq!(social.twitter.as_ref().unwrap(), "https://twitter.com/test");
        assert_eq!(social.email.as_ref().unwrap(), "social@test.com");

        // Test build config
        assert_eq!(config.build.output_dir, "public");
        assert_eq!(config.build.content_dir, "posts");
        assert_eq!(config.build.static_dir, "assets");
        assert_eq!(config.build.templates_dir, "themes");
        assert_eq!(config.build.clean_output, Some(false));
        assert_eq!(config.build.generate_rss, Some(true));
        assert_eq!(config.build.rss_limit, Some(30));
        assert_eq!(config.build.include_drafts, Some(true));

        // Test markdown config
        let markdown = config.markdown.as_ref().unwrap();
        assert_eq!(markdown.enable_tables, Some(true));
        assert_eq!(markdown.enable_footnotes, Some(false));
        assert_eq!(markdown.syntax_theme.as_ref().unwrap(), "monokai");

        // Test content config
        let content = config.content.as_ref().unwrap();
        assert_eq!(content.enable_smart_quotes, Some(false));
        assert_eq!(content.excerpt_separator.as_ref().unwrap(), "<!--break-->");
        assert_eq!(content.reading_speed, Some(250));

        // Test RSS config
        let rss = config.rss.as_ref().unwrap();
        assert_eq!(rss.enabled, Some(false));
        assert_eq!(rss.limit, Some(15));
        assert_eq!(rss.filename.as_ref().unwrap(), "atom.xml");

        // Test helper methods
        assert_eq!(config.get_excerpt_separator(), "<!--break-->");
        assert_eq!(config.get_excerpt_length(), 5);
        assert_eq!(config.get_reading_speed(), 250);
        assert_eq!(config.get_rss_limit(), 15);
        assert_eq!(config.get_posts_per_page(), 20);
        assert_eq!(config.get_rss_filename(), "atom.xml");
        assert!(!config.is_rss_enabled());
        assert!(!config.should_clean_output());
        assert!(config.include_drafts());
        assert_eq!(config.get_popular_tags_limit(), 30);
        assert_eq!(config.get_recent_posts_limit(), 10);
        assert_eq!(config.get_max_tags_in_cloud(), 100);
        assert!(!config.is_tags_enabled());
        assert!(config.is_categories_enabled());
    }
}
