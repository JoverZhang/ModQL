mod cli;
mod convert;
mod graph;
mod model;
mod naming;
mod render_md;
mod rustdoc_json;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Command};
use convert::ConvertMode;
use rustdoc_json::{PackageInfo, RustdocOptions};

fn generate_for_package(
    opts: &RustdocOptions,
    pkg: &PackageInfo,
    out_dir: &std::path::Path,
    is_workspace: bool,
    prefix: &str,
) -> Result<()> {
    eprintln!("{prefix}Generating rustdoc JSON...");
    let krate = rustdoc_json::generate_rustdoc_json(opts, pkg, is_workspace)?;

    eprintln!("{prefix}Converting public surface...");
    let surface_doc = convert::convert(&krate, ConvertMode::Surface)?;

    eprintln!("{prefix}Converting internal view...");
    let internal_doc = convert::convert(&krate, ConvertMode::Internal)?;

    eprintln!("{prefix}Rendering Markdown to {}...", out_dir.display());
    render_md::render(&surface_doc, &internal_doc, out_dir)?;

    eprintln!("{prefix}Building type graph...");
    graph::generate(&krate, out_dir)?;

    eprintln!(
        "{prefix}Done. Documentation written to {}",
        out_dir.display()
    );
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate {
            manifest_path,
            out,
            nightly,
        } => {
            let info = rustdoc_json::resolve_workspace_info(&manifest_path)?;
            let opts = RustdocOptions {
                manifest_path,
                nightly,
            };

            if info.is_workspace {
                let names: Vec<&str> = info.packages.iter().map(|p| p.name.as_str()).collect();
                eprintln!(
                    "Detected workspace with {} packages: {}",
                    info.packages.len(),
                    names.join(", ")
                );

                for pkg in &info.packages {
                    let pkg_out = out.join(&pkg.name);
                    eprintln!("\n--- Generating docs for package `{}` ---", pkg.name);
                    generate_for_package(&opts, pkg, &pkg_out, true, "  ")?;
                }

                eprintln!("\nAll packages processed.");
            } else {
                let pkg = &info.packages[0];
                generate_for_package(&opts, pkg, &out, false, "")?;
            }
        }
    }

    Ok(())
}
