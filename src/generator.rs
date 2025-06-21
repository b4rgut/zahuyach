use crate::config::Config;
use crate::content::Post;
use crate::error::{Result, ZahuyachError};
use chrono::Datelike;
use handlebars::Handlebars;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir; // Добавлен импорт для метода year()

pub struct SiteGenerator {
    config: Config,
    handlebars: Handlebars<'static>,
    posts: Vec<Post>,
}

impl SiteGenerator {
    pub fn new(config: Config) -> Result<Self> {
        let mut handlebars = Handlebars::new();

        // Configure handlebars
        handlebars.set_strict_mode(false);
        handlebars.register_escape_fn(handlebars::html_escape);

        Ok(SiteGenerator {
            config,
            handlebars,
            posts: Vec::new(),
        })
    }

    pub fn build(&mut self) -> Result<()> {
        println!("🚀 Starting site generation...");

        // 1. Очистка выходной директории
        self.clean_output_dir()?;
        println!("✅ Output directory cleaned");

        // 2. Загрузка постов
        self.load_posts()?;
        println!("✅ Loaded {} posts", self.posts.len());

        // 3. Загрузка шаблонов
        self.load_templates()?;
        println!("✅ Templates loaded");

        // 4. Генерация HTML страниц
        self.generate_posts()?;
        println!("✅ Individual posts generated");

        self.generate_index()?;
        println!("✅ Index page generated");

        self.generate_archive()?;
        println!("✅ Archive page generated");

        self.generate_tags_pages()?;
        println!("✅ Tags pages generated");

        self.generate_categories_pages()?;
        println!("✅ Categories pages generated");

        self.generate_rss_feed()?;
        println!("✅ RSS feed generated");

        // 5. Копирование статических файлов
        self.copy_static_files()?;
        println!("✅ Static files copied");

        println!("🎉 Site generation completed successfully!");
        Ok(())
    }

    fn clean_output_dir(&self) -> Result<()> {
        let output_path = Path::new(&self.config.build.output_dir);

        if output_path.exists() && self.config.build.clean_output.unwrap_or(true) {
            fs::remove_dir_all(output_path)?;
        }

        fs::create_dir_all(output_path)?;

        // Create necessary subdirectories
        fs::create_dir_all(output_path.join("posts"))?;
        fs::create_dir_all(output_path.join("categories"))?;
        fs::create_dir_all(output_path.join("tags"))?;
        fs::create_dir_all(output_path.join("static"))?;

        Ok(())
    }

    fn load_posts(&mut self) -> Result<()> {
        let content_dir = Path::new(&self.config.build.content_dir);

        if !content_dir.exists() {
            return Err(ZahuyachError::InvalidInput(format!(
                "Content directory '{}' does not exist",
                content_dir.display()
            )));
        }

        for entry in WalkDir::new(content_dir) {
            let entry = entry.map_err(|e| ZahuyachError::Io(e.into()))?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "md") {
                let post = Post::from_file(path)?;
                if !post.is_draft() {
                    self.posts.push(post);
                }
            }
        }

        // Sort posts by date (newest first)
        self.posts
            .sort_by(|a, b| b.front_matter.date.cmp(&a.front_matter.date));

