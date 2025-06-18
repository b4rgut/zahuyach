use clap::Parser;
use zahuyach::{Cli, Commands, commands};

fn main() -> zahuyach::error::Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { name } => commands::init::run(name)?,
        Commands::Build { dir } => commands::build::run(dir)?,
        Commands::Serve { port } => commands::serve::run(port)?,
    };

    println!("{}", result);

    Ok(())
}
