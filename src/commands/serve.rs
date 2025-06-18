use crate::error::Result;

pub fn run(port: u16) -> Result<String> {
    // TODO: Implementation goes here
    Ok(format!("Serving blog project on port {}", port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run(8080);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Serving blog project on port 8080");
    }
}
