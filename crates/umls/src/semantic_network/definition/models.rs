//! Module regrouping datatype

use serde::{Deserialize, Deserializer, Serialize};

/// Semantic Network Definition (SRDEF.RRF)
///
/// This struct defines the semantic types and relations that make up the UMLS
/// Semantic Network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticDefinition {
    /// RT: Record Type (STY = Semantic Type or RL = Relation)
    #[serde(rename = "RT")]
    pub record_type: RecordType,
    /// UI: Unique Identifier of the Semantic Type or Relation
    #[serde(rename = "UI")]
    pub ui: String,
    /// STY/RL: Name of the Semantic Type or Relation
    #[serde(rename = "STY/RL")]
    pub name: String,
    /// STN/RTN: Tree Number of the Semantic Type or Relation
    #[serde(rename = "STN/RTN")]
    pub tree_number: String,
    /// DEF: Definition of the Semantic Type or Relation
    #[serde(rename = "DEF")]
    pub definition: String,
    /// EX: Examples of Metathesaurus concepts with this Semantic Type
    /// (STY records only)
    #[serde(rename = "EX")]
    pub examples: Option<String>,
    /// UN: Usage note for Semantic Type assignment (STY records only)
    #[serde(rename = "UN", deserialize_with = "deserialize_usage_note")]
    pub usage_note: Option<String>,
    /// NH: The Semantic Type and its descendants allow the non-human flag
    /// (STY records only)
    #[serde(rename = "NH")]
    pub allow_non_human: Option<String>,
    /// ABR: Abbreviation of the Relation Name or Semantic Type
    #[serde(rename = "ABR")]
    pub abbreviation: String,
    /// RIN: Inverse of the Relation (RL records only)
    #[serde(rename = "RIN")]
    pub inverse_relation: Option<String>,
}

/// Record Type (RT)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordType {
    /// Semantic Type
    #[serde(rename = "STY")]
    SemanticType,
    /// Relation
    #[serde(rename = "RL")]
    Relation,
}

/// Custom deserializer to handle "NULL" strings as logical `None` values.
///
/// # Arguments
/// * `deserializer` - The Serde deserializer instance.
///
/// # Returns
/// * `Ok(None)` if the value is not here or the string "NULL".
/// * `Ok(Some(String))` for any other string value.
///
/// # Errors
/// * `Err` if the input cannot be deserialized as an optional string.
pub fn deserialize_usage_note<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.filter(|s| s != "NULL"))
}
