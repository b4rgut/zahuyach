use std::fmt;

#[derive(Debug)]
pub enum ZahuyachError {
    Io(std::io::Error),
    InvalidInput(String),
}

impl fmt::Display for ZahuyachError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZahuyachError::Io(err) => write!(f, "IO error: {}", err),
            ZahuyachError::InvalidInput(msg) => write!(f, "Error: Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for ZahuyachError {}

impl From<std::io::Error> for ZahuyachError {
    fn from(err: std::io::Error) -> Self {
        ZahuyachError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, ZahuyachError>;
