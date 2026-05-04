//! Module regrouping datatype

use serde::{Deserialize, Serialize};

/// Represents a single record from the MRSTY.RRF file (Semantic Types).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SemanticTypeRecord {
    /// Unique identifier for concept (CUI)
    pub cui: String,

    /// Unique identifier of Semantic Type (TUI)
    pub tui: String,

    /// Semantic Type tree number (STN)
    pub stn: String,

    /// Semantic Type. The valid values are defined in the Semantic Network
    /// (STY)
    pub sty: String,

    /// Unique identifier for attribute (ATUI)
    pub atui: String,
}
