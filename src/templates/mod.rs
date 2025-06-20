use crate::error::{Result, ZahuyachError};
use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "src/templates/basic"]
pub struct BasicTemplate;

/// Копирует базовый шаблон в указанную директорию
pub fn copy_basic_template<P: AsRef<Path>>(target_dir: P) -> Result<()> {
    let target_dir = target_dir.as_ref();

    // Создаем целевую директорию если она не существует
    fs::create_dir_all(target_dir).map_err(|e| ZahuyachError::Io(e))?;

    // Перебираем все встроенные файлы
    for file_path in BasicTemplate::iter() {
        let file_data = BasicTemplate::get(&file_path).ok_or_else(|| {
            ZahuyachError::InvalidInput(format!("Failed to get embedded file: {}", file_path))
        })?;

        // Создаем полный путь к файлу в целевой директории
        let target_file_path = target_dir.join(file_path.as_ref());

        // Создаем родительские директории если нужно
        if let Some(parent) = target_file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| ZahuyachError::Io(e))?;
        }

        // Записываем содержимое файла
        fs::write(&target_file_path, file_data.data).map_err(|e| ZahuyachError::Io(e))?;

        println!("Created: {}", target_file_path.display());
    }

    Ok(())
}

/// Возвращает список всех файлов в базовом шаблоне
pub fn list_basic_template_files() -> Vec<String> {
    BasicTemplate::iter().map(|f| f.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_list_basic_template_files() {
        let files = list_basic_template_files();
        assert!(!files.is_empty(), "Should have some template files");
        // Проверяем что есть config.toml
        assert!(files.iter().any(|f| f.ends_with("config.toml")));
    }

    #[test]
    fn test_copy_basic_template() {
        let temp_dir = TempDir::new().unwrap();
        let result = copy_basic_template(temp_dir.path());

        assert!(result.is_ok());

        // Проверяем что config.toml был создан
        let config_path = temp_dir.path().join("config.toml");
        assert!(config_path.exists());
    }
}
