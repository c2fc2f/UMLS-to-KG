//! Module of a Saver which save every entity

use std::{path::Path, sync::Arc};

use fxhash::FxHashSet;
use umls::metathesaurus::conso::models::{CoNSoRecord, StringType, TermStatus};

use crate::{saver::Writer, writer};

/// Struct that regroup CSV Files
#[derive(Debug)]
pub struct EntitySaver {
    /// CSV Writer for Concept Nodes
    concepts: Writer,
    /// CSV Writer for Lexical Nodes
    lexicals: Writer,
    /// CSV Writer for String Nodes
    strings: Writer,
    /// CSV Writer for Atom Nodes
    atoms: Writer,

    /// CSV Writer for IS_ATOM_OF Relation
    is_atom_of: Writer,
    /// CSV Writer for IS_STRING_OF Relation
    is_string_of: Writer,
    /// CSV Writer for IS_LEXICAL_OF Relation
    is_lexical_of: Writer,

    /// Set of the ID of saved Concept node
    cuis: FxHashSet<Arc<String>>,
    /// Set of the ID of saved Lexical node
    luis: FxHashSet<Arc<String>>,
    /// Set of the ID of saved String node
    suis: FxHashSet<Arc<String>>,
    /// Set of the pair ID of saved IS_STRING_OF relation
    string_ofs: FxHashSet<(Arc<String>, Arc<String>)>,
    /// Set of the pair ID of saved IS_LEXICAL_OF relation
    lexical_ofs: FxHashSet<(Arc<String>, Arc<String>)>,
}

impl EntitySaver {
    /// Init a saver which creates CSV file and writes header
    pub async fn new(dir: &Path) -> std::io::Result<Self> {
        Ok(Self {
            concepts: writer!(
                dir,
                "UMLSConcept",
                ["ui:ID(UMLSMetathesaurus)",]
            ),
            lexicals: writer!(
                dir,
                "UMLSLexical",
                ["ui:ID(UMLSMetathesaurus)",]
            ),
            strings: writer!(dir, "UMLSString", ["ui:ID(UMLSMetathesaurus)",]),
            atoms: writer!(
                dir,
                "UMLSAtom",
                [
                    "ui:ID(UMLSMetathesaurus)",
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
                [
                    ":START_ID(UMLSMetathesaurus)",
                    "isPreferred:boolean",
                    ":END_ID(UMLSMetathesaurus)"
                ]
            ),
            is_string_of: writer!(
                dir,
                "IS_STRING_OF",
                [
                    ":START_ID(UMLSMetathesaurus)",
                    "isPreferred:boolean",
                    ":END_ID(UMLSMetathesaurus)"
                ]
            ),
            is_lexical_of: writer!(
                dir,
                "IS_LEXICAL_OF",
                [
                    ":START_ID(UMLSMetathesaurus)",
                    "isPreferred:boolean",
                    ":END_ID(UMLSMetathesaurus)"
                ]
            ),
            cuis: FxHashSet::default(),
            luis: FxHashSet::default(),
            suis: FxHashSet::default(),
            string_ofs: FxHashSet::default(),
            lexical_ofs: FxHashSet::default(),
        })
    }

    /// Flush every CSV file
    pub async fn flush(mut self) -> std::io::Result<()> {
        self.concepts.flush()?;
        self.lexicals.flush()?;
        self.strings.flush()?;
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
        let sui: Arc<String> = Arc::new(record.sui);
        let lui: Arc<String> = Arc::new(record.lui);
        let cui: Arc<String> = Arc::new(record.cui);

        self.is_atom_of.write_record([
            record.aui.as_str(),
            if matches!(record.stt, StringType::PreferredForm) {
                "true"
            } else {
                "false"
            },
            sui.as_str(),
        ])?;

        let e: (Arc<String>, Arc<String>) =
            (Arc::clone(&sui), Arc::clone(&lui));

        if !self.string_ofs.contains(&e) {
            self.is_string_of.write_record([
                sui.as_str(),
                if matches!(record.ts, TermStatus::Preferred) {
                    "true"
                } else {
                    "false"
                },
                lui.as_str(),
            ])?;
            self.string_ofs.insert(e);
        }

        let e: (Arc<String>, Arc<String>) =
            (Arc::clone(&lui), Arc::clone(&cui));

        if !self.lexical_ofs.contains(&e) {
            self.is_lexical_of.write_record([
                lui.as_str(),
                if record.is_pref { "true" } else { "false" },
                cui.as_str(),
            ])?;
            self.lexical_ofs.insert(e);
        }

        if !self.cuis.contains(&cui) {
            self.concepts.write_record([cui.as_str()])?;
            self.cuis.insert(cui);
        }

        if !self.luis.contains(&lui) {
            self.lexicals.write_record([lui.as_str()])?;
            self.luis.insert(lui);
        }

        if !self.suis.contains(&sui) {
            self.strings.write_record([sui.as_str()])?;
            self.suis.insert(sui);
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
