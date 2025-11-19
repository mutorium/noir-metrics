mod cli;

use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    let args = Cli::parse();

    if args.verbose {
        eprintln!("noir-metrics");
        eprintln!("  project_root: {}", args.project_root.display());
        eprintln!("  json: {}", args.json);
        eprintln!(
            "  output: {}",
            args.output
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "<stdout>".to_string())
        );
    }

    Ok(())
}
