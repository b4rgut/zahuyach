use crate::error::Result;

pub fn run(name: String) -> Result<String> {
    // TODO: Implementation goes here
    Ok(format!("Initializing new blog project: {}", name))
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
