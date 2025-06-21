//! Zahuyach - static site generator library

pub mod cli;
pub mod commands;
pub mod config;
pub mod content;
pub mod error;
pub mod generator;
pub mod templates;

pub use error::Result;
