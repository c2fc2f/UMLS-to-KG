//! Async streaming interface for local
//! [UMLS](https://www.nlm.nih.gov/research/umls/index.html) (Unified Medical
//! Language System) datasets.
//!
//! This crate provides a zero-copy, lazily-evaluated reader for UMLS Rich
//! Release Format (`.RRF`) files. Rather than loading an entire file into
//! memory — which is impractical given the size of UMLS releases — records
//! are deserialized on demand via [`futures::Stream`], letting you filter,
//! map, or aggregate without ever materializing the full dataset.
//!
//! # Dataset layout
//!
//! UMLS distributes its data as a collection of pipe-delimited `.RRF` files,
//! conventionally stored under a `META/` and `NET/` directory after
//! extraction. Point [`UMLS::new`] at that directory:
//!
//! ```text
//! umls_2026/
//! ├── META/
//! │   ├── MRCONSO.RRF   ← concept names & source atoms
//! │   ├── MRDEF.RRF     ← definitions
//! │   ├── MRSTY.RRF     ← semantic type assignments
//! │   └── MRREL.RRF     ← inter-concept relationships
//! └── NET/
//!     ├── SRDEF         ← semantic types and relations
//!     └── SRSTRE1       ← set of relations
//! ```
//!
//! # Supported files
//!
//! | Method | File | Record type |
//! |---|---|---|
//! | [`UMLS::concept_names_and_sources`] | `MRCONSO.RRF` | [`metathesaurus::conso::models::CoNSoRecord`] |
//! | [`UMLS::definitions`] | `MRDEF.RRF` | [`metathesaurus::definition::models::DefinitionRecord`] |
//! | [`UMLS::semantic_types`] | `MRSTY.RRF` | [`metathesaurus::sty::models::SemanticTypeRecord`] |
//! | [`UMLS::related_concepts`] | `MRREL.RRF` | [`metathesaurus::rel::models::RelatedConceptRecord`] |
//! | [`UMLS::semantic_definitions`] | `SRDEF` | [`semantic_network::definition::models::SemanticDefinition`] |
//! | [`UMLS::semantic_types_relations`] | `SRSTRE1` | [`semantic_network::relation::models::SemanticTypeRelationship`] |
//!
//! # Quick start
//!
//! ```rust,no_run
//! use std::path::PathBuf;
//! use futures::StreamExt;
//! use umls::UMLS;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let db = UMLS::new(PathBuf::from("/data/umls"));
//!
//!     // Stream English concept names from MRCONSO.RRF
//!     let mut concepts = db.concept_names_and_sources();
//!     while let Some(record) = concepts.next().await {
//!         let record = record?;
//!         if record.lat == "ENG" {
//!             println!("{}: {}", record.cui, record.str);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Error handling
//!
//! Every stream item is a [`Result<T, UMLSError>`]. Errors fall into two
//! categories:
//!
//! - [`error::UMLSError::IO`] — the underlying file could not be opened or
//!   read.
//! - [`error::UMLSError::Parsing`] — a row could not be deserialized into the
//!   target record type (malformed field count, unexpected type, etc.).

pub mod error;
pub mod metathesaurus;
pub mod semantic_network;

use std::path::PathBuf;

use async_stream::try_stream;
use csv_async::AsyncReaderBuilder;
use futures::stream::{BoxStream, StreamExt};
use tokio::fs::File;

use crate::{
    error::UMLSError,
    metathesaurus::{
        conso::models::CoNSoRecord, definition::models::DefinitionRecord,
        rel::models::RelatedConceptRecord, sty::models::SemanticTypeRecord,
    },
    semantic_network::{
        definition::models::SemanticDefinition,
        relation::models::SemanticTypeRelationship,
    },
};

/// Represents a connection to a local UMLS dataset.
///
/// This structure holds the configuration required to locate and parse
/// UMLS data files (such as `.RRF` files) from the disk.
pub struct UMLS {
    /// The root directory containing the extracted UMLS data files.
    ///
    /// This path is used as the base for resolving specific dictionary files
    /// internally by the streaming modules.
    folder: PathBuf,
}

/// Generates a streaming method that lazily reads and deserializes records
/// from an RRF file in the UMLS dataset folder.
///
/// # Arguments
///
/// - `$method` — The name of the generated method (e.g., `conso`).
/// - `$record` — The record type to deserialize into (e.g., `CoNSoRecord`).
///   Must implement [`serde::Deserialize`].
/// - `$path` — The RRF path to read from (e.g., `"META/MRCONSO.RRF"`).
///
/// # Generated method signature
///
/// ```rust,ignore
/// pub fn $method(&self) -> BoxStream<'_, Result<$record, UMLSError>>
/// ```
///
/// # Example
///
/// ```rust,ignore
/// impl UMLSDataset {
///     rrf_stream_method!(conso, CoNSoRecord, "META/MRCONSO.RRF");
/// }
///
/// // Expands to:
/// // pub fn conso(&self) -> BoxStream<'_, Result<CoNSoRecord, UMLSError>> {
/// //     ...
/// // }
/// ```
macro_rules! rrf_stream_method {
    ($method:ident, $record:ty, $path:literal) => {
        #[doc = concat!(
            "Returns a stream of [`",
            stringify!($record),
            "`] entries from the `",
            $path,
            "` file.\n\n",
            "This allows for flexible, lazy processing of the dataset. Each item in ",
            "the stream is a [`Result`], ensuring that I/O or parsing errors ",
            "encountered during streaming are handled gracefully."
        )]
        pub fn $method(&self) -> BoxStream<'_, Result<$record, UMLSError>> {
            try_stream! {
                let path = self.folder.join($path);
                let file = File::open(&path).await.map_err(|e| {
                    UMLSError::IO {
                        file: $path,
                        source: e,
                    }
                })?;

                let mut reader = AsyncReaderBuilder::new()
                    .delimiter(b'|')
                    .has_headers(false)
                    .quoting(false)
                    .escape(None)
                    .create_deserializer(file);

                let mut stream = reader.deserialize::<$record>();

                while let Some(result) = stream.next().await {
                    let record = result.map_err(|e| {
                        UMLSError::Parsing {
                            file: $path,
                            source: e,
                        }
                    })?;
                    yield record;
                }
            }
            .boxed()
        }
    };
}

impl UMLS {
    /// Creates a new [`UMLS`] instance pointing to the specified directory.
    ///
    /// # Arguments
    ///
    /// * `folder` - A [`PathBuf`] representing the path to the UMLS dataset
    ///   directory.
    pub fn new(folder: PathBuf) -> Self {
        Self { folder }
    }

    rrf_stream_method!(
        concept_names_and_sources,
        CoNSoRecord,
        "META/MRCONSO.RRF"
    );
    rrf_stream_method!(definitions, DefinitionRecord, "META/MRDEF.RRF");
    rrf_stream_method!(semantic_types, SemanticTypeRecord, "META/MRSTY.RRF");
    rrf_stream_method!(
        related_concepts,
        RelatedConceptRecord,
        "META/MRREL.RRF"
    );

    rrf_stream_method!(semantic_definitions, SemanticDefinition, "NET/SRDEF");
    rrf_stream_method!(
        semantic_types_relations,
        SemanticTypeRelationship,
        "NET/SRSTRE1"
    );
}
