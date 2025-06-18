use clap::Parser;
use zahuyach::cli::Cli;
fn main() {
    let cli = Cli::parse();
    println!("{}", cli.run().unwrap());
}
