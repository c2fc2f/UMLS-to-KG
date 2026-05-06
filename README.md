# UMLS-to-KG

A command-line tool written in Rust that reads a local [UMLS (Unified Medical Language System)](https://www.nlm.nih.gov/research/umls/index.html) release and converts it into a set of CSV files ready for bulk import into a [Neo4j](https://neo4j.com/) knowledge graph.

## Overview

UMLS distributes its data as large pipe-delimited `.RRF` files spread across two sub-datasets: the **Metathesaurus** (concepts, atoms, strings, definitions, and inter-concept relationships) and the **Semantic Network** (semantic types, semantic relations, and their hierarchical structure). This tool streams both datasets — without loading them fully into memory — and writes Neo4j-compatible CSV node and relationship files.

The project is organized as a Cargo workspace with one library crate:

- **`crates/umls`** — async, streaming Rust client for local UMLS `.RRF` files

The `umls2kg` binary ties this crate together and writes the final CSV output.

> **License requirement** — access to UMLS data requires a [UMLS Metathesaurus License](https://www.nlm.nih.gov/research/umls/index.html) from the NLM. The tool operates on a locally extracted UMLS release; it does not download data automatically.

## Requirements

- Rust toolchain (edition 2024, stable)
- A locally extracted UMLS release (see [Dataset layout](#dataset-layout) below)
- Sufficient disk space for the output CSVs (~4Go)

## Installation

### From source

```bash
git clone https://github.com/c2fc2f/UMLS-to-KG
cd UMLS-to-KG
cargo build --release
# or
cargo install --git https://github.com/c2fc2f/UMLS-to-KG
```

The compiled binary will be at `target/release/umls2kg`.

### With Nix

A Nix flake is provided:

```bash
nix run github:c2fc2f/UMLS-to-KG -- --help
# or
nix build
# or, to enter a development shell:
nix develop
```

## Dataset layout

After extracting a UMLS release, the tool expects the following directory structure. Pass the root directory (e.g. `umls_2026/`) to `--umls`:

```
umls_2026/
├── META/
│   ├── MRCONSO.RRF   ← concept names & source atoms
│   ├── MRDEF.RRF     ← definitions
│   ├── MRSTY.RRF     ← semantic type assignments
│   └── MRREL.RRF     ← inter-concept relationships
└── NET/
    ├── SRDEF         ← semantic type and relation definitions
    └── SRSTRE1       ← semantic network relationships
```

## Usage

```
umls2kg [OPTIONS] --umls <PATH>
```

| Flag | Short | Description | Default |
|---|---|---|---|
| `--umls <PATH>` | `-u` | Path to the extracted UMLS release directory | *(required)* |
| `--output <DIR>` | `-o` | Directory where CSV files are written | `.` (current directory) |

### Examples

Process a UMLS release and write CSVs to `./out`:

```bash
umls2kg --umls /data/umls_2026 --output ./out
```

## Output: Knowledge Graph Schema

All output files are written to the directory specified by `--output`. Node files and relationship files are formatted for [Neo4j's bulk CSV importer](https://neo4j.com/docs/operations-manual/current/import/).

### Nodes

The graph is split across two ID spaces: `UMLSMetathesaurus` (concepts, atoms, strings, lexical forms, definitions) and `UMLSSemanticNetwork` (semantic types and relations).

| File | Labels | Description |
|---|---|---|
| `UMLSConcept.csv` | `UMLS`, `UMLSMetathesaurus`, `UMLSConcept` | Concept unique identifiers (CUIs) |
| `UMLSLexical.csv` | `UMLS`, `UMLSMetathesaurus`, `UMLSLexical` | Lexical unique identifiers (LUIs) |
| `UMLSString.csv` | `UMLS`, `UMLSMetathesaurus`, `UMLSString` | String unique identifiers (SUIs) |
| `UMLSAtom.csv` | `UMLS`, `UMLSMetathesaurus`, `UMLSAtom` | Atom records (AUIs) with source string, vocabulary, and source codes |
| `UMLSDefinition.csv` | `UMLS`, `UMLSMetathesaurus`, `UMLSAttribute`, `UMLSDefinition` | Source-asserted definitions for concepts and atoms |
| `UMLSSemanticType.csv` | `UMLS`, `UMLSSemanticNetwork`, `UMLSSemanticType` | Semantic type nodes from the SRDEF file |
| `UMLSSemanticRelation.csv` | `UMLS`, `UMLSSemanticNetwork`, `UMLSSemanticRelation` | Semantic relation nodes from the SRDEF file |

### Relationships

#### Metathesaurus — structural hierarchy

These relationships model the four-level atom hierarchy defined in MRCONSO and MRDEF.

| File | Type | From → To | Description |
|---|---|---|---|
| `IS_ATOM_OF.csv` | `IS_ATOM_OF` | UMLSAtom → UMLSString | Links an atom to its string form |
| `IS_STRING_OF.csv` | `IS_STRING_OF` | UMLSString → UMLSLexical | Links a string to its lexical identifier |
| `IS_LEXICAL_OF.csv` | `IS_LEXICAL_OF` | UMLSLexical → UMLSConcept | Links a lexical form to its concept |
| `HAS_DEFINITION.csv` | `HAS_DEFINITION` | UMLSConcept / UMLSAtom → UMLSDefinition | Links a concept or atom to its definitions |

All three structural relationship files carry an `isPreferred:boolean` property indicating whether the link represents the preferred form at that level.

#### Metathesaurus — inter-concept relationships (MRREL)

Each UMLS relationship code is mapped to a dedicated file.

| File | Type | UMLS Code | Description |
|---|---|---|---|
| `HAS_ALLOWED_QUALIFIER.csv` | `HAS_ALLOWED_QUALIFIER` | `AQ` | Allowed qualifier |
| `CHILD_OF.csv` | `CHILD_OF` | `CHD` | Child relationship |
| `PARENT_OF.csv` | `PARENT_OF` | `PAR` | Parent relationship |
| `BROADER_THAN.csv` | `BROADER_THAN` | `RB` | Broader relationship |
| `NARROWER_THAN.csv` | `NARROWER_THAN` | `RN` | Narrower relationship |
| `SYNONYM_OF.csv` | `SYNONYM_OF` | `SY` | Synonym |
| `POSSIBLY_SYNONYM_OF.csv` | `POSSIBLY_SYNONYM_OF` | `RQ` | Related and possibly synonymous |
| `SIMILAR_TO.csv` | `SIMILAR_TO` | `RL` | Similar |
| `RELATED_TO.csv` | `RELATED_TO` | `RU` | Related, unspecified |
| `HAS_OTHER_RELATIONSHIP.csv` | `HAS_OTHER_RELATIONSHIP` | `RO` | Other relationship |
| `QUALIFIED_BY.csv` | `QUALIFIED_BY` | `QB` | Qualified by |
| `NOT_RELATED_TO.csv` | `NOT_RELATED_TO` | `XR` | Not related |
| `DELETED.csv` | `DELETED` | `DEL` | Deleted concept |
| `UNASSIGNED.csv` | `UNASSIGNED` | *(empty)* | Unassigned relationship type |

All inter-concept relationship files carry `relationshipLabel`, `source`, and `relationshipGroup` properties.

#### Metathesaurus → Semantic Network

| File | Type | From → To | Description |
|---|---|---|---|
| `HAS_SEMANTIC_TYPE.csv` | `HAS_SEMANTIC_TYPE` | UMLSConcept → UMLSSemanticType | Assigns a semantic type to a concept (from MRSTY) |

#### Semantic Network relationships (SRSTRE1)

All semantic network relationship files use the `SN-` prefix and connect `UMLSSemanticType` or `UMLSSemanticRelation` nodes to one another.

| File | Type | File | Type |
|---|---|---|---|
| `SN-IS_A.csv` | `IS_A` | `SN-CAUSES.csv` | `CAUSES` |
| `SN-PART_OF.csv` | `PART_OF` | `SN-PRODUCES.csv` | `PRODUCES` |
| `SN-CONCEPTUAL_PART_OF.csv` | `CONCEPTUAL_PART_OF` | `SN-AFFECTS.csv` | `AFFECTS` |
| `SN-PHYSICALLY_RELATED_TO.csv` | `PHYSICALLY_RELATED_TO` | `SN-DISRUPTS.csv` | `DISRUPTS` |
| `SN-SPATIALLY_RELATED_TO.csv` | `SPATIALLY_RELATED_TO` | `SN-PREVENTS.csv` | `PREVENTS` |
| `SN-TEMPORALLY_RELATED_TO.csv` | `TEMPORALLY_RELATED_TO` | `SN-TREATS.csv` | `TREATS` |
| `SN-FUNCTIONALLY_RELATED_TO.csv` | `FUNCTIONALLY_RELATED_TO` | `SN-MANAGES.csv` | `MANAGES` |
| `SN-CONCEPTUALLY_RELATED_TO.csv` | `CONCEPTUALLY_RELATED_TO` | `SN-COMPLICATES.csv` | `COMPLICATES` |
| `SN-ASSOCIATED_WITH.csv` | `ASSOCIATED_WITH` | `SN-MANIFESTATION_OF.csv` | `MANIFESTATION_OF` |
| `SN-INTERACTS_WITH.csv` | `INTERACTS_WITH` | `SN-DIAGNOSES.csv` | `DIAGNOSES` |
| `SN-CO_OCCURS_WITH.csv` | `CO_OCCURS_WITH` | `SN-INDICATES.csv` | `INDICATES` |
| `SN-PRECEDES.csv` | `PRECEDES` | `SN-ASSESSES_EFFECT_OF.csv` | `ASSESSES_EFFECT_OF` |
| `SN-LOCATION_OF.csv` | `LOCATION_OF` | `SN-MEASURES.csv` | `MEASURES` |
| `SN-CONTAINS.csv` | `CONTAINS` | `SN-MEASUREMENT_OF.csv` | `MEASUREMENT_OF` |
| `SN-CONNECTED_TO.csv` | `CONNECTED_TO` | `SN-EVALUATION_OF.csv` | `EVALUATION_OF` |
| `SN-ADJACENT_TO.csv` | `ADJACENT_TO` | `SN-PERFORMS.csv` | `PERFORMS` |
| `SN-INTERCONNECTS.csv` | `INTERCONNECTS` | `SN-CARRIES_OUT.csv` | `CARRIES_OUT` |
| `SN-SURROUNDS.csv` | `SURROUNDS` | `SN-PRACTICES.csv` | `PRACTICES` |
| `SN-TRAVERSES.csv` | `TRAVERSES` | `SN-USES.csv` | `USES` |
| `SN-BRANCH_OF.csv` | `BRANCH_OF` | `SN-PROCESS_OF.csv` | `PROCESS_OF` |
| `SN-TRIBUTARY_OF.csv` | `TRIBUTARY_OF` | `SN-RESULT_OF.csv` | `RESULT_OF` |
| `SN-CONSISTS_OF.csv` | `CONSISTS_OF` | `SN-BRINGS_ABOUT.csv` | `BRINGS_ABOUT` |
| `SN-INGREDIENT_OF.csv` | `INGREDIENT_OF` | `SN-OCCURS_IN.csv` | `OCCURS_IN` |
| `SN-DERIVATIVE_OF.csv` | `DERIVATIVE_OF` | `SN-EXHIBITS.csv` | `EXHIBITS` |
| `SN-DEVELOPMENTAL_FORM_OF.csv` | `DEVELOPMENTAL_FORM_OF` | `SN-ANALYZES.csv` | `ANALYZES` |
| `SN-DEGREE_OF.csv` | `DEGREE_OF` | `SN-ISSUE_IN.csv` | `ISSUE_IN` |
| `SN-PROPERTY_OF.csv` | `PROPERTY_OF` | `SN-METHOD_OF.csv` | `METHOD_OF` |

## Importing into Neo4j

Once `umls2kg` has finished writing the CSV files, use `neo4j-admin database import full` to bulk-load them into Neo4j. The command below assumes all CSV files are in the current directory and targets the default `neo4j` database.

> The database must be stopped before running an import. The `--overwrite-destination` flag will erase any existing data in the target database.

```bash
sudo JDK_JAVA_OPTIONS="--add-opens=java.base/java.nio=ALL-UNNAMED --add-opens=java.base/java.lang=ALL-UNNAMED" \
  neo4j-admin database import full neo4j \
    --verbose \
    --overwrite-destination \
    --nodes=UMLS:UMLSMetathesaurus:UMLSConcept=./UMLSConcept.csv \
    --nodes=UMLS:UMLSMetathesaurus:UMLSLexical=./UMLSLexical.csv \
    --nodes=UMLS:UMLSMetathesaurus:UMLSString=./UMLSString.csv \
    --nodes=UMLS:UMLSMetathesaurus:UMLSAtom=./UMLSAtom.csv \
    --nodes=UMLS:UMLSMetathesaurus:UMLSAttribute:UMLSDefinition=./UMLSDefinition.csv \
    --nodes=UMLS:UMLSSemanticNetwork:UMLSSemanticType=./UMLSSemanticType.csv \
    --nodes=UMLS:UMLSSemanticNetwork:UMLSSemanticRelation=./UMLSSemanticRelation.csv \
    --relationships=IS_ATOM_OF=./IS_ATOM_OF.csv \
    --relationships=IS_STRING_OF=./IS_STRING_OF.csv \
    --relationships=IS_LEXICAL_OF=./IS_LEXICAL_OF.csv \
    --relationships=HAS_DEFINITION=./HAS_DEFINITION.csv \
    --relationships=HAS_SEMANTIC_TYPE=./HAS_SEMANTIC_TYPE.csv \
    --relationships=HAS_ALLOWED_QUALIFIER=./HAS_ALLOWED_QUALIFIER.csv \
    --relationships=CHILD_OF=./CHILD_OF.csv \
    --relationships=PARENT_OF=./PARENT_OF.csv \
    --relationships=BROADER_THAN=./BROADER_THAN.csv \
    --relationships=NARROWER_THAN=./NARROWER_THAN.csv \
    --relationships=SYNONYM_OF=./SYNONYM_OF.csv \
    --relationships=POSSIBLY_SYNONYM_OF=./POSSIBLY_SYNONYM_OF.csv \
    --relationships=SIMILAR_TO=./SIMILAR_TO.csv \
    --relationships=RELATED_TO=./RELATED_TO.csv \
    --relationships=HAS_OTHER_RELATIONSHIP=./HAS_OTHER_RELATIONSHIP.csv \
    --relationships=QUALIFIED_BY=./QUALIFIED_BY.csv \
    --relationships=NOT_RELATED_TO=./NOT_RELATED_TO.csv \
    --relationships=DELETED=./DELETED.csv \
    --relationships=UNASSIGNED=./UNASSIGNED.csv \
    --relationships=IS_A=./SN-IS_A.csv \
    --relationships=PART_OF=./SN-PART_OF.csv \
    --relationships=CONCEPTUAL_PART_OF=./SN-CONCEPTUAL_PART_OF.csv \
    --relationships=PHYSICALLY_RELATED_TO=./SN-PHYSICALLY_RELATED_TO.csv \
    --relationships=SPATIALLY_RELATED_TO=./SN-SPATIALLY_RELATED_TO.csv \
    --relationships=TEMPORALLY_RELATED_TO=./SN-TEMPORALLY_RELATED_TO.csv \
    --relationships=FUNCTIONALLY_RELATED_TO=./SN-FUNCTIONALLY_RELATED_TO.csv \
    --relationships=CONCEPTUALLY_RELATED_TO=./SN-CONCEPTUALLY_RELATED_TO.csv \
    --relationships=ASSOCIATED_WITH=./SN-ASSOCIATED_WITH.csv \
    --relationships=CAUSES=./SN-CAUSES.csv \
    --relationships=PRODUCES=./SN-PRODUCES.csv \
    --relationships=AFFECTS=./SN-AFFECTS.csv \
    --relationships=DISRUPTS=./SN-DISRUPTS.csv \
    --relationships=PREVENTS=./SN-PREVENTS.csv \
    --relationships=TREATS=./SN-TREATS.csv \
    --relationships=MANAGES=./SN-MANAGES.csv \
    --relationships=COMPLICATES=./SN-COMPLICATES.csv \
    --relationships=MANIFESTATION_OF=./SN-MANIFESTATION_OF.csv \
    --relationships=DIAGNOSES=./SN-DIAGNOSES.csv \
    --relationships=INDICATES=./SN-INDICATES.csv \
    --relationships=ASSESSES_EFFECT_OF=./SN-ASSESSES_EFFECT_OF.csv \
    --relationships=MEASURES=./SN-MEASURES.csv \
    --relationships=MEASUREMENT_OF=./SN-MEASUREMENT_OF.csv \
    --relationships=EVALUATION_OF=./SN-EVALUATION_OF.csv \
    --relationships=PERFORMS=./SN-PERFORMS.csv \
    --relationships=CARRIES_OUT=./SN-CARRIES_OUT.csv \
    --relationships=PRACTICES=./SN-PRACTICES.csv \
    --relationships=USES=./SN-USES.csv \
    --relationships=PROCESS_OF=./SN-PROCESS_OF.csv \
    --relationships=RESULT_OF=./SN-RESULT_OF.csv \
    --relationships=BRINGS_ABOUT=./SN-BRINGS_ABOUT.csv \
    --relationships=OCCURS_IN=./SN-OCCURS_IN.csv \
    --relationships=EXHIBITS=./SN-EXHIBITS.csv \
    --relationships=INTERACTS_WITH=./SN-INTERACTS_WITH.csv \
    --relationships=CO_OCCURS_WITH=./SN-CO_OCCURS_WITH.csv \
    --relationships=PRECEDES=./SN-PRECEDES.csv \
    --relationships=LOCATION_OF=./SN-LOCATION_OF.csv \
    --relationships=CONTAINS=./SN-CONTAINS.csv \
    --relationships=CONNECTED_TO=./SN-CONNECTED_TO.csv \
    --relationships=ADJACENT_TO=./SN-ADJACENT_TO.csv \
    --relationships=INTERCONNECTS=./SN-INTERCONNECTS.csv \
    --relationships=SURROUNDS=./SN-SURROUNDS.csv \
    --relationships=TRAVERSES=./SN-TRAVERSES.csv \
    --relationships=BRANCH_OF=./SN-BRANCH_OF.csv \
    --relationships=TRIBUTARY_OF=./SN-TRIBUTARY_OF.csv \
    --relationships=CONSISTS_OF=./SN-CONSISTS_OF.csv \
    --relationships=INGREDIENT_OF=./SN-INGREDIENT_OF.csv \
    --relationships=DERIVATIVE_OF=./SN-DERIVATIVE_OF.csv \
    --relationships=DEVELOPMENTAL_FORM_OF=./SN-DEVELOPMENTAL_FORM_OF.csv \
    --relationships=DEGREE_OF=./SN-DEGREE_OF.csv \
    --relationships=PROPERTY_OF=./SN-PROPERTY_OF.csv \
    --relationships=METHOD_OF=./SN-METHOD_OF.csv \
    --relationships=ANALYZES=./SN-ANALYZES.csv \
    --relationships=ISSUE_IN=./SN-ISSUE_IN.csv \
    --additional-config=/var/lib/neo4j/conf/neo4j.conf
```

The two `--add-opens` JVM flags are required on recent JDK versions to allow Neo4j's importer to access internal NIO and language APIs. Adjust `--additional-config` to point to your actual `neo4j.conf` if it lives elsewhere.

## Library Crate

The `umls` library crate can be used independently in other projects.

### `umls`

An async, streaming client for local UMLS `.RRF` files. It opens each file with `tokio::fs`, deserializes records on demand via `futures::Stream`, and forwards them to the caller without ever holding the full dataset in memory.

| Method | File | Record type |
|---|---|---|
| `concept_names_and_sources` | `MRCONSO.RRF` | `CoNSoRecord` |
| `definitions` | `MRDEF.RRF` | `DefinitionRecord` |
| `semantic_types` | `MRSTY.RRF` | `SemanticTypeRecord` |
| `related_concepts` | `MRREL.RRF` | `RelatedConceptRecord` |
| `semantic_definitions` | `SRDEF` | `SemanticDefinition` |
| `semantic_types_relations` | `SRSTRE1` | `SemanticTypeRelationship` |

See [`crates/umls/README.md`](crates/umls/README.md) for the full API documentation and usage examples.

## Feature Flags

The `umls` crate exposes a `debug_path` feature. When enabled, parse errors include the exact field path where the failure occurred (e.g. `MRCONSO row 1042, field "LAT"`). This is useful during development but adds overhead; leave it disabled in production.

```toml
umls = { path = "...", features = ["debug_path"] }
```

## License

This project is licensed under the [MIT License](LICENSE).
