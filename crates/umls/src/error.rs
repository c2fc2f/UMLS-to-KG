//! Error types for UMLS operations.
//!
//! This module defines [`UMLSError`], which encapsulates all possible
//! failures that can occur when parsing dataset information.

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
    #[error("CSV parsing failed in {file}: {source}")]
    Parsing {
        /// File where the error occurred
        file: &'static str,

        /// Source error
        #[source]
        source: ParseError,
    },

    /// A low-level I/O error occurred during file operations.
    #[error("I/O error encountered in {file}: {source}")]
    IO {
        /// File where the error occurred
        file: &'static str,

        /// Source error
        #[source]
        source: std::io::Error,
    },
}
