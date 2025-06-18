//! Zahuyach - static site generator library

pub mod cli;
pub mod commands;
pub mod error;

pub use cli::{Cli, Commands};
pub use error::Result;
