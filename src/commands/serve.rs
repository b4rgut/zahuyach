use crate::error::Result;

pub fn run(port: u16) -> Result<String> {
    // TODO: Implementation goes here
    Ok(format!("Serving blog project on port {}", port))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, "Serving blog project on port 1")]
    #[case(3000, "Serving blog project on port 3000")]
    #[case(8080, "Serving blog project on port 8080")]
    #[case(65535, "Serving blog project on port 65535")]
    fn test_serve_command(#[case] port: u16, #[case] expected: &str) {
        let result = run(port).unwrap();
        assert_eq!(result, expected);
    }
}
