use crate::error::{Result, ZahuyachError};
use std::env;

/// Runs the init command to create a new blog project.
///
/// If no project name is provided, uses the current directory name.
/// Asks for user confirmation before creating the project.
pub fn run(name: Option<String>) -> Result<String> {
    let project_name = determine_project_name(name)?;

    match validate_project_name(&project_name) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            return Err(err);
        }
    }

    // TODO: Implementation goes here
    Ok(format!("Initializing new blog project: {}", project_name))
}

/// Determines the project name based on user input or current directory.
fn determine_project_name(name: Option<String>) -> Result<String> {
    match name {
        Some(name) => Ok(name),
        None => {
            let current_dir = env::current_dir().map_err(|e| ZahuyachError::Io(e))?;

            let dir_name = current_dir
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| {
                    ZahuyachError::InvalidInput(
                        "Cannot determine current directory name".to_string(),
                    )
                })?;

            Ok(dir_name.to_string())
        }
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
    fn test_determine_project_name_with_input(
        #[case] input: Option<String>,
        #[case] expected: &str,
    ) {
        let result = determine_project_name(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    fn test_determine_project_name_without_input() {
        let expected = std::env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let result = determine_project_name(None).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_valid_project_name() {
        let result = run(Some("my-blog".to_string()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Initializing new blog project: my-blog");
    }
}
