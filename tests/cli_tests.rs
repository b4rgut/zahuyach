use clap::Parser;
use zahuyach::{Cli, Commands, commands};

#[test]
fn test_cli_init_command() {
    let args = vec!["zahuyach", "init", "my-blog"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(
        cli.command,
        Commands::Init {
            name: "my-blog".to_string()
        }
    );
}

#[test]
fn test_cli_build_command_default() {
    let args = vec!["zahuyach", "build"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(
        cli.command,
        Commands::Build {
            dir: "dist".to_string()
        }
    );
}

#[test]
fn test_cli_build_long_command_custom_output() {
    let args = vec!["zahuyach", "build", "--dir", "public"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(
        cli.command,
        Commands::Build {
            dir: "public".to_string()
        }
    );
}

#[test]
fn test_cli_build_short_command_custom_output() {
    let args = vec!["zahuyach", "build", "-d", "public"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(
        cli.command,
        Commands::Build {
            dir: "public".to_string()
        }
    );
}

#[test]
fn test_cli_serve_command_default_port() {
    let args = vec!["zahuyach", "serve"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(cli.command, Commands::Serve { port: 3000 });
}

#[test]
fn test_cli_serve_long_command_custom_port() {
    let args = vec!["zahuyach", "serve", "--port", "8080"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(cli.command, Commands::Serve { port: 8080 });
}

#[test]
fn test_cli_serve_short_command_custom_port() {
    let args = vec!["zahuyach", "serve", "-p", "8080"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert_eq!(cli.command, Commands::Serve { port: 8080 });
}

#[test]
fn test_handle_init_command_execution() {
    let result = commands::init::run("my-blog".to_string()).unwrap();
    assert_eq!(result, "Initializing new blog project: my-blog");
}

#[test]
fn test_handle_build_command_execution_default() {
    let result = commands::build::run("dist".to_string()).unwrap();
    assert_eq!(result, "Building blog project: dist");
}

#[test]
fn test_handle_build_command_execution_custom_output() {
    let result = commands::build::run("public".to_string()).unwrap();
    assert_eq!(result, "Building blog project: public");
}

#[test]
fn test_handle_serve_command_execution_default_port() {
    let result = commands::serve::run(3000).unwrap();
    assert_eq!(result, "Serving blog project on port 3000");
}

#[test]
fn test_handle_serve_command_execution_custom_port() {
    let result = commands::serve::run(8080).unwrap();
    assert_eq!(result, "Serving blog project on port 8080");
}
