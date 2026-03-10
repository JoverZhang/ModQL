use std::path::{Path, PathBuf};
use std::process::Command;

fn modql_binary() -> PathBuf {
    let mut path = std::env::current_exe().expect("Failed to get current exe path");
    path.pop();
    path.pop();
    path.push("modql");
    path
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("simple")
}

fn fixture_manifest() -> PathBuf {
    fixture_root().join("Cargo.toml")
}

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join("simple")
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

#[test]
fn test_generate_defaults_to_docs_modql() {
    let fixture = fixture_root();
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

    let _ = std::fs::remove_dir_all(&out_dir);
}

#[test]
fn test_generate_simple_fixture_matches_golden() {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-output")
        .join("simple-golden");
    let _ = std::fs::remove_dir_all(&out_dir);

    let manifest = fixture_manifest();
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

    let expected_files = [
        "index.md",
        "module.simple.utils.md",
        "struct.simple.Greeter.md",
        "enum.simple.Format.md",
        "trait.simple.Render.md",
        "function.simple.run.md",
        "function.simple.utils.helper.md",
    ];

    let golden = golden_dir();
    for filename in &expected_files {
        let actual_path = out_dir.join(filename);
        let golden_path = golden.join(filename);

        assert!(
            actual_path.exists(),
            "Expected output file does not exist: {}",
            actual_path.display()
        );
        assert!(
            golden_path.exists(),
            "Missing golden file: {}",
            golden_path.display()
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
}
