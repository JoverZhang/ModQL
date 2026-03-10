mod cli;
mod convert;
mod model;
mod naming;
mod render_md;
mod rustdoc_json;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Command};
use rustdoc_json::RustdocOptions;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate {
            manifest_path,
            out,
            package,
            document_private_items,
            nightly,
        } => {
            eprintln!("Generating rustdoc JSON...");
            let krate = rustdoc_json::generate_rustdoc_json(&RustdocOptions {
                manifest_path,
                package,
                document_private_items,
                nightly,
            })?;

            eprintln!("Converting to documentation model...");
            let crate_doc = convert::convert(&krate)?;

            eprintln!("Rendering Markdown to {}...", out.display());
            render_md::render(&crate_doc, &out)?;

            eprintln!("Done. Documentation written to {}", out.display());
        }
    }

    Ok(())
}
