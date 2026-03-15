use std::path::{Path, PathBuf};
use std::process::Command;

fn modql_binary() -> PathBuf {
    let mut path = std::env::current_exe().expect("Failed to get current exe path");
    path.pop();
    path.pop();
    path.push("modql");
    path
}

fn fixture_root_for(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn golden_dir_for(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join(name)
}

fn test_output_dir(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-output")
        .join(name)
}

fn read_file(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()))
}

fn run_modql(args: &[&str], workdir: &Path) {
    let binary = modql_binary();
    let output = Command::new(&binary)
        .args(args)
        .current_dir(workdir)
        .output()
        .unwrap_or_else(|e| panic!("Failed to run modql binary at {}: {e}", binary.display()));

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("modql command failed:\nstderr:\n{stderr}\nstdout:\n{stdout}");
    }
}

// ---------------------------------------------------------------------------
// Golden directory comparison
// ---------------------------------------------------------------------------

/// Compare all files in `golden_dir` against corresponding files in `actual_dir`.
/// Also checks for unexpected extra files in `actual_dir`.
fn compare_dirs(actual_dir: &Path, golden_dir: &Path) {
    assert!(
        golden_dir.exists(),
        "Golden directory does not exist: {}",
        golden_dir.display()
    );
    assert!(
        actual_dir.exists(),
        "Output directory does not exist: {}",
        actual_dir.display()
    );

    // Collect golden file names
    let mut golden_files: Vec<String> = std::fs::read_dir(golden_dir)
        .unwrap_or_else(|e| panic!("Failed to read golden dir {}: {e}", golden_dir.display()))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();
    golden_files.sort();

    assert!(
        !golden_files.is_empty(),
        "Golden directory is empty: {}",
        golden_dir.display()
    );

    // Check every golden file matches generated output
    for filename in &golden_files {
        let actual_path = actual_dir.join(filename);
        let golden_path = golden_dir.join(filename);

        assert!(
            actual_path.exists(),
            "Expected output file does not exist: {} (golden: {})",
            actual_path.display(),
            golden_dir.display()
        );

        let actual = read_file(&actual_path);
        let expected = read_file(&golden_path);
        pretty_assertions::assert_eq!(
            expected.trim(),
            actual.trim(),
            "Generated file does not match golden: {}",
            filename
        );
    }

    // Check for unexpected extra files in the output directory
    let mut actual_files: Vec<String> = std::fs::read_dir(actual_dir)
        .unwrap_or_else(|e| panic!("Failed to read output dir {}: {e}", actual_dir.display()))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();
    actual_files.sort();

    let extra: Vec<&String> = actual_files
        .iter()
        .filter(|f| !golden_files.contains(f))
        .collect();
    assert!(
        extra.is_empty(),
        "Unexpected extra files in output directory {}:\n  {}",
        actual_dir.display(),
        extra
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}

/// Run modql generate for a single-crate fixture and compare against golden output.
fn generate_and_compare(fixture_name: &str, golden_name: &str) {
    let out_dir = test_output_dir(golden_name);
    let _ = std::fs::remove_dir_all(&out_dir);

    let manifest = fixture_root_for(fixture_name).join("Cargo.toml");
    let manifest_str = manifest
        .to_str()
        .unwrap_or_else(|| panic!("Non-UTF8 manifest path: {}", manifest.display()));
    let out_dir_str = out_dir
        .to_str()
        .unwrap_or_else(|| panic!("Non-UTF8 output path: {}", out_dir.display()));

    run_modql(
        &[
            "generate",
            "--manifest-path",
            manifest_str,
            "--out",
            out_dir_str,
        ],
        Path::new(env!("CARGO_MANIFEST_DIR")),
    );

    let golden = golden_dir_for(golden_name);
    compare_dirs(&out_dir, &golden);
}

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn test_generate_defaults_to_docs_modql() {
    let fixture = fixture_root_for("simple");
    let out_dir = fixture.join("docs").join("modql");
    let _ = std::fs::remove_dir_all(&out_dir);

    run_modql(&["generate", "--manifest-path", "Cargo.toml"], &fixture);

    assert!(
        out_dir.exists(),
        "Expected default output directory to exist: {}",
        out_dir.display()
    );
    assert!(
        out_dir.join("index.md").exists(),
        "Expected default output file to exist: {}",
        out_dir.join("index.md").display()
    );
    assert!(
        out_dir.join("index.internal.md").exists(),
        "Expected internal output file to exist: {}",
        out_dir.join("index.internal.md").display()
    );

    let _ = std::fs::remove_dir_all(&out_dir);
}

#[test]
fn test_simple_matches_golden() {
    generate_and_compare("simple", "simple");
}

#[test]
fn test_advanced_matches_golden() {
    generate_and_compare("advanced", "advanced");
}

#[test]
fn test_workspace_matches_golden() {
    let out_dir = test_output_dir("workspace");
    let _ = std::fs::remove_dir_all(&out_dir);

    let manifest = fixture_root_for("workspace").join("Cargo.toml");
    let manifest_str = manifest
        .to_str()
        .unwrap_or_else(|| panic!("Non-UTF8 manifest path: {}", manifest.display()));
    let out_dir_str = out_dir
        .to_str()
        .unwrap_or_else(|| panic!("Non-UTF8 output path: {}", out_dir.display()));

    run_modql(
        &[
            "generate",
            "--manifest-path",
            manifest_str,
            "--out",
            out_dir_str,
        ],
        Path::new(env!("CARGO_MANIFEST_DIR")),
    );

    // Workspace auto-discover should create per-package subdirectories
    compare_dirs(&out_dir.join("core"), &golden_dir_for("workspace-core"));
    compare_dirs(
        &out_dir.join("service"),
        &golden_dir_for("workspace-service"),
    );
}
