//! Error types for UMLS operations.
//!
//! This module defines [`UMLSError`], which encapsulates all possible
//! failures that can occur when parsing dataset information.

use std::io;

use thiserror::Error;

#[cfg(feature = "debug_path")]
/// Type alias for parsing errors with path tracking enabled.
type ParseError = serde_path_to_error::Error<csv_async::Error>;

#[cfg(not(feature = "debug_path"))]
/// Type alias for standard CSV parsing errors.
type ParseError = csv_async::Error;

/// Errors that can occur when interacting with the UMLS dataset.
#[derive(Error, Debug)]
pub enum UMLSError {
    /// An error occurred while parsing the CSV content.
    ///
    /// This variant wraps failures from the [`csv_async`] deserializer,
    /// such as malformed CSV syntax, unexpected fields, or data that
    /// does not conform to the expected UMLS schema.
    #[error("CSV parsing failed: {0}")]
    Parsing(#[from] ParseError),

    /// A low-level I/O error occurred during file operations.
    #[error("I/O error encountered: {0}")]
    IO(#[from] io::Error),
}
