//! Module regrouping datatype

use serde::{Deserialize, Serialize};

use crate::metathesaurus::conso::models::{
    SuppressStatus, deserialize_opt_yes_no,
};

/// Represents a single record from the MRREL.RRF file (Related Concepts).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct RelatedConceptRecord {
    /// Unique identifier of first concept (CUI1)
    pub cui1: Option<String>,

    /// Unique identifier of first atom (AUI1)
    pub aui1: Option<String>,

    /// The name of the column in MRCONSO.RRF that contains the identifier
    /// used for the first element (STYPE1).
    pub stype1: Option<IdentifierType>,

    /// Relationship of second concept or atom to first concept or atom (REL)
    pub rel: Relationship,

    /// Unique identifier of second concept (CUI2)
    pub cui2: String,

    /// Unique identifier of second atom (AUI2)
    pub aui2: Option<String>,

    /// The name of the column in MRCONSO.RRF that contains the identifier
    /// used for the second element (STYPE2)
    pub stype2: Option<IdentifierType>,

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

/// Relationship (REL)
///
/// Defines the relationship of the second concept (CUI2) to the first concept (CUI1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Relationship {
    /// Allowed qualifier
    #[serde(rename = "AQ")]
    AllowedQualifier,
    /// Has child relationship in a Metathesaurus source vocabulary
    #[serde(rename = "CHD")]
    Child,
    /// Deleted concept
    #[serde(rename = "DEL")]
    Deleted,
    /// Has parent relationship in a Metathesaurus source vocabulary
    #[serde(rename = "PAR")]
    Parent,
    /// Can be qualified by
    #[serde(rename = "QB")]
    QualifiedBy,
    /// Has a broader relationship
    #[serde(rename = "RB")]
    Broader,
    /// Relationship is similar or "alike"
    #[serde(rename = "RL")]
    Similar,
    /// Has a narrower relationship
    #[serde(rename = "RN")]
    Narrower,
    /// Relationship other than synonymous, narrower, or broader
    #[serde(rename = "RO")]
    Other,
    /// Related and possibly synonymous
    #[serde(rename = "RQ")]
    RelatedSynonymous,
    /// Related, unspecified
    #[serde(rename = "RU")]
    RelatedUnspecified,
    /// Source asserted synonymy
    #[serde(rename = "SY")]
    Synonym,
    /// Not related, no mapping
    #[serde(rename = "XR")]
    NotRelated,
    /// Empty relationship / Unassigned
    #[serde(rename = "")]
    Empty,
}
