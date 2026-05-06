//! CLI tool that converts the UMLS dataset into a CSV-based Knowledge Graph
//! representation (Neo4J)

mod saver;

use clap::Parser;
use futures::StreamExt;
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
};
use tokio::task::JoinError;
use umls::UMLS;

use crate::saver::{
    definition::DefinitionSaver, entity::EntitySaver, relation::RelationSaver,
    semdef::SemTypeSaver, semmet::SemTypeRelMetSaver, semrel::SemTypeRelSaver,
};

/// CLI tool that converts the UMLS dataset into a CSV-based Knowledge Graph
/// representation (Neo4J)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the folder containing UMLS dataset
    #[arg(short, long)]
    umls: PathBuf,

    /// Output dir
    #[arg(short, long, default_value = ".")]
    output: PathBuf,
}

/// Spawn a saver
macro_rules! spawn_saver {
    ($umls:expr, $output:expr, $saver_type:ty, $stream_method:ident) => {{
        let umls = Arc::clone(&$umls);
        let output = Arc::clone(&$output);

        tokio::spawn(async move {
            let mut saver: $saver_type = <$saver_type>::new(&output).await?;
            let mut stream = umls.$stream_method();

            while let Some(record) = stream.next().await {
                let record = record?;
                saver.save_record(record).await?;
            }
            saver.flush().await?;

            anyhow::Ok(())
        })
    }};
}

#[tokio::main]
async fn main() -> ExitCode {
    let args: Args = Args::parse();

    let umls: Arc<UMLS> = Arc::new(UMLS::new(args.umls));
    let output: Arc<Path> = Arc::from(args.output);

    let join: Result<_, JoinError> = tokio::try_join!(
        spawn_saver!(umls, output, EntitySaver, concept_names_and_sources),
        spawn_saver!(umls, output, DefinitionSaver, definitions),
        spawn_saver!(umls, output, SemTypeRelMetSaver, semantic_types),
        spawn_saver!(umls, output, RelationSaver, related_concepts),
        spawn_saver!(umls, output, SemTypeRelSaver, semantic_types_relations),
        spawn_saver!(umls, output, SemTypeSaver, semantic_definitions),
    );

    if let Err(e) = join {
        eprintln!("Error during writing of the CSV files:\n{:?}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
