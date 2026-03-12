use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "modql",
    about = "Generate docs.rs-like Markdown from Rust projects"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Generate Markdown documentation from a Rust crate
    Generate {
        /// Path to the crate's Cargo.toml
        #[arg(long)]
        manifest_path: PathBuf,

        /// Output directory for generated Markdown files
        #[arg(long, default_value = "docs/modql")]
        out: PathBuf,

        /// Package name (for workspaces)
        #[arg(long)]
        package: Option<String>,

        /// Nightly toolchain to use (e.g. "nightly" or "nightly-2024-01-01")
        #[arg(long, default_value = "nightly")]
        nightly: String,
    },
}
