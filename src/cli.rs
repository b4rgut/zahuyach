use clap::{Parser, Subcommand};

#[derive(Parser, Debug, PartialEq)]
#[command(name = "zahuyach")]
#[command(about = "A static site generator for blogs")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Initialize a new blog project
    Init {
        /// Project name
        name: String,
    },
    /// Build a blog project
    Build {
        /// Output directory
        #[arg(short, long, default_value = "dist")]
        dir: String,
    },
    /// Serve a blog project
    Serve {
        /// Port to serve on
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
}
