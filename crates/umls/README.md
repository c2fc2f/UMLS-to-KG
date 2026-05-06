# umls

An async, streaming Rust library for reading local [UMLS](https://www.nlm.nih.gov/research/umls/index.html) (Unified Medical Language System) datasets.

Rather than loading multi-gigabyte `.RRF` files into memory, `umls` deserializes records on demand via [`futures::Stream`], letting you filter, map, and aggregate without ever materializing the full dataset.

## Features

- **Lazy streaming** — records are read and deserialized one at a time, keeping memory usage flat regardless of file size
- **Strongly typed** — all fields are mapped to idiomatic Rust types; enums cover the full UMLS vocabularies for languages, term types, relationship identifiers, suppress flags, and more
- **Tokio-native** — built on `tokio::fs` and `csv-async` for non-blocking I/O throughout
- **Optional error paths** — enable the `debug_path` feature to get structured `serde_path_to_error` output that pinpoints exactly which field failed to parse

## Supported files

| Method | File | Record type |
|---|---|---|
| `concept_names_and_sources` | `MRCONSO.RRF` | `CoNSoRecord` |
| `definitions` | `MRDEF.RRF` | `DefinitionRecord` |
| `semantic_types` | `MRSTY.RRF` | `SemanticTypeRecord` |
| `related_concepts` | `MRREL.RRF` | `RelatedConceptRecord` |
| `semantic_definitions` | `SRDEF` | `SemanticDefinition` |
| `semantic_types_relations` | `SRSTRE1` | `SemanticTypeRelationship` |

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
umls = { path = "..." }
```

To get structured parse error paths (useful during development):

```toml
[dependencies]
umls = { path = "...", features = ["debug_path"] }
```

## Dataset layout

After extracting a UMLS release, point the library at the `2026AA/` directory:

```text
2026AA/
umls_2026/
├── META/
│   ├── MRCONSO.RRF   ← concept names & source atoms
│   ├── MRDEF.RRF     ← definitions
│   ├── MRSTY.RRF     ← semantic type assignments
│   └── MRREL.RRF     ← inter-concept relationships
└── NET/
    ├── SRDEF         ← semantic types and relations
    └── SRSTRE1       ← set of relations
```

Access to UMLS data requires a [UMLS Metathesaurus License](https://www.nlm.nih.gov/research/umls/licensing_authentication/index.html).

## Usage

### Stream all English concept names

```rust
use std::path::PathBuf;
use futures::StreamExt;
use umls::UMLS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = UMLS::new(PathBuf::from("/data/umls"));

    let mut concepts = db.concept_names_and_sources();
    while let Some(record) = concepts.next().await {
        let record = record?;
        if record.lat == umls::conso::models::LanguageOfTerms::Eng {
            println!("{}: {}", record.cui, record.string);
        }
    }

    Ok(())
}
```

### Collect semantic types for a specific concept

```rust
use futures::StreamExt;
use umls::UMLS;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = UMLS::new(PathBuf::from("/data/umls"));
    let target = "C0009450"; // COVID-19

    let mut stream = db.semantic_types();
    while let Some(record) = stream.next().await {
        let record = record?;
        if record.cui == target {
            println!("{}: {}", record.tui, record.sty);
        }
    }

    Ok(())
}
```

### Run the bundled example

```bash
cargo run --example simple -- --folder /data/umls
```

## Error handling

Every stream item is a `Result<T, UMLSError>`. Two variants are possible:

- `UMLSError::IO` — the file could not be opened or read from disk
- `UMLSError::Parsing` — a row could not be deserialized (wrong field count, unexpected enum value, etc.)

With the `debug_path` feature enabled, `UMLSError::Parsing` wraps a `serde_path_to_error::Error` that includes the exact field path that caused the failure.

## Requirements

- A Tokio async runtime (the `#[tokio::main]` macro or a manually constructed `tokio::runtime::Runtime`)
