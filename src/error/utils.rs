use std::path::PathBuf;

use crate::error::{Result, SpedError};

pub fn format_error(error: &SpedError) -> String {
    format!(
        "{}",
        match error {
            SpedError::Database { source, file, line } => {
                format!(
                    "Database error in {} at line {}: {}",
                    file.display(),
                    line,
                    source
                )
            }
            SpedError::FileNotFound(path) => {
                format!("File not found: {}", path.display())
            }
            SpedError::InvalidFormat { message, file, line } => {
                format!(
                    "Invalid format in {} at line {}: {}",
                    file.display(),
                    line,
                    message
                )
            }
            SpedError::Validation { message, file, line } => {
                format!(
                    "Validation error in {} at line {}: {}",
                    file.display(),
                    line,
                    message
                )
            }
            SpedError::Io(err) => {
                format!("IO error: {}", err)
            }
            SpedError::ParseError { source, file, line } => {
                format!(
                    "Parse error in {} at line {}: {}",
                    file.display(),
                    line,
                    source
                )
            }
            SpedError::Configuration(msg) => {
                format!("Configuration error: {}", msg)
            }
        }
    )
}

pub fn with_context<T, F>(
    result: Result<T>,
    file: impl Into<PathBuf>,
    line: usize,
) -> Result<T> {
    result.map_err(|err| match err {
        SpedError::Database { .. } => {
            SpedError::Database {
                source: err.source().map(|e| {
                    DieselError::DatabaseError(
                        diesel::result::DatabaseErrorKind::Unknown,
                        Box::new(e.to_string()),
                    )
                }).unwrap_or_else(|| {
                    DieselError::DatabaseError(
                        diesel::result::DatabaseErrorKind::Unknown,
                        Box::new("Unknown database error".to_string()),
                    )
                }),
                file: file.into(),
                line,
            }
        }
        SpedError::FileNotFound(path) => SpedError::FileNotFound(path),
        SpedError::InvalidFormat { message, .. } => {
            SpedError::InvalidFormat { message, file: file.into(), line }
        }
        SpedError::Validation { message, .. } => {
            SpedError::Validation { message, file: file.into(), line }
        }
        SpedError::Io(err) => SpedError::Io(err),
        SpedError::ParseError { source, .. } => {
            SpedError::ParseError { source, file: file.into(), line }
        }
        SpedError::Configuration(msg) => SpedError::Configuration(msg),
    })
}

pub fn map_diesel_error(err: diesel::result::Error, file: PathBuf, line: usize) -> SpedError {
    SpedError::Database {
        source: err,
        file,
        line,
    }
}

pub fn handle_file_not_found(path: PathBuf) -> SpedError {
    SpedError::FileNotFound(path)
}

pub fn handle_invalid_format(message: impl Into<String>, file: PathBuf, line: usize) -> SpedError {
    SpedError::invalid_format(message, file, line)
}

pub fn handle_validation_error(message: impl Into<String>, file: PathBuf, line: usize) -> SpedError {
    SpedError::validation(message, file, line)
}
