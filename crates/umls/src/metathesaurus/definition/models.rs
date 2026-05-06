//! Module regrouping datatype

use serde::{Deserialize, Serialize};

use crate::metathesaurus::conso::models::SuppressStatus;

/// Represents a single record from the MRDEF.RRF file (Definitions).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct DefinitionRecord {
    /// Unique identifier for concept (CUI)
    pub cui: String,

    /// Unique identifier for atom - variable length field, 8 or 9 characters
    /// (AUI)
    pub aui: String,

    /// Unique identifier for attribut (ATUI)
    pub atui: String,

    /// Source asserted attribute identifier [optional] (SATUI)
    pub satui: Option<String>,

    /// Abbreviated source name (SAB). Max length 20 alphanumeric characters.
    pub sab: String,

    /// Definition (DEF)
    pub definition: String,

    /// Suppressible flag.
    pub suppress: SuppressStatus,
}
