use std::path::{Path, PathBuf};
use std::process::Command;

fn modql_binary() -> PathBuf {
    // The test binary is in target/debug/modql
    let mut path = std::env::current_exe().expect("Failed to get current exe path");
    // Go up from target/debug/deps/<test-binary> to target/debug/
    path.pop();
    path.pop();
    path.push("modql");
    path
}

fn fixture_manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("simple")
        .join("Cargo.toml")
}

fn assert_file_contains(path: &Path, expected: &str) {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
    assert!(
        content.contains(expected),
        "File {} does not contain expected text: {}\n\nActual content:\n{}",
        path.display(),
        expected,
        content
    );
}

#[test]
fn test_generate_simple_fixture() {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-output")
        .join("simple-docs");

    // Clean up previous output
    let _ = std::fs::remove_dir_all(&out_dir);

    let binary = modql_binary();
    let manifest = fixture_manifest();

    let output = Command::new(&binary)
        .arg("generate")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--out")
        .arg(&out_dir)
        .output()
        .unwrap_or_else(|e| panic!("Failed to run modql binary at {}: {e}", binary.display()));

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("modql generate failed:\nstderr:\n{stderr}\nstdout:\n{stdout}");
    }

    // Check that expected files exist
    let expected_files = [
        "index.md",
        "module.simple.utils.md",
        "struct.simple.Greeter.md",
        "enum.simple.Format.md",
        "trait.simple.Render.md",
        "function.simple.run.md",
        "function.simple.utils.helper.md",
    ];

    for filename in &expected_files {
        let path = out_dir.join(filename);
        assert!(
            path.exists(),
            "Expected output file does not exist: {}",
            path.display()
        );
    }

    // Verify crate page content
    let index_path = out_dir.join("index.md");
    assert_file_contains(&index_path, "# Crate `simple`");
    assert_file_contains(&index_path, "A simple test crate for ModQL.");
    assert_file_contains(&index_path, "## Modules");
    assert_file_contains(&index_path, "## Structs");
    assert_file_contains(&index_path, "## Enums");
    assert_file_contains(&index_path, "## Traits");
    assert_file_contains(&index_path, "## Functions");
    assert_file_contains(&index_path, "[`utils`](module.simple.utils.md)");
    assert_file_contains(&index_path, "[`Greeter`](struct.simple.Greeter.md)");
    assert_file_contains(&index_path, "[`Format`](enum.simple.Format.md)");
    assert_file_contains(&index_path, "[`Render`](trait.simple.Render.md)");
    assert_file_contains(&index_path, "[`run`](function.simple.run.md)");

    // Verify module page is distinct from crate page
    let module_path = out_dir.join("module.simple.utils.md");
    assert_file_contains(&module_path, "# Module `simple::utils`");
    assert_file_contains(&module_path, "Utility functions");
    assert_file_contains(&module_path, "[`helper`](function.simple.utils.helper.md)");

    // Verify struct page with methods
    let struct_path = out_dir.join("struct.simple.Greeter.md");
    assert_file_contains(&struct_path, "# Struct `simple::Greeter`");
    assert_file_contains(&struct_path, "A greeting struct");
    assert_file_contains(&struct_path, "## Methods");
    assert_file_contains(&struct_path, "fn new(");
    assert_file_contains(&struct_path, "fn greet(");
    assert_file_contains(&struct_path, "Create a new");
    assert_file_contains(&struct_path, "Generate a greeting");

    // Verify enum page
    let enum_path = out_dir.join("enum.simple.Format.md");
    assert_file_contains(&enum_path, "# Enum `simple::Format`");
    assert_file_contains(&enum_path, "Plain");
    assert_file_contains(&enum_path, "Rich");

    // Verify trait page
    let trait_path = out_dir.join("trait.simple.Render.md");
    assert_file_contains(&trait_path, "# Trait `simple::Render`");
    assert_file_contains(&trait_path, "fn render(");

    // Verify function page
    let fn_path = out_dir.join("function.simple.run.md");
    assert_file_contains(&fn_path, "# Function `simple::run`");
    assert_file_contains(&fn_path, "fn run()");
    assert_file_contains(&fn_path, "Run the application");

    // Golden test: verify the complete index.md content is stable
    let index_content = std::fs::read_to_string(&index_path).expect("Failed to read index.md");

    // Save golden file for future reference
    let golden_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden");
    std::fs::create_dir_all(&golden_dir).expect("Failed to create golden dir");
    let golden_path = golden_dir.join("index.md");

    if golden_path.exists() {
        let golden = std::fs::read_to_string(&golden_path).expect("Failed to read golden file");
        pretty_assertions::assert_eq!(
            golden.trim(),
            index_content.trim(),
            "index.md does not match golden file. \
             If the change is intentional, delete tests/golden/index.md and re-run tests."
        );
    } else {
        // First run: create the golden file
        std::fs::write(&golden_path, &index_content).expect("Failed to write golden file");
        eprintln!("Golden file created at {}", golden_path.display());
    }
}
