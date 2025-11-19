use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments for noir-metrics.
///
/// Example:
///   noir-metrics . --json --output metrics.json
#[derive(Debug, Parser)]
#[command(name = "noir-metrics")]
pub struct Cli {
    /// Path to the Noir project root (default: current directory)
    #[arg(value_name = "PROJECT_ROOT", default_value = ".")]
    pub project_root: PathBuf,

    /// Output JSON instead of a human-readable summary
    #[arg(long)]
    pub json: bool,

    /// Write JSON output to this file instead of stdout
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}
