use crate::error::{Result, ZahuyachError};

pub fn run(name: String) -> Result<String> {
    match validate_project_name(&name) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            return Err(err);
        }
    }

    // TODO: Implementation goes here
    Ok(format!("Initializing new blog project: {}", name))
}

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

    #[test]
    fn test_init_command() {
        let result = run("my-blog".to_string()).unwrap();
        assert_eq!(result, "Initializing new blog project: my-blog");
    }
}
