use clap::{Parser, Subcommand};

#[derive(Parser, Debug, PartialEq)]
#[command(name = "zahuyach")]
#[command(about = "A static site generator for blogs")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Initialize a new blog project
    Init {
        /// Project name
        #[arg(short, long)]
        name: String,
    },
    /// Build a blog project
    Build {
        /// Output directory
        #[arg(short, long, default_value = "dist")]
        output: String,
    },
    /// Serve a blog project
    Serve {
        /// Port to serve on
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
}

pub fn handle_command(command: Commands) -> String {
    match command {
        Commands::Init { name } => {
            format!("Initializing new blog project: {}", name)
        }
        Commands::Build { output } => {
            format!("Building blog project: {}", output)
        }
        Commands::Serve { port } => {
            format!("Serving blog project on port {}", port)
        }
    }
}

fn main() {
    let cli = Cli::parse();
    println!("{}", handle_command(cli.command));
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_init_command() {
        let args = vec!["zahuyach", "init", "--name", "my-blog"];
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
                output: "dist".to_string()
            }
        );
    }

    #[test]
    fn test_cli_build_command_custom_output() {
        let args = vec!["zahuyach", "build", "--output", "public"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(
            cli.command,
            Commands::Build {
                output: "public".to_string()
            }
        );
    }

    #[test]
    fn test_cli_serve_command_custom_port() {
        let args = vec!["zahuyach", "serve", "--port", "8080"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.command, Commands::Serve { port: 8080 });
    }

    #[test]
    fn test_cli_serve_command_default_port() {
        let args = vec!["zahuyach", "serve"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.command, Commands::Serve { port: 3000 });
    }

    #[test]
    fn test_handle_init_command() {
        let command = Commands::Init {
            name: "my-blog".to_string(),
        };

        let result = handle_command(command);

        assert_eq!(result, "Initializing new blog project: my-blog");
    }

    #[test]
    fn test_handle_build_command_default() {
        let command = Commands::Build {
            output: "dist".to_string(),
        };

        let result = handle_command(command);

        assert_eq!(result, "Building blog project: dist");
    }

    #[test]
    fn test_handle_build_command_custom_output() {
        let command = Commands::Build {
            output: "public".to_string(),
        };

        let result = handle_command(command);

        assert_eq!(result, "Building blog project: public");
    }

    #[test]
    fn test_handle_serve_command_default_port() {
        let command = Commands::Serve { port: 3000 };

        let result = handle_command(command);

        assert_eq!(result, "Serving blog project on port 3000");
    }

    #[test]
    fn test_handle_serve_command_custom_port() {
        let command = Commands::Serve { port: 8080 };

        let result = handle_command(command);

        assert_eq!(result, "Serving blog project on port 8080");
    }
}