        Ok(())
    }

    fn load_templates(&mut self) -> Result<()> {
        let templates_dir = Path::new(&self.config.build.templates_dir);

        if !templates_dir.exists() {
            return Err(ZahuyachError::InvalidInput(format!(
                "Templates directory '{}' does not exist",
                templates_dir.display()
            )));
        }

        let mut loaded_templates = Vec::new();

        // Load all .html files as templates
        for entry in WalkDir::new(templates_dir) {
            let entry = entry.map_err(|e| ZahuyachError::Io(e.into()))?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "html") {
                let template_name = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
                    ZahuyachError::InvalidInput("Invalid template name".to_string())
                })?;

                let template_content = fs::read_to_string(path)?;

                self.handlebars
                    .register_template_string(template_name, template_content)
                    .map_err(|e| ZahuyachError::InvalidInput(format!("Template error: {}", e)))?;

                loaded_templates.push(template_name.to_string());
            }
        }

        println!("📝 Loaded templates: {}", loaded_templates.join(", "));
        Ok(())
    }

    // Метод для проверки наличия шаблона
    fn has_template(&self, name: &str) -> bool {
        self.handlebars.get_template(name).is_some()
    }

    fn generate_posts(&self) -> Result<()> {
        let output_dir = Path::new(&self.config.build.output_dir);

        // Проверяем наличие шаблона post
        if !self.has_template("post") {
            println!("⚠️  Template 'post' not found, skipping individual post generation");
            return Ok(());
        }

        for post in &self.posts {
            let post_data = self.create_post_data(post);
            let html = self.handlebars.render("post", &post_data).map_err(|e| {
                ZahuyachError::InvalidInput(format!("Template render error: {}", e))
            })?;

            let post_path = output_dir.join("posts").join(format!("{}.html", post.slug));
            fs::write(post_path, html)?;
        }

        Ok(())
    }

    fn generate_index(&self) -> Result<()> {
        let output_dir = Path::new(&self.config.build.output_dir);

        // Исправлено: передаем ссылки на посты
        let posts_refs: Vec<&Post> = self.posts.iter().collect();

        let context = json!({
            "site": self.get_site_context(),
            "posts": self.get_posts_list_context(posts_refs, 10),
            "popular_tags": self.get_popular_tags(),
            "categories": self.get_categories_tree(),
            "recent_posts": self.get_recent_posts(5),
            "stats": self.get_site_stats(),
            "page": {
                "title": "Главная",
                "description": self.config.site.description,
                "url": "/"
            }
        });

        let html = self
            .handlebars
            .render("index", &context)
            .map_err(|e| ZahuyachError::InvalidInput(format!("Template render error: {}", e)))?;

        let index_path = output_dir.join("index.html");
        fs::write(index_path, html)?;

        Ok(())
    }

    fn generate_archive(&self) -> Result<()> {
        let output_dir = Path::new(&self.config.build.output_dir);

        // Проверяем наличие шаблона archive
        if !self.has_template("archive") {
            println!("⚠️  Template 'archive' not found, skipping archive generation");
            return Ok(());
        }

        // Group posts by year and month
        let mut posts_by_date: HashMap<String, HashMap<String, Vec<&Post>>> = HashMap::new();

        for post in &self.posts {
            let date_parts: Vec<&str> = post.front_matter.date.split('-').collect();
            if date_parts.len() >= 2 {
                let year = date_parts[0];
                let month = date_parts[1];

                posts_by_date
                    .entry(year.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(month.to_string())
                    .or_insert_with(Vec::new)
                    .push(post);
            }
        }

        let context = json!({
            "site": self.get_site_context(),
            "posts_by_date": posts_by_date,
            "total_posts": self.posts.len(),
            "categories": self.get_categories_tree(),
            "popular_tags": self.get_popular_tags(),
            "page": {
                "title": "Архив",
                "description": "Архив всех статей блога",
                "url": "/archive"
            }
        });

        let html = self
            .handlebars
            .render("archive", &context)
            .map_err(|e| ZahuyachError::InvalidInput(format!("Template render error: {}", e)))?;

        let archive_path = output_dir.join("archive.html");
        fs::write(archive_path, html)?;

        Ok(())
    }

    fn generate_tags_pages(&self) -> Result<()> {
        let output_dir = Path::new(&self.config.build.output_dir);
        let tags_dir = output_dir.join("tags");

        // Проверяем наличие шаблонов tags и tag
        if !self.has_template("tags") && !self.has_template("tag") {
            println!("⚠️  Templates 'tags' and 'tag' not found, skipping tags generation");
            return Ok(());
        }

        // Collect all tags
        let mut all_tags: HashMap<String, Vec<&Post>> = HashMap::new();

        for post in &self.posts {
            if let Some(tags) = &post.front_matter.tags {
                for tag in tags {
                    all_tags
                        .entry(tag.clone())
                        .or_insert_with(Vec::new)
                        .push(post);
                }
            }
        }

        // Generate tags index page only if template exists
        if self.has_template("tags") {
            let tags_context = json!({
                "site": self.get_site_context(),
                "all_tags": all_tags.iter().map(|(tag, posts)| {
                    json!({
                        "name": tag,
                        "count": posts.len(),
                        "slug": self.slugify(tag)
                    })
                }).collect::<Vec<_>>(),
                "categories": self.get_categories_tree(),
                "page": {
                    "title": "Теги",
                    "description": "Все теги блога",
                    "url": "/tags"
                }
            });

            let tags_html = self.handlebars.render("tags", &tags_context).map_err(|e| {
                ZahuyachError::InvalidInput(format!("Template render error: {}", e))
            })?;

            fs::write(tags_dir.join("index.html"), tags_html)?;
        }

        // Generate individual tag pages only if template exists
        if self.has_template("tag") {
            for (tag, posts) in all_tags {
                let tag_slug = self.slugify(&tag);
                let tag_context = json!({
                    "site": self.get_site_context(),
                    "tag": {
                        "name": tag,
                        "slug": tag_slug
                    },
                    "posts": self.get_posts_list_context(posts, 0),
                    "categories": self.get_categories_tree(),
                    "popular_tags": self.get_popular_tags(),
                    "page": {
                        "title": format!("Тег: {}", tag),
                        "description": format!("Все статьи с тегом {}", tag),
                        "url": format!("/tags/{}", tag_slug)
                    }
                });

                let tag_html = self.handlebars.render("tag", &tag_context).map_err(|e| {
                    ZahuyachError::InvalidInput(format!("Template render error: {}", e))
                })?;

                fs::write(tags_dir.join(format!("{}.html", tag_slug)), tag_html)?;
            }
        }

        Ok(())
    }

    fn generate_categories_pages(&self) -> Result<()> {
        let output_dir = Path::new(&self.config.build.output_dir);
        let categories_dir = output_dir.join("categories");

        // Проверяем наличие шаблона category
        if !self.has_template("category") {
            println!("⚠️  Template 'category' not found, skipping categories generation");
            return Ok(());
        }

        // Collect all categories
        let mut all_categories: HashMap<String, Vec<&Post>> = HashMap::new();

        for post in &self.posts {
            if let Some(categories) = &post.front_matter.categories {
                for category in categories {
                    all_categories
                        .entry(category.clone())
                        .or_insert_with(Vec::new)
                        .push(post);
                }
            }
        }

        // Generate individual category pages
        for (category, posts) in all_categories {
            let category_slug = self.slugify(&category);
            let category_context = json!({
                "site": self.get_site_context(),
                "category": {
                    "name": category,
                    "slug": category_slug
                },
                "posts": self.get_posts_list_context(posts, 0),
                "categories": self.get_categories_tree(),
                "popular_tags": self.get_popular_tags(),
                "page": {
                    "title": format!("Категория: {}", category),
                    "description": format!("Все статьи в категории {}", category),
                    "url": format!("/categories/{}", category_slug)
                }
            });

            let category_html = self
                .handlebars
                .render("category", &category_context)
                .map_err(|e| {
                    ZahuyachError::InvalidInput(format!("Template render error: {}", e))
                })?;

            fs::create_dir_all(categories_dir.join(&category_slug))?;
            fs::write(
                categories_dir.join(&category_slug).join("index.html"),
                category_html,
            )?;
        }

        Ok(())
    }

    fn generate_rss_feed(&self) -> Result<()> {
        let output_dir = Path::new(&self.config.build.output_dir);

        // Проверяем наличие шаблона rss
        if !self.has_template("rss") {
            println!("⚠️  Template 'rss' not found, generating simple RSS feed");

            // Создаем простой RSS без шаблона
            let rss_content = self.generate_simple_rss()?;
            fs::write(output_dir.join("feed.xml"), rss_content)?;
            return Ok(());
        }

        let rss_posts: Vec<Value> = self.posts.iter()
                .take(20) // Limit to latest 20 posts
                .map(|post| {
                    json!({
                        "title": post.front_matter.title,
                        "link": format!("{}/posts/{}", self.config.site.base_url, post.slug),
                        "description": post.front_matter.description.as_ref().unwrap_or(&post.front_matter.title),
                        "content": post.html_content,
                        "pub_date": post.front_matter.date,
                        "author": post.front_matter.author.as_ref().unwrap_or(&self.config.site.author)
                    })
                })
                .collect();

        let rss_context = json!({
            "site": self.get_site_context(),
            "posts": rss_posts,
            "build_date": chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S %z").to_string()
        });

        let rss_xml = self.handlebars.render("rss", &rss_context).map_err(|e| {
            ZahuyachError::InvalidInput(format!("RSS template render error: {}", e))
        })?;

        fs::write(output_dir.join("feed.xml"), rss_xml)?;

        Ok(())
    }

    fn generate_simple_rss(&self) -> Result<String> {
        let mut rss = String::new();
        rss.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        rss.push_str("\n");
        rss.push_str(r#"<rss version="2.0">"#);
        rss.push_str("\n<channel>");
        rss.push_str(&format!("\n<title>{}</title>", self.config.site.title));
        rss.push_str(&format!(
            "\n<description>{}</description>",
            self.config.site.description
        ));
        rss.push_str(&format!("\n<link>{}</link>", self.config.site.base_url));
        rss.push_str(&format!(
            "\n<lastBuildDate>{}</lastBuildDate>",
            chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S %z")
        ));

        for post in self.posts.iter().take(20) {
            rss.push_str("\n<item>");
            rss.push_str(&format!("\n<title>{}</title>", post.front_matter.title));
            rss.push_str(&format!(
                "\n<link>{}/posts/{}</link>",
                self.config.site.base_url, post.slug
            ));
            rss.push_str(&format!(
                "\n<description>{}</description>",
                post.front_matter
                    .description
                    .as_ref()
                    .unwrap_or(&post.front_matter.title)
            ));
            rss.push_str(&format!("\n<pubDate>{}</pubDate>", post.front_matter.date));
            rss.push_str("\n</item>");
        }

        rss.push_str("\n</channel>");
        rss.push_str("\n</rss>");

        Ok(rss)
    }

    fn copy_static_files(&self) -> Result<()> {
        let static_dir = Path::new(&self.config.build.static_dir);
        let output_static_dir = Path::new(&self.config.build.output_dir).join("static");

        if !static_dir.exists() {
            println!(
                "⚠️  Static directory '{}' does not exist, skipping",
                static_dir.display()
            );
            return Ok(());
        }

        self.copy_dir_recursive(static_dir, &output_static_dir)?;

        Ok(())
    }

    fn copy_dir_recursive(&self, src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                self.copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }

    // Helper methods for creating template contexts

    fn create_post_data(&self, post: &Post) -> Value {
        json!({
            "site": self.get_site_context(),
            "post": {
                "title": post.front_matter.title,
                "content": post.html_content,
                "date": post.front_matter.date,
                "date_formatted": self.format_date(&post.front_matter.date),
                "author": post.front_matter.author.as_ref().unwrap_or(&self.config.site.author),
                "tags": post.front_matter.tags.as_ref().unwrap_or(&vec![]),
                "categories": post.front_matter.categories.as_ref().unwrap_or(&vec![]),
                "description": post.front_matter.description.as_ref().unwrap_or(&post.front_matter.title),
                "slug": post.slug,
                "reading_time": self.calculate_reading_time(&post.content),
                "word_count": post.content.split_whitespace().count()
            },
            "categories": self.get_categories_tree(),
            "popular_tags": self.get_popular_tags(),
            "recent_posts": self.get_recent_posts(5),
            "page": {
                "title": post.front_matter.title,
                "description": post.front_matter.description.as_ref().unwrap_or(&post.front_matter.title),
                "url": format!("/posts/{}", post.slug)
            }
        })
    }

    fn get_site_context(&self) -> Value {
        json!({
            "title": self.config.site.title,
            "description": self.config.site.description,
            "author": self.config.site.author,
            "base_url": self.config.site.base_url,
            "language": self.config.site.language.as_ref().unwrap_or(&"ru".to_string()),
            "current_year": chrono::Utc::now().year() // Исправлено: добавлен импорт Datelike
        })
    }

    // Исправлено: изменена сигнатура функции
    fn get_posts_list_context(&self, posts: Vec<&Post>, limit: usize) -> Vec<Value> {
        let posts_to_show = if limit > 0 && posts.len() > limit {
            &posts[..limit]
        } else {
            &posts
        };

        posts_to_show.iter().map(|post| {
            json!({
                "title": post.front_matter.title,
                "slug": post.slug,
                "date": post.front_matter.date,
                "date_formatted": self.format_date(&post.front_matter.date),
                "date_iso": post.front_matter.date,
                "author": post.front_matter.author.as_ref().unwrap_or(&self.config.site.author),
                "tags": post.front_matter.tags.as_ref().unwrap_or(&vec![]),
                "categories": post.front_matter.categories.as_ref().unwrap_or(&vec![]),
                "description": post.front_matter.description.as_ref().unwrap_or(&post.front_matter.title),
                "excerpt": self.create_excerpt(&post.content),
                "reading_time": self.calculate_reading_time(&post.content),
                "featured": post.front_matter.featured.unwrap_or(false),
                "permalink": format!("{}/posts/{}", self.config.site.base_url, post.slug)
            })
        }).collect()
    }

    fn get_popular_tags(&self) -> Vec<Value> {
        let mut tag_counts: HashMap<String, usize> = HashMap::new();

        for post in &self.posts {
            if let Some(tags) = &post.front_matter.tags {
                for tag in tags {
                    *tag_counts.entry(tag.clone()).or_insert(0) += 1;
                }
            }
        }

        let mut tags: Vec<_> = tag_counts.into_iter().collect();
        tags.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending

        tags.into_iter()
            .take(20) // Limit to top 20 tags
            .map(|(name, count)| {
                json!({
                    "name": name,
                    "count": count,
                    "slug": self.slugify(&name)
                })
            })
            .collect()
    }

    fn get_categories_tree(&self) -> Vec<Value> {
        let mut category_counts: HashMap<String, usize> = HashMap::new();

        for post in &self.posts {
            if let Some(categories) = &post.front_matter.categories {
                for category in categories {
                    *category_counts.entry(category.clone()).or_insert(0) += 1;
                }
            }
        }

        category_counts
            .into_iter()
            .map(|(name, count)| {
                json!({
                    "name": name,
                    "count": count,
                    "slug": self.slugify(&name),
                    "has_children": false,
                    "is_expanded": false,
                    "posts_count": count
                })
            })
            .collect()
    }

    fn get_recent_posts(&self, limit: usize) -> Vec<Value> {
        self.posts
            .iter()
            .take(limit)
            .map(|post| {
                json!({
                    "title": post.front_matter.title,
                    "slug": post.slug,
                    "date_short": self.format_date_short(&post.front_matter.date),
                    "date_iso": post.front_matter.date,
                    "reading_time": self.calculate_reading_time(&post.content)
                })
            })
            .collect()
    }

    fn get_site_stats(&self) -> Value {
        let total_words: usize = self
            .posts
            .iter()
            .map(|post| post.content.split_whitespace().count())
            .sum();

        // Исправлено: собираем категории в owned значения
        let all_categories: HashSet<String> = self
            .posts
            .iter()
            .filter_map(|post| post.front_matter.categories.as_ref())
            .flat_map(|categories| categories.iter().cloned())
            .collect();

        json!({
            "total_posts": self.posts.len(),
            "total_words": total_words,
            "total_categories": all_categories.len(),
            "last_updated": chrono::Utc::now().format("%Y-%m-%d").to_string()
        })
    }

    // Utility methods

    fn slugify(&self, text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| match c {
                'а'..='я' | 'ё' => c,
                'a'..='z' | '0'..='9' => c,
                _ => '-',
            })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn format_date(&self, date: &str) -> String {
        // Simple date formatting, can be enhanced
        date.to_string()
    }

    fn format_date_short(&self, date: &str) -> String {
        // Short date format
        if let Some(date_part) = date.split('T').next() {
            date_part.to_string()
        } else {
            date.to_string()
        }
    }

    fn calculate_reading_time(&self, content: &str) -> usize {
        let words = content.split_whitespace().count();
        (words / 200).max(1) // Assume 200 words per minute
    }

    fn create_excerpt(&self, content: &str) -> String {
        let words: Vec<&str> = content.split_whitespace().collect();
        if words.len() > 50 {
            format!("{}...", words[..50].join(" "))
        } else {
            words.join(" ")
        }
    }
}
