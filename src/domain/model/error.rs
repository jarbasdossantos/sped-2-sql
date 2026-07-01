use std::path::PathBuf;

use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpedError {
    #[error("Database error on line {line} in file {file}: {source}")]
    Database {
        source: DieselError,
        file: PathBuf,
        line: usize,
    },

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Invalid format at line {line} in file {file}: {message}")]
    InvalidFormat {
        message: String,
        file: PathBuf,
        line: usize,
    },

    #[error("Validation error at line {line} in file {file}: {message}")]
    Validation {
        message: String,
        file: PathBuf,
        line: usize,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse line {line} in file {file}: {source}")]
    ParseError {
        source: Box<dyn std::error::Error + Send + Sync>,
        file: PathBuf,
        line: usize,
    },

    #[error("Configuration error: {0}")]
    Configuration(String),
}

impl SpedError {
    pub fn database<E>(source: E, file: PathBuf, line: usize) -> Self
    where
        E: Into<DieselError>,
    {
        SpedError::Database {
            source: source.into(),
            file,
            line,
        }
    }

    pub fn invalid_format<M>(message: M, file: PathBuf, line: usize) -> Self
    where
        M: Into<String>,
    {
        SpedError::InvalidFormat {
            message: message.into(),
            file,
            line,
        }
    }

    pub fn validation<M>(message: M, file: PathBuf, line: usize) -> Self
    where
        M: Into<String>,
    {
        SpedError::Validation {
            message: message.into(),
            file,
            line,
        }
    }

    pub fn parse<E>(source: E, file: PathBuf, line: usize) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        SpedError::ParseError {
            source: Box::new(source),
            file,
            line,
        }
    }
}

pub type Result<T> = std::result::Result<T, SpedError>;

impl From<DieselError> for SpedError {
    fn from(err: DieselError) -> Self {
        SpedError::Database {
            source: err,
            file: PathBuf::from("unknown"),
            line: 0,
        }
    }
}