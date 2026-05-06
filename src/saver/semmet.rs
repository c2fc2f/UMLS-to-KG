//! Module of a Saver which saves every relation of semantic type from
//! metathesaurus

use std::path::Path;
use umls::metathesaurus::sty::models::SemanticTypeRecord;

use crate::{saver::Writer, writer};

/// Struct that regroups CSV Files for each relationship type
#[derive(Debug)]
pub struct SemTypeRelMetSaver {
    /// CSV Writer for HAS_SEMANTIC_TYPE Relation
    has_semantic_type: Writer,
}

impl SemTypeRelMetSaver {
    /// Init a saver which creates CSV files and writes headers
    pub async fn new(dir: &Path) -> std::io::Result<Self> {
        Ok(Self {
            has_semantic_type: writer!(
                dir,
                "HAS_SEMANTIC_TYPE",
                [
                    ":START_ID(UMLSMetathesaurus)",
                    "uniqueIdentifierAttribute",
                    ":END_ID(UMLSSemanticNetwork)"
                ]
            ),
        })
    }

    /// Flush every CSV file
    pub async fn flush(mut self) -> std::io::Result<()> {
        self.has_semantic_type.flush()?;

        Ok(())
    }

    /// Save one record
    pub async fn save_record(
        &mut self,
        rel: SemanticTypeRecord,
    ) -> std::io::Result<()> {
        self.has_semantic_type
            .write_record([&rel.cui, &rel.atui, &rel.tui])?;

        Ok(())
    }
}
