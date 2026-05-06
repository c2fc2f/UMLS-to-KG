//! Module of a Saver which saves every relation

use std::path::Path;
use umls::metathesaurus::rel::models::{RelatedConceptRecord, Relationship};

use crate::{saver::Writer, writer};

/// Struct that regroups CSV Files for each relationship type
#[derive(Debug)]
pub struct RelationSaver {
    /// CSV Writer for Allowed Qualifier (AQ)
    has_allowed_qualifier: Writer,
    /// CSV Writer for Child (CHD)
    child_of: Writer,
    /// CSV Writer for Deleted concept (DEL)
    deleted: Writer,
    /// CSV Writer for Parent (PAR)
    parent_of: Writer,
    /// CSV Writer for Qualified By (QB)
    qualified_by: Writer,
    /// CSV Writer for Broader relationship (RB)
    broader_than: Writer,
    /// CSV Writer for Similar relationship (RL)
    similar_to: Writer,
    /// CSV Writer for Narrower relationship (RN)
    narrower_than: Writer,
    /// CSV Writer for Other relationship (RO)
    has_other_relationship: Writer,
    /// CSV Writer for Related/Synonymous (RQ)
    possibly_synonym_of: Writer,
    /// CSV Writer for Related/Unspecified (RU)
    related_to: Writer,
    /// CSV Writer for Synonym (SY)
    synonym_of: Writer,
    /// CSV Writer for Not related (XR)
    not_related_to: Writer,
    /// CSV Writer for Empty/Unassigned relationship
    unassigned: Writer,
}

impl RelationSaver {
    /// Init a saver which creates CSV files and writes headers
    pub async fn new(dir: &Path) -> std::io::Result<Self> {
        let header = [
            ":START_ID(UMLSMetathesaurus)",
            "relationshipLabel",
            "source",
            "relationshipGroup",
            ":END_ID(UMLSMetathesaurus)",
        ];

        Ok(Self {
            has_allowed_qualifier: writer!(
                dir,
                "HAS_ALLOWED_QUALIFIER",
                header
            ),
            child_of: writer!(dir, "CHILD_OF", header),
            deleted: writer!(dir, "DELETED", header),
            parent_of: writer!(dir, "PARENT_OF", header),
            qualified_by: writer!(dir, "QUALIFIED_BY", header),
            broader_than: writer!(dir, "BROADER_THAN", header),
            similar_to: writer!(dir, "SIMILAR_TO", header),
            narrower_than: writer!(dir, "NARROWER_THAN", header),
            has_other_relationship: writer!(
                dir,
                "HAS_OTHER_RELATIONSHIP",
                header
            ),
            possibly_synonym_of: writer!(dir, "POSSIBLY_SYNONYM_OF", header),
            related_to: writer!(dir, "RELATED_TO", header),
            synonym_of: writer!(dir, "SYNONYM_OF", header),
            not_related_to: writer!(dir, "NOT_RELATED_TO", header),
            unassigned: writer!(dir, "UNASSIGNED", header),
        })
    }

    /// Flush every CSV file
    pub async fn flush(mut self) -> std::io::Result<()> {
        self.has_allowed_qualifier.flush()?;
        self.child_of.flush()?;
        self.deleted.flush()?;
        self.parent_of.flush()?;
        self.qualified_by.flush()?;
        self.broader_than.flush()?;
        self.similar_to.flush()?;
        self.narrower_than.flush()?;
        self.has_other_relationship.flush()?;
        self.possibly_synonym_of.flush()?;
        self.related_to.flush()?;
        self.synonym_of.flush()?;
        self.not_related_to.flush()?;
        self.unassigned.flush()?;

        Ok(())
    }

    /// Save one record
    pub async fn save_record(
        &mut self,
        rel: RelatedConceptRecord,
    ) -> std::io::Result<()> {
        let cui1: &str = match rel.cui1.as_deref() {
            None => return Ok(()),
            Some(cui1) => cui1,
        };

        let file: &mut Writer = match rel.rel {
            Relationship::AllowedQualifier => &mut self.has_allowed_qualifier,
            Relationship::Child => &mut self.child_of,
            Relationship::Deleted => &mut self.deleted,
            Relationship::Parent => &mut self.parent_of,
            Relationship::QualifiedBy => &mut self.qualified_by,
            Relationship::Broader => &mut self.broader_than,
            Relationship::Similar => &mut self.similar_to,
            Relationship::Narrower => &mut self.narrower_than,
            Relationship::Other => &mut self.has_other_relationship,
            Relationship::RelatedSynonymous => &mut self.possibly_synonym_of,
            Relationship::RelatedUnspecified => &mut self.related_to,
            Relationship::Synonym => &mut self.synonym_of,
            Relationship::NotRelated => &mut self.not_related_to,
            Relationship::Empty => &mut self.unassigned,
        };

        file.write_record([
            rel.aui1.as_deref().unwrap_or(cui1),
            rel.rela.as_deref().unwrap_or(""),
            &rel.sl,
            rel.rg.as_deref().unwrap_or(""),
            &rel.aui2.unwrap_or(rel.cui2),
        ])?;

        Ok(())
    }
}

