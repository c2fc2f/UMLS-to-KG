//! Module of a Saver which save every entity

use std::path::Path;

use fxhash::FxHashSet;
use umls::conso::models::{CoNSoRecord, StringType, TermStatus};

use crate::{saver::Writer, writer};

/// Struct that regroup CSV Files
#[derive(Debug)]
pub struct EntitySaver {
    /// CSV Writer for Concept Nodes
    concepts: Writer,
    /// CSV Writer for Lexical Nodes
    lexicals: Writer,
    /// CSV Writer for String Nodes
    string: Writer,
    /// CSV Writer for Atom Nodes
    atoms: Writer,

    /// CSV Writer for IS_ATOM_OF Relation
    is_atom_of: Writer,
    /// CSV Writer for IS_STRING_OF Relation
    is_string_of: Writer,
    /// CSV Writer for IS_LEXICAL_OF Relation
    is_lexical_of: Writer,

    /// Set of the ID of saved Concept node
    cuis: FxHashSet<String>,
    /// Set of the ID of saved Lexical node
    luis: FxHashSet<String>,
    /// Set of the ID of saved String node
    suis: FxHashSet<String>,
}

impl EntitySaver {
    /// Init a saver which creates CSV file and writes header
    pub async fn new(dir: &Path) -> std::io::Result<Self> {
        Ok(Self {
            concepts: writer!(dir, "UMLSConcept", ["ui:ID(UMLS)",]),
            lexicals: writer!(dir, "UMLSLexical", ["ui:ID(UMLS)",]),
            string: writer!(dir, "UMLSString", ["ui:ID(UMLS)",]),
            atoms: writer!(
                dir,
                "UMLSAtom",
                [
                    "ui:ID(UMLS)",
                    "value",
                    "source",
                    "sourceIdentifier",
                    "sourceAssertedAtomIdentifier",
                    "sourceAssertedConceptIdentifier",
                    "sourceAssertedDescriptorIdentifier"
                ]
            ),
            is_atom_of: writer!(
                dir,
                "IS_ATOM_OF",
                [":START_ID(UMLS)", "isPreferred:boolean", ":END_ID(UMLS)"]
            ),
            is_string_of: writer!(
                dir,
                "IS_STRING_OF",
                [":START_ID(UMLS)", "isPreferred:boolean", ":END_ID(UMLS)"]
            ),
            is_lexical_of: writer!(
                dir,
                "IS_LEXICAL_OF",
                [":START_ID(UMLS)", "isPreferred:boolean", ":END_ID(UMLS)"]
            ),
            cuis: FxHashSet::default(),
            luis: FxHashSet::default(),
            suis: FxHashSet::default(),
        })
    }

    /// Flush every CSV file
    pub async fn flush(mut self) -> std::io::Result<()> {
        self.concepts.flush()?;
        self.lexicals.flush()?;
        self.string.flush()?;
        self.atoms.flush()?;
        self.is_atom_of.flush()?;
        self.is_string_of.flush()?;
        self.is_lexical_of.flush()?;

        Ok(())
    }

    /// Save one record
    pub async fn save_record(
        &mut self,
        record: CoNSoRecord,
    ) -> std::io::Result<()> {
        self.is_atom_of.write_record([
            record.aui.as_str(),
            if matches!(record.stt, StringType::PreferredForm) {
                "true"
            } else {
                "false"
            },
            record.sui.as_str(),
        ])?;

        self.is_string_of.write_record([
            record.sui.as_str(),
            if matches!(record.ts, TermStatus::Preferred) {
                "true"
            } else {
                "false"
            },
            record.lui.as_str(),
        ])?;

        self.is_lexical_of.write_record([
            record.lui.as_str(),
            if record.is_pref { "true" } else { "false" },
            record.cui.as_str(),
        ])?;

        if !self.cuis.contains(&record.cui) {
            self.concepts.write_record([&record.cui])?;
            self.cuis.insert(record.cui);
        }

        if !self.luis.contains(&record.lui) {
            self.lexicals.write_record([&record.lui])?;
            self.luis.insert(record.lui);
        }

        if !self.suis.contains(&record.sui) {
            self.string.write_record([&record.sui])?;
            self.suis.insert(record.sui);
        }

        self.atoms.write_record([
            &record.aui,
            &record.string,
            &record.sab,
            &record.code,
            record.saui.as_deref().unwrap_or(""),
            record.scui.as_deref().unwrap_or(""),
            record.sdui.as_deref().unwrap_or(""),
        ])?;

        Ok(())
    }
}
