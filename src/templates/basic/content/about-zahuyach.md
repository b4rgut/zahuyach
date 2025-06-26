---
title: "О генераторе Zahuyach"
date: "2024-01-20"
author: "Разработчик"
tags: ["zahuyach", "rust", "ssg"]
categories: ["технологии"]
description: "Рассказ о том, как создавался генератор статических сайтов Zahuyach"
---

# Zahuyach - генератор статических сайтов

Zahuyach - это быстрый и мощный генератор статических сайтов, написанный на Rust.

## Ключевые особенности

1. **Высокая производительность** благодаря Rust
2. **GitHub Dark тема** из коробки
3. **HTMX интеграция** для динамических элементов
4. **Простая конфигурация** через TOML
5. **Markdown поддержка** с расширениями

## Архитектура

```rust
pub struct SiteGenerator {
    config: Config,
    handlebars: Handlebars<'static>,
    posts: Vec<Post>,
}
```
