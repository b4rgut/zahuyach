use crate::error::Result;

pub fn run(dir: String) -> Result<String> {
    // TODO: Implementation goes here
    Ok(format!("Building blog project: {}", dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_command() {
        let result = run("dist".to_string()).unwrap();
        assert_eq!(result, "Building blog project: dist");
    }

    #[test]
    fn test_build_command_custom_output() {
        let result = run("public".to_string()).unwrap();
        assert_eq!(result, "Building blog project: public");
    }
}
