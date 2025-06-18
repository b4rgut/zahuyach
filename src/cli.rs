//! CLI module for the Zahuyach static site generator.
//!
//! This module defines the command-line interface structure and handles
//! argument parsing using the clap library.

use crate::{commands, error::Result};
use clap::{Parser, Subcommand};

/// Available commands for the CLI application.
///
/// Each variant represents a different operation that can be performed
/// by the static site generator.
#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Initialize a new blog project
    ///
    /// Creates a new blog project with the specified name, setting up
    /// the necessary directory structure and configuration files.
    Init {
        /// Project name
        ///
        /// The name of the blog project to create. This will be used
        /// as the directory name and in configuration.
        name: String,
    },
    /// Build a blog project
    ///
    /// Processes Markdown files and generates static HTML files
    /// ready for deployment.
    Build {
        /// Output directory
        ///
        /// The directory where the generated static files will be placed.
        /// Defaults to "dist" if not specified.
        #[arg(short, long, default_value = "dist")]
        dir: String,
    },
    /// Serve a blog project
    ///
    /// Starts a local development server to preview the generated site.
    /// Useful for testing and development.
    Serve {
        /// Port to serve on
        ///
        /// The port number for the local development server.
        /// Defaults to 3000 if not specified.
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
}

/// Main CLI structure for the Zahuyach static site generator.
///
/// This struct represents the parsed command-line arguments and provides
/// methods to execute the requested operations.
#[derive(Parser, Debug, PartialEq)]
#[command(name = "zahuyach")]
#[command(about = "A static site generator for blogs")]
#[command(version)]
pub struct Cli {
    /// The subcommand to execute
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    /// Executes the parsed command.
    ///
    /// This method matches the parsed command and delegates execution
    /// to the appropriate command module. It returns a string result
    /// that can be displayed to the user.
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` with a success message on successful execution,
    /// or `Err(ZahuyachError)` if an error occurs during command execution.
    pub fn run(self) -> Result<String> {
        match self.command {
            Commands::Init { name } => commands::init::run(name),
            Commands::Build { dir } => commands::build::run(dir),
            Commands::Serve { port } => commands::serve::run(port),
        }
    }

    /// Returns a reference to the parsed command.
    ///
    /// This method provides access to the command that was parsed from
    /// the command-line arguments. Useful for testing and introspection.
    pub fn get_command(&self) -> &Commands {
        &self.command
    }
}
