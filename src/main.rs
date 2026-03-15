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
            nightly,
        } => {
            let info = rustdoc_json::resolve_workspace_info(&manifest_path)?;

            if info.is_workspace {
                eprintln!(
                    "Detected workspace with {} packages: {}",
                    info.packages.len(),
                    info.packages.join(", ")
                );

                let opts = RustdocOptions {
                    manifest_path,
                    nightly,
                };

                for pkg_name in &info.packages {
                    let pkg_out = out.join(pkg_name);
                    eprintln!("\n--- Generating docs for package `{pkg_name}` ---");

                    eprintln!("  Generating rustdoc JSON...");
                    let krate =
                        rustdoc_json::generate_rustdoc_json(&opts, Some(pkg_name.as_str()))?;

                    eprintln!("  Converting public surface...");
                    let surface_doc = convert::convert(&krate, ConvertMode::Surface)?;

                    eprintln!("  Converting internal view...");
                    let internal_doc = convert::convert(&krate, ConvertMode::Internal)?;

                    eprintln!("  Rendering Markdown to {}...", pkg_out.display());
                    render_md::render(&surface_doc, &internal_doc, &pkg_out)?;

                    eprintln!("  Done. Documentation written to {}", pkg_out.display());
                }

                eprintln!("\nAll packages processed.");
            } else {
                let opts = RustdocOptions {
                    manifest_path,
                    nightly,
                };

                eprintln!("Generating rustdoc JSON...");
                let krate = rustdoc_json::generate_rustdoc_json(&opts, None)?;

                eprintln!("Converting public surface...");
                let surface_doc = convert::convert(&krate, ConvertMode::Surface)?;

                eprintln!("Converting internal view...");
                let internal_doc = convert::convert(&krate, ConvertMode::Internal)?;

                eprintln!("Rendering Markdown to {}...", out.display());
                render_md::render(&surface_doc, &internal_doc, &out)?;

                eprintln!("Done. Documentation written to {}", out.display());
            }
        }
    }

    Ok(())
}
