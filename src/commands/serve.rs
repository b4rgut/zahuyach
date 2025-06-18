use crate::error::Result;

pub fn run(port: u16) -> Result<String> {
    // TODO: Implementation goes here
    Ok(format!("Serving blog project on port {}", port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serve_command_default_port() {
        let result = run(3000).unwrap();
        assert_eq!(result, "Serving blog project on port 3000");
    }

    #[test]
    fn test_serve_command_custom_port() {
        let result = run(8080).unwrap();
        assert_eq!(result, "Serving blog project on port 8080");
    }
}
