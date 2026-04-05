// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
/// TieXiu: A high-performance PEG engine for TatSu grammars.
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Execute a grammar against one or more input files.
    Run {
        /// Path to the compiled TatSu JSON grammar.
        // #[arg(short, long)]
        #[arg(required = true)]
        grammar: PathBuf,

        /// The files to be parsed.
        #[arg(required = true)]
        inputs: Vec<PathBuf>,

        /// Display a detailed trace of the parsing process.
        #[arg(short, long)]
        trace: bool,
    },
}

pub fn cli() {
    use crate::ui::cli::{Cli, Commands};
    use clap::Parser;
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            grammar, inputs, ..
        } => {
            println!(
                "Ready to parse with grammar {}  {}",
                grammar.as_path().to_str().unwrap(),
                inputs
                    .iter()
                    .map(|p| p.as_path().to_str().unwrap())
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
    }
}
