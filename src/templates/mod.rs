use crate::error::{Result, ZahuyachError};
use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "src/templates/basic"]
pub struct BasicTemplate;

/// Copies the basic template to the specified directory
pub fn copy_basic_template<P: AsRef<Path>>(target_dir: P) -> Result<()> {
    let target_dir = target_dir.as_ref();

    // Create the target directory if it doesn't exist
    fs::create_dir_all(target_dir).map_err(|e| ZahuyachError::Io(e))?;

    // Iterate through all embedded files
    for file_path in BasicTemplate::iter() {
        let file_data = BasicTemplate::get(&file_path).ok_or_else(|| {
            ZahuyachError::InvalidInput(format!("Failed to get embedded file: {}", file_path))
        })?;

        // Create the full path to the file in the target directory
        let target_file_path = target_dir.join(file_path.as_ref());

        // Create parent directories if needed
        if let Some(parent) = target_file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| ZahuyachError::Io(e))?;
        }

        // Write the file content
        fs::write(&target_file_path, file_data.data).map_err(|e| ZahuyachError::Io(e))?;

        println!("Created: {}", target_file_path.display());
    }

    Ok(())
}

/// Returns a list of all files in the basic template
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
        // Check that config.toml exists
        assert!(files.iter().any(|f| f.ends_with("config.toml")));
    }

    #[test]
    fn test_copy_basic_template() {
        let temp_dir = TempDir::new().unwrap();
        let result = copy_basic_template(temp_dir.path());

        assert!(result.is_ok());

        // Check that config.toml was created
        let config_path = temp_dir.path().join("config.toml");
        assert!(config_path.exists());
    }
}
