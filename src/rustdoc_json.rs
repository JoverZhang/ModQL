/// Invoke `cargo rustdoc` to produce JSON output and read the result.
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Options for generating rustdoc JSON.
pub struct RustdocOptions {
    pub manifest_path: PathBuf,
    pub nightly: String,
}

/// Metadata about a single Cargo package extracted from `cargo metadata`.
pub struct PackageInfo {
    /// The package name (e.g. `"mira-cli"`).
    pub name: String,
    /// The library target name, if present (e.g. `"mira_core"`).
    pub lib_target: Option<String>,
    /// The first binary target name, if present (e.g. `"mira"`).
    pub bin_target: Option<String>,
}

impl PackageInfo {
    /// The target name that will be documented by `cargo rustdoc`.
    /// Prefers the lib target; falls back to the first bin target.
    pub fn doc_target_name(&self) -> Option<&str> {
        self.lib_target.as_deref().or(self.bin_target.as_deref())
    }
}

/// Information about the workspace / package layout.
pub struct WorkspaceInfo {
    /// True when the manifest defines a `[workspace]` with multiple members.
    pub is_workspace: bool,
    /// Packages discovered via `cargo metadata`.
    pub packages: Vec<PackageInfo>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Inspect the manifest via `cargo metadata` and decide whether we are dealing
/// with a single crate or a workspace with multiple members.
pub fn resolve_workspace_info(manifest_path: &Path) -> Result<WorkspaceInfo> {
    let json = run_cargo_metadata(manifest_path)?;

    let packages = json["packages"]
        .as_array()
        .context("cargo metadata missing packages array")?;

    let workspace_members = json["workspace_members"]
        .as_array()
        .context("cargo metadata missing workspace_members")?;

    let is_workspace = workspace_members.len() > 1;

    let package_infos: Vec<PackageInfo> = packages
        .iter()
        .filter_map(|p| {
            let name = p["name"].as_str()?.to_string();
            let targets = p["targets"].as_array()?;

            let lib_target = targets.iter().find_map(|t| {
                let kinds = t["kind"].as_array()?;
                if kinds.iter().any(|k| k.as_str() == Some("lib")) {
                    t["name"].as_str().map(|s| s.to_string())
                } else {
                    None
                }
            });

            let bin_target = targets.iter().find_map(|t| {
                let kinds = t["kind"].as_array()?;
                if kinds.iter().any(|k| k.as_str() == Some("bin")) {
                    t["name"].as_str().map(|s| s.to_string())
                } else {
                    None
                }
            });

            Some(PackageInfo {
                name,
                lib_target,
                bin_target,
            })
        })
        .collect();

    if package_infos.is_empty() {
        bail!("No packages found in cargo metadata");
    }

    Ok(WorkspaceInfo {
        is_workspace,
        packages: package_infos,
    })
}

/// Run `cargo +<nightly> rustdoc` for a single package and return the
/// deserialized `rustdoc_types::Crate`.
pub fn generate_rustdoc_json(
    opts: &RustdocOptions,
    pkg: &PackageInfo,
    is_workspace: bool,
) -> Result<rustdoc_types::Crate> {
    let target_dir = resolve_target_dir(&opts.manifest_path)?;

    let doc_target = pkg.doc_target_name().with_context(|| {
        format!(
            "Package `{}` has no library or binary target to document",
            pkg.name
        )
    })?;

    invoke_cargo_rustdoc(opts, pkg, is_workspace)?;
    let json_path = locate_json(&target_dir, doc_target)?;
    read_and_parse(&json_path)
}

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

fn run_cargo_metadata(manifest_path: &Path) -> Result<serde_json::Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("metadata")
        .arg("--manifest-path")
        .arg(manifest_path)
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

    serde_json::from_slice(&output.stdout).context("Failed to parse cargo metadata JSON")
}

/// Extract just the target directory from `cargo metadata`.
fn resolve_target_dir(manifest_path: &Path) -> Result<PathBuf> {
    let json = run_cargo_metadata(manifest_path)?;
    let target_dir = json["target_directory"]
        .as_str()
        .context("cargo metadata missing target_directory")?;
    Ok(PathBuf::from(target_dir))
}

/// Invoke `cargo +<nightly> rustdoc` with the appropriate flags.
fn invoke_cargo_rustdoc(
    opts: &RustdocOptions,
    pkg: &PackageInfo,
    is_workspace: bool,
) -> Result<()> {
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

    // In a workspace we must specify the package.
    if is_workspace {
        cmd.arg("--package").arg(&pkg.name);
    }

    // Explicitly select the target to document so that the extra rustdoc
    // flags (after `--`) are unambiguous when a package has multiple targets.
    if pkg.lib_target.is_some() {
        cmd.arg("--lib");
    } else if let Some(ref bin) = pkg.bin_target {
        cmd.arg("--bin").arg(bin);
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
fn locate_json(target_dir: &Path, target_name: &str) -> Result<PathBuf> {
    // rustdoc JSON is written to target/doc/<target_name>.json
    // The target name in filenames uses underscores instead of hyphens
    let file_name = format!("{}.json", target_name.replace('-', "_"));
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
