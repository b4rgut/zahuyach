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

        self.generate_about_page()?;
        println!("✅ About page generated");

        if self.config.is_tags_enabled() {
            self.generate_tags_pages()?;
            println!("✅ Tags pages generated");
        }

        if self.config.is_categories_enabled() {
            self.generate_categories_pages()?;
            println!("✅ Categories pages generated");
        }

        if self.config.is_rss_enabled() {
            self.generate_rss_feed()?;
            println!("✅ RSS feed generated");
        }

        // 9. Генерация страницы 404
        self.generate_404_page()?;
        println!("✅ 404 page generated");

        // 10. Копирование статических файлов
        self.copy_static_files()?;
        println!("✅ Static files copied");

        println!("🎉 Site generation completed successfully!");
        Ok(())
    }

    fn clean_output_dir(&self) -> Result<()> {
        let output_path = Path::new(&self.config.build.output_dir);

        if output_path.exists() && self.config.should_clean_output() {
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
                if self.config.include_drafts() || !post.is_draft() {
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

        // Создаем директорию posts если она не существует
        let posts_dir = output_dir.join("posts");
        fs::create_dir_all(&posts_dir)?;

        for post in &self.posts {
            let post_data = self.create_post_data(post);
            let html = self.handlebars.render("post", &post_data).map_err(|e| {
                ZahuyachError::InvalidInput(format!("Template render error: {}", e))
            })?;

            let post_path = posts_dir.join(format!("{}.html", post.slug));
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
            "posts": self.get_posts_list_context(posts_refs, self.config.get_posts_per_page()),
            "popular_tags": self.get_popular_tags(),
            "categories": self.get_categories_tree(),
            "recent_posts": self.get_recent_posts(self.config.get_recent_posts_limit()),
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

        // Convert posts_by_date to a more template-friendly format
        let mut posts_by_year: Vec<Value> = Vec::new();

        let mut years: Vec<_> = posts_by_date.keys().cloned().collect();
        years.sort_by(|a, b| b.cmp(a)); // Sort years descending

        for year in years {
            let months = posts_by_date.get(&year).unwrap();
            let mut months_list: Vec<_> = months.keys().cloned().collect();
            months_list.sort_by(|a, b| b.cmp(a)); // Sort months descending

            let mut year_posts: Vec<Value> = Vec::new();

            for month in months_list {
                let month_posts = months.get(&month).unwrap();
                for post in month_posts {
                    year_posts.push(json!({
                        "title": post.front_matter.title,
                        "url": format!("/posts/{}", post.slug),
                        "date": self.format_date_short(&post.front_matter.date),
                        "date_iso": post.front_matter.date,
                        "tags": post.front_matter.tags.as_ref().unwrap_or(&vec![])
                    }));
                }
            }

            posts_by_year.push(json!({
                "year": year,
                "posts": year_posts
            }));
        }

        let context = json!({
            "site": self.get_site_context(),
            "posts_by_year": posts_by_year,
            "posts_by_date": posts_by_date, // Keep for backward compatibility
            "total_posts": self.posts.len(),
            "categories": self.get_categories_tree(),
            "popular_tags": self.get_popular_tags(),
            "recent_posts": self.get_recent_posts(self.config.get_recent_posts_limit()),
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
            fs::write(output_dir.join(self.config.get_rss_filename()), rss_content)?;
            return Ok(());
        }

        let rss_posts: Vec<Value> = self.posts.iter()
                .take(self.config.get_rss_limit())
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

        fs::write(output_dir.join(self.config.get_rss_filename()), rss_xml)?;

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

        for post in self.posts.iter().take(self.config.get_rss_limit()) {
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

    fn generate_about_page(&self) -> Result<()> {
        let output_dir = Path::new(&self.config.build.output_dir);

        // Проверяем наличие шаблона about
        if !self.has_template("about") {
            println!("⚠️  Template 'about' not found, skipping about page generation");
            return Ok(());
        }

        let context = json!({
            "site": self.get_site_context(),
            "recent_posts": self.get_recent_posts(self.config.get_recent_posts_limit()),
            "popular_tags": self.get_popular_tags(),
            "page": {
                "title": "About",
                "description": "About this blog",
                "url": "/about"
            },
            "is_about": true
        });

        let html = self.handlebars.render("about", &context).map_err(|e| {
            ZahuyachError::InvalidInput(format!("About template render error: {}", e))
        })?;

        fs::write(output_dir.join("about.html"), html)?;

        Ok(())
    }

    fn generate_404_page(&self) -> Result<()> {
        let output_dir = Path::new(&self.config.build.output_dir);

        // Проверяем наличие шаблона 404
        if !self.has_template("404") {
            println!("⚠️  Template '404' not found, skipping 404 page generation");
            return Ok(());
        }

        let context = json!({
            "site": self.get_site_context(),
            "recent_posts": self.get_recent_posts(self.config.get_recent_posts_limit()),
            "popular_tags": self.get_popular_tags(),
            "page": {
                "title": "404 - Page Not Found",
                "description": "The page you're looking for doesn't exist",
                "url": "/404.html"
            }
        });

        let html = self.handlebars.render("404", &context).map_err(|e| {
            ZahuyachError::InvalidInput(format!("404 template render error: {}", e))
        })?;

        fs::write(output_dir.join("404.html"), html)?;

        Ok(())
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
                "date": self.format_date(&post.front_matter.date),
                "date_raw": post.front_matter.date,
                "date_formatted": self.format_date(&post.front_matter.date),
                "date_short": self.format_date_short(&post.front_matter.date),
                "date_iso": post.front_matter.date,
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
            "recent_posts": self.get_recent_posts(self.config.get_recent_posts_limit()),
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
            "current_year": chrono::Utc::now().year(),
            "email": self.config.site.email.as_ref().unwrap_or(&String::new()),
            "timezone": self.config.site.timezone.as_ref().unwrap_or(&"UTC".to_string()),
            "social": self.config.site.social.as_ref().map(|s| json!({
                "github": s.github.as_ref().unwrap_or(&String::new()),
                "twitter": s.twitter.as_ref().unwrap_or(&String::new()),
                "linkedin": s.linkedin.as_ref().unwrap_or(&String::new()),
                "email": s.email.as_ref().or(self.config.site.email.as_ref()).unwrap_or(&String::new()),
                "mastodon": s.mastodon.as_ref().unwrap_or(&String::new()),
                "youtube": s.youtube.as_ref().unwrap_or(&String::new()),
                "instagram": s.instagram.as_ref().unwrap_or(&String::new()),
                "facebook": s.facebook.as_ref().unwrap_or(&String::new())
            })).unwrap_or_else(|| json!({
                "github": "",
                "twitter": "",
                "linkedin": "",
                "email": self.config.site.email.as_ref().unwrap_or(&String::new()),
                "mastodon": "",
                "youtube": "",
                "instagram": "",
                "facebook": ""
            }))
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
                "url": format!("/posts/{}", post.slug),
                "date": self.format_date(&post.front_matter.date),
                "date_raw": post.front_matter.date,
                "date_formatted": self.format_date(&post.front_matter.date),
                "date_short": self.format_date_short(&post.front_matter.date),
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
            .filter(|(_, count)| *count >= self.config.get_min_tag_count())
            .take(self.config.get_max_tags_in_cloud())
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
                    "url": format!("/posts/{}", post.slug),
                    "date": self.format_date(&post.front_matter.date),
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
        use chrono::{DateTime, NaiveDateTime};

        // Parse the date string
        let parsed = if let Ok(dt) = DateTime::parse_from_rfc3339(date) {
            dt.naive_local()
        } else if let Ok(dt) = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S") {
            dt
        } else if let Ok(dt) = NaiveDateTime::parse_from_str(date, "%Y-%m-%d") {
            dt
        } else {
            // If parsing fails, return original string
            return date.to_string();
        };

        // Get format from config or use default
        let format = self
            .config
            .date_format
            .as_ref()
            .and_then(|df| df.posts.as_deref())
            .unwrap_or("%B %d, %Y");

        parsed.format(format).to_string()
    }

    fn format_date_short(&self, date: &str) -> String {
        use chrono::{DateTime, NaiveDateTime};

        // Parse the date string
        let parsed = if let Ok(dt) = DateTime::parse_from_rfc3339(date) {
            dt.naive_local()
        } else if let Ok(dt) = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S") {
            dt
        } else if let Ok(dt) = NaiveDateTime::parse_from_str(date, "%Y-%m-%d") {
            dt
        } else {
            // If parsing fails, try simple split
            if let Some(date_part) = date.split('T').next() {
                return date_part.to_string();
            }
            return date.to_string();
        };

        // Get format from config or use default
        let format = self
            .config
            .date_format
            .as_ref()
            .and_then(|df| df.archive.as_deref())
            .unwrap_or("%b %d");

        parsed.format(format).to_string()
    }

    fn calculate_reading_time(&self, content: &str) -> usize {
        let words = content.split_whitespace().count();
        (words / self.config.get_reading_speed()).max(1)
    }

    fn create_excerpt(&self, content: &str) -> String {
        use pulldown_cmark::{Parser, html};

        // Разделяем контент на строки
        let lines: Vec<&str> = content.lines().collect();

        // Ищем маркер excerpt или берем первые несколько абзацев
        let excerpt_separator = self.config.get_excerpt_separator();
        let excerpt_content = if let Some(more_index) = lines
            .iter()
            .position(|line| line.trim() == excerpt_separator)
        {
            lines[..more_index].join("\n")
        } else {
            // Берем первые N непустых абзаца
            let mut paragraphs = Vec::new();
            let mut current_paragraph = Vec::new();

            for line in lines.iter() {
                if line.trim().is_empty() {
                    if !current_paragraph.is_empty() {
                        paragraphs.push(current_paragraph.join("\n"));
                        current_paragraph.clear();
                    }
                } else {
                    current_paragraph.push(*line);
                }

                if paragraphs.len() >= self.config.get_excerpt_length() {
                    break;
                }
            }

            if !current_paragraph.is_empty() && paragraphs.len() < self.config.get_excerpt_length()
            {
                paragraphs.push(current_paragraph.join("\n"));
            }

            paragraphs.join("\n\n")
        };

        // Конвертируем Markdown в HTML
        let parser = Parser::new(&excerpt_content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }
}
