/// Invoke `cargo rustdoc` to produce JSON output and read the result.
use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Options for generating rustdoc JSON.
pub struct RustdocOptions {
    pub manifest_path: PathBuf,
    pub package: Option<String>,
    pub nightly: String,
}

/// Run `cargo +<nightly> rustdoc` and return the deserialized `rustdoc_types::Crate`.
pub fn generate_rustdoc_json(opts: &RustdocOptions) -> Result<rustdoc_types::Crate> {
    let (target_dir, crate_name) = resolve_metadata(opts)?;
    invoke_cargo_rustdoc(opts)?;
    let json_path = locate_json(&target_dir, &crate_name)?;
    read_and_parse(&json_path)
}

/// Use `cargo metadata` to find the target directory and crate name.
fn resolve_metadata(opts: &RustdocOptions) -> Result<(PathBuf, String)> {
    let mut cmd = Command::new("cargo");
    cmd.arg("metadata")
        .arg("--manifest-path")
        .arg(&opts.manifest_path)
        .arg("--no-deps")
        .arg("--format-version")
        .arg("1");

    let output = cmd
        .output()
        .context("Failed to run `cargo metadata`. Is cargo installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("cargo metadata failed:\n{stderr}");
    }

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).context("Failed to parse cargo metadata JSON")?;

    let target_dir = json["target_directory"]
        .as_str()
        .context("cargo metadata missing target_directory")?;
    let target_dir = PathBuf::from(target_dir);

    let crate_name = if let Some(ref pkg) = opts.package {
        pkg.clone()
    } else {
        let packages = json["packages"]
            .as_array()
            .context("cargo metadata missing packages array")?;
        if packages.is_empty() {
            bail!("No packages found in cargo metadata");
        }
        packages[0]["name"]
            .as_str()
            .context("Package missing name field")?
            .to_string()
    };

    Ok((target_dir, crate_name))
}

/// Invoke `cargo +<nightly> rustdoc` with the appropriate flags.
fn invoke_cargo_rustdoc(opts: &RustdocOptions) -> Result<()> {
    let toolchain_arg = format!("+{}", opts.nightly);

    let mut cmd = Command::new("cargo");
    cmd.arg(&toolchain_arg)
        .arg("rustdoc")
        .arg("--manifest-path")
        .arg(&opts.manifest_path)
        .arg("-Z")
        .arg("unstable-options")
        .arg("--output-format")
        .arg("json");

    if let Some(ref pkg) = opts.package {
        cmd.arg("--package").arg(pkg);
    }

    // Always include private items so we can generate both surface and internal views.
    cmd.arg("--").arg("--document-private-items");

    let output = cmd.output().with_context(|| {
        format!(
            "Failed to run `cargo {}`. Is the '{}' toolchain installed?\n\
             Install it with: rustup toolchain install {}",
            toolchain_arg, opts.nightly, opts.nightly
        )
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Check for missing toolchain
        if stderr.contains("toolchain") && stderr.contains("not installed") {
            bail!(
                "The nightly toolchain '{}' is not installed.\n\
                 Install it with: rustup toolchain install {}\n\n\
                 stderr:\n{}",
                opts.nightly,
                opts.nightly,
                stderr
            );
        }

        bail!(
            "cargo rustdoc failed (exit code: {}):\n\nstderr:\n{}\n\nstdout:\n{}",
            output
                .status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            stderr,
            stdout
        );
    }

    Ok(())
}

/// Find the rustdoc JSON file in the target directory.
fn locate_json(target_dir: &Path, crate_name: &str) -> Result<PathBuf> {
    // rustdoc JSON is written to target/doc/<crate_name>.json
    // The crate name in filenames uses underscores instead of hyphens
    let file_name = format!("{}.json", crate_name.replace('-', "_"));
    let json_path = target_dir.join("doc").join(&file_name);

    if json_path.exists() {
        return Ok(json_path);
    }

    // If not found, list what's in the doc directory for a better error
    let doc_dir = target_dir.join("doc");
    if doc_dir.exists() {
        let entries: Vec<String> = std::fs::read_dir(&doc_dir)
            .ok()
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .filter_map(|e| {
                        let name = e.file_name().to_string_lossy().to_string();
                        if name.ends_with(".json") {
                            Some(name)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        if entries.is_empty() {
            bail!(
                "No rustdoc JSON file found at {}\n\
                 The doc directory exists but contains no .json files.\n\
                 Did cargo rustdoc complete successfully?",
                json_path.display()
            );
        } else {
            bail!(
                "Expected rustdoc JSON at {} but it was not found.\n\
                 Available JSON files in {}:\n  {}",
                json_path.display(),
                doc_dir.display(),
                entries.join("\n  ")
            );
        }
    }

    bail!(
        "No doc directory found at {}.\n\
         Did cargo rustdoc complete successfully?",
        doc_dir.display()
    );
}

/// Read the JSON file and deserialize into `rustdoc_types::Crate`.
fn read_and_parse(json_path: &Path) -> Result<rustdoc_types::Crate> {
    let content = std::fs::read_to_string(json_path)
        .with_context(|| format!("Failed to read {}", json_path.display()))?;

    let krate: rustdoc_types::Crate = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse rustdoc JSON from {}", json_path.display()))?;

    // Warn about format version mismatch but don't fail
    if krate.format_version != rustdoc_types::FORMAT_VERSION {
        eprintln!(
            "Warning: rustdoc JSON format version {} does not match expected version {}. \
             Output may be incomplete or incorrect.",
            krate.format_version,
            rustdoc_types::FORMAT_VERSION
        );
    }

    Ok(krate)
}
