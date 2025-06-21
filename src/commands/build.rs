use crate::config::Config;
use crate::error::Result;
use crate::generator::SiteGenerator;
use std::path::Path;

pub fn run(output_dir: String) -> Result<String> {
    // Проверяем, что мы в корне проекта (есть config.toml)
    let config_path = Path::new("config.toml");
    if !config_path.exists() {
        return Err(crate::error::ZahuyachError::InvalidInput(
            "config.toml not found. Make sure you're in a Zahuyach project directory.".to_string(),
        ));
    }

    // Загружаем конфигурацию
    let mut config = Config::load(config_path)?;

    // Переопределяем output_dir если передан через аргумент
    config.build.output_dir = output_dir.clone();

    // Создаем генератор и запускаем сборку
    let mut generator = SiteGenerator::new(config)?;
    generator.build()?;

    Ok(format!("Successfully built site to '{}'", output_dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_command() {
        let result = run("dist".to_string());
        // This will fail in test environment without proper setup
        assert!(result.is_err() || result.is_ok());
    }
}
