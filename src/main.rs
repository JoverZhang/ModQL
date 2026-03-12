mod cli;
mod convert;
mod model;
mod naming;
mod render_md;
mod rustdoc_json;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Command};
use convert::ConvertMode;
use rustdoc_json::RustdocOptions;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate {
            manifest_path,
            out,
            package,
            nightly,
        } => {
            eprintln!("Generating rustdoc JSON...");
            let krate = rustdoc_json::generate_rustdoc_json(&RustdocOptions {
                manifest_path,
                package,
                nightly,
            })?;

            eprintln!("Converting public surface...");
            let surface_doc = convert::convert(&krate, ConvertMode::Surface)?;

            eprintln!("Converting internal view...");
            let internal_doc = convert::convert(&krate, ConvertMode::Internal)?;

            eprintln!("Rendering Markdown to {}...", out.display());
            render_md::render(&surface_doc, &internal_doc, &out)?;

            eprintln!("Done. Documentation written to {}", out.display());
        }
    }

    Ok(())
}
