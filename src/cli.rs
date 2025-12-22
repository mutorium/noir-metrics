use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Command-line arguments for noir-metrics.
///
/// Example:
///   noir-metrics . --format json --output metrics.json
#[derive(Debug, Parser)]
#[command(name = "noir-metrics")]
pub struct Cli {
    /// Path to the Noir project root (default: current directory)
    #[arg(value_name = "PROJECT_ROOT", default_value = ".")]
    pub project_root: PathBuf,

    /// Output format (`human` or `json`)
    #[arg(long, value_enum, value_name = "FORMAT")]
    pub format: Option<OutputFormat>,

    /// Backwards-compat alias for JSON output (prefer `--format json`)
    #[arg(long, hide = true)]
    pub json: bool,

    /// Write JSON output to this file instead of stdout
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Human,
    Json,
}
