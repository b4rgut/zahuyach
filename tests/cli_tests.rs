use clap::Parser;
use zahuyach::cli::{Cli, Commands};

#[test]
fn test_cli_init_command() {
    let args = vec!["zahuyach", "init", "my-blog"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(
        cli.get_command(),
        &Commands::Init {
            name: "my-blog".to_string()
        }
    );
}

#[test]
fn test_cli_build_command_default() {
    let args = vec!["zahuyach", "build"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(
        cli.get_command(),
        &Commands::Build {
            dir: "dist".to_string()
        }
    );
}

#[test]
fn test_cli_build_long_command_custom_output() {
    let args = vec!["zahuyach", "build", "--dir", "public"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(
        cli.get_command(),
        &Commands::Build {
            dir: "public".to_string()
        }
    );
}

#[test]
fn test_cli_build_short_command_custom_output() {
    let args = vec!["zahuyach", "build", "-d", "public"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(
        cli.get_command(),
        &Commands::Build {
            dir: "public".to_string()
        }
    );
}

#[test]
fn test_cli_serve_command_default_port() {
    let args = vec!["zahuyach", "serve"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(cli.get_command(), &Commands::Serve { port: 3000 });
}

#[test]
fn test_cli_serve_long_command_custom_port() {
    let args = vec!["zahuyach", "serve", "--port", "8080"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(cli.get_command(), &Commands::Serve { port: 8080 });
}

#[test]
fn test_cli_serve_short_command_custom_port() {
    let args = vec!["zahuyach", "serve", "-p", "8080"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(cli.get_command(), &Commands::Serve { port: 8080 });
}
