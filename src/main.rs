use std::time::Instant;

use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger::Env;
use essence::{summarizer::Summarizer, transcriber::Transcriber};
use log::info;

#[derive(Parser)]
#[command(name = "essence")]
#[command(about = "Essence is a tool for summarizing meetings")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Transcribe {
        #[arg(short = 'i', long)]
        audio_path: String,
        #[arg(short = 'l', long)]
        language: String,
        #[arg(short = 'm', long)]
        model_path: String,
    },
    Summarize {
        #[arg(short = 'i', long)]
        transcript_path: String,
        #[arg(short = 'm', long)]
        model_name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::new().filter_or("RUST_LOG", "info");
    env_logger::Builder::from_env(env)
        .target(env_logger::Target::Stderr)
        .init();

    let cli = Cli::parse();

    let start_time = Instant::now();

    match cli.command {
        Commands::Transcribe {
            model_path,
            audio_path,
            language,
        } => {
            let mut transcriber = Transcriber::new(&model_path)?;
            let transcript = transcriber.transcribe(&audio_path, &language)?;
            println!("{}", transcript);
        }
        Commands::Summarize {
            transcript_path,
            model_name,
        } => {
            let summarizer = Summarizer::new();
            let transcript = std::fs::read_to_string(transcript_path)?;

            summarizer
                .summarize_stream(&transcript, &model_name)
                .await?;
        }
    }

    let duration = Instant::now().duration_since(start_time);
    info!("Time taken: {:?}", duration);

    Ok(())
}
