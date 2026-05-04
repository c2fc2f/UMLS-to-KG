//! Module regrouping datatype

use serde::{Deserialize, Serialize};

use crate::conso::models::{SuppressStatus, deserialize_opt_yes_no};

/// Represents a single record from the MRREL.RRF file (Related Concepts).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct RelatedConceptRecord {
    /// Unique identifier of first concept (CUI1)
    pub cui1: String,

    /// Unique identifier of first atom (AUI1)
    pub aui1: String,

    /// The name of the column in MRCONSO.RRF that contains the identifier
    /// used for the first element (STYPE1).
    pub stype1: IdentifierType,

    /// Relationship of second concept or atom to first concept or atom (REL)
    pub rel: String,

    /// Unique identifier of second concept (CUI2)
    pub cui2: String,

    /// Unique identifier of second atom (AUI2)
    pub aui2: String,

    /// The name of the column in MRCONSO.RRF that contains the identifier
    /// used for the second element (STYPE2)
    pub stype2: IdentifierType,

    /// Additional (more specific) relationship label (RELA)
    pub rela: Option<String>,

    /// Unique identifier of relationship (RUI)
    pub rui: String,

    /// Source asserted relationship identifier (SRUI)
    pub srui: Option<String>,

    /// Abbreviated source name (SAB). Max length 20 alphanumeric characters.
    pub sab: String,

    /// Source of relationship labels (SL)
    pub sl: String,

    /// Relationship group. Used to indicate that a set of relationships
    /// should be looked at in conjunction (RG)
    pub rg: Option<String>,

    /// Source asserted directionality flag (DIR)
    /// None = not important/determined.
    #[serde(deserialize_with = "deserialize_opt_yes_no")]
    pub dir: Option<bool>,

    /// Suppressible flag.
    pub suppress: SuppressStatus,
}

/// Identifier Type (STYPE)
///
/// Indicates the name of the column in the source files (like MRCONSO.RRF)
/// that contains the identifier used for the element in the relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum IdentifierType {
    /// Atom Unique Identifier
    Aui,
    /// Concept Unique Identifier
    Cui,
    /// Code identifier
    Code,
    /// Source Concept Unique Identifier
    Scui,
    /// Source Descriptor Unique Identifier
    Sdui,
}
