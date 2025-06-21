use crate::error::{Result, ZahuyachError};
use crate::templates;
use std::{env, fs, path::PathBuf};

/// Runs the init command to create a new blog project.
///
/// If no project name is provided, uses the current directory name.
/// Asks for user confirmation before creating the project.
pub fn run(name: Option<&String>) -> Result<String> {
    let project_path = determine_project_path(name)?;

    // Extract project name for validation and display
    let project_name = project_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            ZahuyachError::InvalidInput("Cannot determine project name from path".to_string())
        })?;

    validate_project_name(&project_name)?;

    // Check if directory exists and is not empty (if we're creating a subdirectory)
    if name.is_some() {
        if project_path.exists() {
            let is_empty = fs::read_dir(&project_path)
                .map_err(|e| ZahuyachError::Io(e))?
                .next()
                .is_none();

            if !is_empty {
                return Err(ZahuyachError::InvalidInput(format!(
                    "Directory '{}' already exists and is not empty",
                    project_path.display()
                )));
            }
        }
    } else {
        // Using current directory - check if it's empty
        let is_empty = fs::read_dir(&project_path)
            .map_err(|e| ZahuyachError::Io(e))?
            .next()
            .is_none();

        if !is_empty {
            return Err(ZahuyachError::InvalidInput(
                    "Current directory is not empty. Please run in an empty directory or specify a project name.".to_string()
                ));
        }
    }

    // Copy the basic template to the project directory
    templates::copy_basic_template(&project_path)?;

    Ok(format!(
        "Successfully initialized new blog project: {} in directory {}",
        project_name,
        project_path.display()
    ))
}

/// Determines the full project path based on user input or current directory.
///
/// If name is provided, returns current_dir + name
/// If name is None, returns current_dir
fn determine_project_path(name: Option<&String>) -> Result<PathBuf> {
    let current_dir = env::current_dir().map_err(|e| ZahuyachError::Io(e))?;

    match name {
        Some(name) => Ok(current_dir.join(name)),
        None => Ok(current_dir),
    }
}

/// Validates the project name to ensure it's suitable for a directory name.
fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(ZahuyachError::InvalidInput(
            "Project name cannot be empty".to_string(),
        ));
    }

    let invalid_chars: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    if name.contains(invalid_chars) {
        return Err(ZahuyachError::InvalidInput(format!(
            "Project name cannot contain invalid characters '{}'",
            invalid_chars.iter().collect::<String>()
        )));
    }

    if name.starts_with('.') || name.starts_with('-') {
        return Err(ZahuyachError::InvalidInput(
            "Project name cannot start with a dot '.' or hyphen '-'.".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("my-blog", true)]
    #[case("blog123", true)]
    #[case("my_blog", true)]
    #[case("", false)]
    #[case(".hidden", false)]
    #[case("-invalid", false)]
    #[case("with/slash", false)]
    #[case("with\\backslash", false)]
    #[case("with:colon", false)]
    #[case("with*asterisk", false)]
    #[case("with?question", false)]
    #[case("with\"quote", false)]
    #[case("with<bracket", false)]
    #[case("with>bracket", false)]
    #[case("with|pipe", false)]
    fn test_validate_project_name(#[case] name: &str, #[case] should_be_valid: bool) {
        let result = validate_project_name(name);

        if should_be_valid {
            assert!(result.is_ok(), "Expected '{}' to be valid", name);
        } else {
            assert!(result.is_err(), "Expected '{}' to be invalid", name);
        }
    }

    #[rstest]
    #[case(Some("test-blog".to_string()), "test-blog")]
    fn test_determine_project_path_with_input(
        #[case] input: Option<String>,
        #[case] expected_name: &str,
    ) {
        let result = determine_project_path(input.as_ref()).unwrap();
        let current_dir = std::env::current_dir().unwrap();
        let expected_path = current_dir.join(expected_name);
        assert_eq!(result, expected_path);
    }

    #[test]
    fn test_determine_project_path_without_input() {
        let expected = std::env::current_dir().unwrap();
        let result = determine_project_path(None).unwrap();
        assert_eq!(result, expected);
    }
}
