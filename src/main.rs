use clap::Parser;
use std::process;
use zahuyach::cli::Cli;

fn main() {
    let cli = Cli::parse();

    match cli.run() {
        Ok(message) => {
            println!("{}", message);
        }
        Err(error) => {
            eprintln!("{}", error);
            self::process::exit(1);
        }
    }
}
