use clap::Parser;
use rstest::rstest;
use zahuyach::cli::{Cli, Commands};

#[rstest]
#[case(vec!["zahuyach", "init", "my-blog"], Commands::Init { name: Some("my-blog".to_string()) })]
#[case(vec!["zahuyach", "init"], Commands::Init { name: None })]
fn test_cli_init_command(#[case] args: Vec<&str>, #[case] expected: Commands) {
    let cli = Cli::try_parse_from(args).unwrap();
    assert_eq!(cli.get_command(), &expected);
}

#[rstest]
#[case(
    vec!["zahuyach", "build"],
    Commands::Build { dir: "dist".to_string() },
    "default output directory"
)]
#[case(
    vec!["zahuyach", "build", "--dir", "public"],
    Commands::Build { dir: "public".to_string() },
    "long flag custom output directory"
)]
#[case(
    vec!["zahuyach", "build", "-d", "public"],
    Commands::Build { dir: "public".to_string() },
    "short flag custom output directory"
)]
#[case(
    vec!["zahuyach", "build", "--dir", "output"],
    Commands::Build { dir: "output".to_string() },
    "long flag different custom directory"
)]
fn test_cli_build_command(
    #[case] args: Vec<&str>,
    #[case] expected: Commands,
    #[case] description: &str,
) {
    let cli = Cli::try_parse_from(args).unwrap();
    assert_eq!(cli.get_command(), &expected, "Failed for: {}", description);
}

#[rstest]
#[case(
    vec!["zahuyach", "serve"],
    Commands::Serve { port: 3000 },
    "default port"
)]
#[case(
    vec!["zahuyach", "serve", "--port", "8080"],
    Commands::Serve { port: 8080 },
    "long flag custom port"
)]
#[case(
    vec!["zahuyach", "serve", "-p", "8080"],
    Commands::Serve { port: 8080 },
    "short flag custom port"
)]
#[case(
    vec!["zahuyach", "serve", "--port", "4000"],
    Commands::Serve { port: 4000 },
    "long flag different custom port"
)]
#[case(
    vec!["zahuyach", "serve", "-p", "9000"],
    Commands::Serve { port: 9000 },
    "short flag different custom port"
)]
fn test_cli_serve_command(
    #[case] args: Vec<&str>,
    #[case] expected: Commands,
    #[case] description: &str,
) {
    let cli = Cli::try_parse_from(args).unwrap();
    assert_eq!(cli.get_command(), &expected, "Failed for: {}", description);
}

#[rstest]
#[case(vec!["zahuyach"], "missing command")]
#[case(vec!["zahuyach", "invalid"], "invalid command")]
#[case(vec!["zahuyach", "serve", "--port", "invalid"], "invalid port number")]
#[case(vec!["zahuyach", "serve", "-p", "70000"], "port number out of range")]
fn test_cli_invalid_arguments(#[case] args: Vec<&str>, #[case] description: &str) {
    let result = Cli::try_parse_from(args);
    assert!(result.is_err(), "Expected error for: {}", description);
}
