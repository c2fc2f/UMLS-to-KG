//! Module of a Saver which saves every semantic type

use std::path::Path;

use umls::semantic_network::definition::models::{
    RecordType, SemanticDefinition,
};

use crate::{saver::Writer, writer};

/// Struct that regroups CSV Files for each relationship type
#[derive(Debug)]
pub struct SemTypeSaver {
    /// CSV Writer for SemanticType Nodes
    semtype: Writer,

    /// CSV Writer for SemanticRelation Nodes
    semrel: Writer,
}

impl SemTypeSaver {
    /// Init a saver which creates CSV files and writes headers
    pub async fn new(dir: &Path) -> std::io::Result<Self> {
        Ok(Self {
            semtype: writer!(
                dir,
                "UMLSSemanticType",
                [
                    "ui:ID(UMLSSemanticNetwork)",
                    "name",
                    "treeNumber",
                    "definition",
                    "abbreviation",
                    "usageNote",
                ]
            ),
            semrel: writer!(
                dir,
                "UMLSSemanticRelation",
                [
                    "ui:ID(UMLSSemanticNetwork)",
                    "name",
                    "treeNumber",
                    "definition",
                    "abbreviation",
                    "inverseOfRelation"
                ]
            ),
        })
    }

    /// Flush every CSV file
    pub async fn flush(mut self) -> std::io::Result<()> {
        self.semtype.flush()?;

        Ok(())
    }

    /// Save one record
    pub async fn save_record(
        &mut self,
        semtype: SemanticDefinition,
    ) -> std::io::Result<()> {
        if matches!(semtype.record_type, RecordType::Relation) {
            self.semrel.write_record([
                &semtype.ui,
                &semtype.name,
                &semtype.tree_number,
                &semtype.definition,
                &semtype.abbreviation,
                semtype.inverse_relation.as_deref().unwrap_or(""),
            ])?;

            return Ok(());
        }

        self.semtype.write_record([
            &semtype.ui,
            &semtype.name,
            &semtype.tree_number,
            &semtype.definition,
            &semtype.abbreviation,
            semtype.usage_note.as_deref().unwrap_or(""),
        ])?;

        Ok(())
    }
}
