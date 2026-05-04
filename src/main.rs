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
use umls::{
    UMLS, conso::models::CoNSoRecord, definition::models::DefinitionRecord,
};

use crate::saver::{definition::DefinitionSaver, entity::EntitySaver};

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

#[tokio::main]
async fn main() -> ExitCode {
    let args: Args = Args::parse();

    let umls: Arc<UMLS> = Arc::new(UMLS::new(args.umls));
    let output: Arc<Path> = Arc::from(args.output);

    let r1 = tokio::spawn({
        let umls: Arc<UMLS> = Arc::clone(&umls);
        let output: Arc<Path> = Arc::clone(&output);
        async move {
            let mut saver = EntitySaver::new(&output).await?;
            let mut stream = umls.concept_names_and_sources();

            while let Some(record) = stream.next().await {
                let record: CoNSoRecord = record?;
                saver.save_record(record).await?;
            }
            saver.flush().await?;

            anyhow::Ok(())
        }
    });

    let r2 = tokio::spawn(async move {
        let mut saver = DefinitionSaver::new(&output).await?;
        let mut stream = umls.definitions();

        while let Some(record) = stream.next().await {
            let record: DefinitionRecord = record?;
            saver.save_record(record).await?;
        }
        saver.flush().await?;

        anyhow::Ok(())
    });

    let (r1, r2) = tokio::join!(r1, r2);

    for res in [r1, r2] {
        if let Err(e) = res {
            eprintln!("Error during writing of the CSV files:\n{:?}", e);
        }
    }

    ExitCode::SUCCESS
}
