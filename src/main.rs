// This is free and unencumbered software released into the public domain.
// Contributors: GotEmCoach (mcochran), KOVA, Claude Opus 4.6, Mattbusel (XFactor)
//
//! battle-bros: Tiny offline AI for military regulation training.
//! Soldier Board simulations. Runs on AR glasses. No cloud. No excuses.
//!
//! Train soldiers to make split-second regulation decisions when
//! there's no one to hold their hand in the middle of a battlefield.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "battle-bros", version, about = "Soldier Board sims. Regulation sharpness. Offline AI.")]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run a Soldier Board simulation. Rapid-fire regulation scenarios.
    Sim {
        /// Regulation domain: ucmj, ar670, fm, all
        #[arg(short, long, default_value = "all")]
        domain: String,
        /// Number of scenarios (default: 10)
        #[arg(short, long, default_value = "10")]
        count: usize,
        /// Difficulty: green, amber, red (default: green)
        #[arg(long, default_value = "green")]
        difficulty: String,
    },
    /// Ingest regulation text files into the training corpus.
    Ingest {
        /// Path to regulation text file or directory.
        path: std::path::PathBuf,
        /// Regulation domain tag (ucmj, ar670, fm, etc.)
        #[arg(short, long)]
        domain: String,
    },
    /// Train a tiny model from ingested regulations.
    Train {
        /// Output model path (default: models/battle-bros.gguf)
        #[arg(short, long)]
        output: Option<std::path::PathBuf>,
        /// Target size in MB (default: 50)
        #[arg(long, default_value = "50")]
        target_mb: usize,
    },
    /// List available regulation domains and scenario counts.
    List,
    /// Export scenarios as JSON for AR glass deployment.
    Export {
        /// Output directory
        #[arg(short, long, default_value = "export")]
        output: std::path::PathBuf,
    },
}

mod regs;
mod sims;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Some(Cmd::Sim { domain, count, difficulty }) => {
            sims::run_board(&domain, count, &difficulty)?;
        }
        Some(Cmd::Ingest { path, domain }) => {
            regs::ingest(&path, &domain)?;
        }
        Some(Cmd::Train { output, target_mb }) => {
            let out = output.unwrap_or_else(|| "models/battle-bros.gguf".into());
            println!("train: {} target={}MB (not yet implemented)", out.display(), target_mb);
        }
        Some(Cmd::List) => {
            regs::list_domains()?;
        }
        Some(Cmd::Export { output }) => {
            sims::export(&output)?;
        }
        None => {
            // Default: interactive sim, like a board president grilling you
            sims::run_board("all", 10, "green")?;
        }
    }
    Ok(())
}
