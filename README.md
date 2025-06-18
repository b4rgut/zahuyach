# Zahuyach

**🇺🇸 English | [🇷🇺 Русский](README_RU.md)**

A static site generator for blogs written in Rust.

## Description

Zahuyach is a simple and fast static site generator designed for bloggers who write in Obsidian and want to host their content on GitHub Pages. It focuses on simplicity and performance while providing essential features for modern blogging.

## Features

- Markdown parsing and HTML generation
- CLI interface for easy usage
- Local development server
- Project scaffolding
- Designed for Obsidian compatibility
- GitHub Pages ready output

## Installation

```bash
cargo install zahuyach
```

## Usage

### Initialize a new project
```bash
zahuyach init
```

### Build your site
```bash
zahuyach build
```

### Serve locally for development
```bash
zahuyach serve
```

### Additional Options

#### Build command
```bash
# Specify output directory
zahuyach build --dir public
zahuyach build -d public
```

#### Serve command
```bash
# Specify port
zahuyach serve --port 8080
zahuyach serve -p 8080
```

## Project Structure

After running `zahuyach init` the following structure is created:

```
my-blog/
├── content/          # Markdown files
├── templates/        # HTML templates
├── static/           # Static files (CSS, JS, images)
└── config.toml       # Site configuration
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
