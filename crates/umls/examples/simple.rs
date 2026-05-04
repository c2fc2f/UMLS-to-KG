//! example

use clap::Parser;
use futures::StreamExt;
use std::{error::Error, path::PathBuf};
use umls::{UMLS, conso::models::CoNSoRecord};

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

    while let Some(record) = umls.concept_names_and_sources().next().await {
        let record: CoNSoRecord = record?;

        println!("CUI: {}", record.cui);
    }

    Ok(())
}
