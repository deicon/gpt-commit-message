use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "PROMPT_PREPARATION", )]
    pub preparation: Option<PathBuf>,

    /// Read user prompt from file, otherwise expect input on STDIN
    #[arg(short, long, value_name = "INPUT_FILE", )]
    pub file: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Prints default config which can be used as input for preparation flag
    CONFIG,
}
