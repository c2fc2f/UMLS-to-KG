//! example

use clap::Parser;
use futures::StreamExt;
use std::{error::Error, path::PathBuf};
use umls::{UMLS, metathesaurus::conso::models::CoNSoRecord};

/// Simple program to test the library
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the folder containing UMLS dataset
    #[arg(short, long)]
    folder: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();

    let umls: UMLS = UMLS::new(args.folder);

    let mut stream = umls.concept_names_and_sources();

    while let Some(record) = stream.next().await {
        let record: CoNSoRecord = record?;

        println!("CUI: {} AUI: {}", record.cui, record.aui);
    }

    Ok(())
}
