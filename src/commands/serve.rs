use crate::config::Config;
use crate::error::{Result, ZahuyachError};
use std::path::Path;

pub fn run(port: u16) -> Result<String> {
    // Проверяем, что мы в корне проекта (есть config.toml)
    let config_path = Path::new("config.toml");
    if !config_path.exists() {
        return Err(ZahuyachError::InvalidInput(
            "config.toml not found. Make sure you're in a Zahuyach project directory.".to_string(),
        ));
    }

    // Загружаем конфигурацию
    let config = Config::load(config_path)?;
    let output_dir = &config.build.output_dir;

    // Проверяем, что сайт собран
    if !Path::new(output_dir).exists() {
        return Err(ZahuyachError::InvalidInput(format!(
            "Output directory '{}' does not exist. Please run 'zahuyach build' first.",
            output_dir
        )));
    }

    println!("🚀 Server would start on http://127.0.0.1:{}", port);
    println!("📁 Serving files from: {}", output_dir);
    println!("🌍 Site title: {}", config.site.title);
    println!("\nThis is a simplified server implementation.");
    println!("To implement full functionality, you would need:");
    println!("1. HTTP server (warp, axum, etc.)");
    println!("2. Static file serving");
    println!("3. HTMX endpoints for search/filter");
    println!("4. Template rendering");
    println!("\nFor now, you can use any static file server like:");
    println!(
        "  python -m http.server {} --directory {}",
        port, output_dir
    );
    println!("  or");
    println!("  npx serve {} -p {}", output_dir, port);

    Ok(format!(
        "Development server info displayed for port {}",
        port
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_serve_command_without_config() {
        let result = run(3000);
        assert!(result.is_err());
    }

    #[rstest]
    #[case(3000)]
    #[case(8080)]
    fn test_serve_command_validation(#[case] port: u16) {
        let result = run(port);
        assert!(result.is_err());
    }
}
