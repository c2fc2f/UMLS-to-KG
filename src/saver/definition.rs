//! Module of a Saver which save every definitions

use std::path::Path;

use umls::definition::models::DefinitionRecord;

use crate::{saver::Writer, writer};

/// Struct that regroup CSV Files
#[derive(Debug)]
pub struct DefinitionSaver {
    /// CSV Writer for Definition Nodes
    definitions: Writer,

    /// CSV Writer for HAS_DEFINITION Relation
    has_definition: Writer,
}

impl DefinitionSaver {
    /// Init a saver which creates CSV file and writes header
    pub async fn new(dir: &Path) -> std::io::Result<Self> {
        Ok(Self {
            definitions: writer!(
                dir,
                "UMLSAttribute",
                [
                    "ui:ID(UMLSAttribute)",
                    "value",
                    "source",
                    "sourceAssertedAttributeIdentifier",
                ]
            ),
            has_definition: writer!(
                dir,
                "HAS_DEFINITION",
                [":START_ID(UMLS)", ":END_ID(UMLSAttribute)"]
            ),
        })
    }

    /// Flush every CSV file
    pub async fn flush(mut self) -> std::io::Result<()> {
        self.definitions.flush()?;
        self.has_definition.flush()?;

        Ok(())
    }

    /// Save one record
    pub async fn save_record(
        &mut self,
        def: DefinitionRecord,
    ) -> std::io::Result<()> {
        self.definitions.write_record([
            &def.atui,
            &def.definition,
            &def.sab,
            def.satui.as_deref().unwrap_or(""),
        ])?;
        self.has_definition.write_record([&def.aui, &def.atui])?;
        self.has_definition.write_record([&def.cui, &def.atui])?;

        Ok(())
    }
}
