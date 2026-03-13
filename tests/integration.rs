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
    assert!(
        out_dir.join("index.internal.md").exists(),
        "Expected internal output file to exist: {}",
        out_dir.join("index.internal.md").display()
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
        "index.internal.md",
        "module.simple.utils.md",
        "module.simple.utils.internal.md",
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

    let index_content = read_file(&out_dir.join("index.md"));
    assert!(
        !index_content.contains("secret: String"),
        "Surface view should not expand private struct fields"
    );
    assert!(
        !index_content.contains("display_name"),
        "Surface view should not expand private inherent methods"
    );
    assert!(
        !index_content.contains("pub(crate) fn secret"),
        "Surface view should not expand restricted inherent methods"
    );
    assert!(
        index_content.contains("pub(crate) fn internal_status() -> &'static str;"),
        "Surface view should include root private free functions as declarations"
    );
    assert!(
        index_content.contains("[`utils`](module.simple.utils.md)"),
        "Root surface should include private root modules in the module table"
    );
    assert!(
        index_content.contains("pub struct Greeter;"),
        "Surface view should summarize root types"
    );
    assert!(
        index_content.contains("pub trait Render;"),
        "Surface view should summarize root traits"
    );
    assert!(
        index_content.contains("pub enum Format;"),
        "Surface view should summarize root enums"
    );
    assert!(
        index_content.contains("impl Greeter;"),
        "Surface view should include inherent impl headers"
    );
    assert!(
        index_content.contains("impl Render for Greeter;"),
        "Surface view should include trait impl headers"
    );
    assert!(
        !index_content.contains("pub fn greet(&self) -> String;"),
        "Surface view should not expand impl methods"
    );
    assert!(
        !index_content.contains("/// Resolve an internal status string for diagnostics."),
        "Surface view should omit per-item doc comments"
    );

    // New item types in surface view
    assert!(
        index_content.contains("pub struct Marker;"),
        "Surface view should summarize unit struct"
    );
    assert!(
        index_content.contains("pub struct Wrapper;"),
        "Surface view should summarize tuple struct"
    );
    assert!(
        index_content.contains("pub struct Container<T: Clone>;"),
        "Surface view should summarize generic struct"
    );
    assert!(
        index_content.contains("pub(crate) struct Config;"),
        "Surface view should summarize private struct with pub(crate) prefix"
    );
    assert!(
        index_content.contains("pub enum Shape;"),
        "Surface view should summarize enum with mixed variant kinds"
    );
    assert!(
        index_content.contains("pub(crate) enum LogLevel;"),
        "Surface view should summarize private enum"
    );
    assert!(
        index_content.contains("pub(crate) trait Validate;"),
        "Surface view should summarize private trait"
    );
    assert!(
        index_content.contains("pub unsafe fn unsafe_op(ptr: *const u8) -> u8;"),
        "Surface view should include unsafe functions"
    );
    assert!(
        index_content.contains("pub const fn const_add(a: u32, b: u32) -> u32;"),
        "Surface view should include const functions"
    );
    assert!(
        index_content.contains("pub const MAX_RETRIES: u32 = 3u32;"),
        "Surface view should include public constants"
    );
    assert!(
        index_content.contains("pub(crate) const BUFFER_SIZE: usize"),
        "Surface view should include private constants"
    );
    assert!(
        index_content.contains("pub static VERSION: &str;"),
        "Surface view should include public statics"
    );
    assert!(
        index_content.contains("pub(crate) static mut INSTANCE_COUNT: u32;"),
        "Surface view should include private mutable statics"
    );
    assert!(
        index_content.contains("pub type Result<T> = Result<T, String>;"),
        "Surface view should include public type aliases"
    );
    assert!(
        index_content.contains("pub(crate) type OptStr = Option<String>;"),
        "Surface view should include private type aliases"
    );

    let module_content = read_file(&out_dir.join("module.simple.utils.md"));
    assert!(
        !module_content.contains("internal_helper"),
        "Nested module surface should still exclude private functions"
    );
    assert!(
        !module_content.contains("/// A helper function that formats a value."),
        "Nested module surface should omit per-item doc comments"
    );
    assert!(
        module_content.contains("pub const UTIL_VERSION: u32 = 1u32;"),
        "Nested module surface should include public constants"
    );
    assert!(
        !module_content.contains("UTIL_LIMIT"),
        "Nested module surface should exclude private constants"
    );

    // Verify no old per-item files are generated
    let old_files = [
        "struct.simple.Greeter.md",
        "enum.simple.Format.md",
        "trait.simple.Render.md",
        "function.simple.run.md",
        "function.simple.utils.helper.md",
    ];
    for filename in &old_files {
        let path = out_dir.join(filename);
        assert!(
            !path.exists(),
            "Old per-item file should not exist: {}",
            path.display()
        );
    }
}

#[test]
fn test_generate_simple_fixture_writes_internal_view_with_private_items() {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-output")
        .join("simple-internal-view");
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

    let index_content = read_file(&out_dir.join("index.internal.md"));
    assert!(
        index_content.contains("secret: String"),
        "Internal view should include private struct fields"
    );
    assert!(
        index_content.contains("pub(crate) fn display_name(&self) -> &str;"),
        "Internal view should include private inherent methods"
    );
    assert!(
        index_content.contains("pub(crate) fn secret(&self) -> &str;"),
        "Internal view should include restricted inherent methods"
    );
    assert!(
        index_content.contains("pub(crate) fn internal_status() -> &'static str;"),
        "Internal view should include private root free functions"
    );

    // New item types in internal view
    assert!(
        index_content.contains("pub struct Marker;"),
        "Internal view should include unit struct"
    );
    assert!(
        index_content.contains("pub struct Wrapper(pub String);"),
        "Internal view should expand tuple struct fields"
    );
    assert!(
        index_content.contains("pub struct Container<T: Clone>"),
        "Internal view should include generic struct"
    );
    assert!(
        index_content.contains("pub(crate) struct Config"),
        "Internal view should include private struct"
    );
    assert!(
        index_content.contains("debug: bool"),
        "Internal view should show private struct fields"
    );
    assert!(
        index_content.contains("pub enum Shape"),
        "Internal view should include enum with mixed variants"
    );
    assert!(
        index_content.contains("Circle(f64)"),
        "Internal view should show tuple variant"
    );
    assert!(
        index_content.contains("width: f64"),
        "Internal view should show struct variant fields"
    );
    assert!(
        index_content.contains("pub(crate) enum LogLevel"),
        "Internal view should include private enum"
    );
    assert!(
        index_content.contains("pub(crate) trait Validate"),
        "Internal view should include private trait"
    );
    assert!(
        index_content.contains("fn validate(&self) -> bool;"),
        "Internal view should show private trait methods"
    );
    assert!(
        index_content.contains("pub unsafe fn unsafe_op(ptr: *const u8) -> u8;"),
        "Internal view should include unsafe functions"
    );
    assert!(
        index_content.contains("pub const fn const_add(a: u32, b: u32) -> u32;"),
        "Internal view should include const functions"
    );
    assert!(
        index_content.contains("pub const MAX_RETRIES: u32 = 3u32;"),
        "Internal view should include public constants"
    );
    assert!(
        index_content.contains("pub(crate) const BUFFER_SIZE: usize"),
        "Internal view should include private constants"
    );
    assert!(
        index_content.contains("pub static VERSION: &str;"),
        "Internal view should include public statics"
    );
    assert!(
        index_content.contains("pub(crate) static mut INSTANCE_COUNT: u32;"),
        "Internal view should include private mutable statics"
    );
    assert!(
        index_content.contains("pub type Result<T> = Result<T, String>;"),
        "Internal view should include public type aliases"
    );
    assert!(
        index_content.contains("pub(crate) type OptStr = Option<String>;"),
        "Internal view should include private type aliases"
    );

    // Separator assertions for internal view
    assert!(
        index_content.contains("// -- private --"),
        "Internal view should have private separator in code blocks"
    );
    assert!(
        index_content.contains("---\n"),
        "Internal view should have horizontal rule separator between pub and private sections"
    );

    // Verify pub items appear before private items in impl blocks
    let impl_greeter_pos = index_content
        .find("pub fn greet(&self) -> String;")
        .unwrap();
    let private_separator_pos = index_content[impl_greeter_pos..]
        .find("// -- private --")
        .map(|p| p + impl_greeter_pos);
    let display_name_pos = index_content
        .find("pub(crate) fn display_name(&self) -> &str;")
        .unwrap();
    assert!(
        private_separator_pos.is_some(),
        "Internal impl block should have private separator"
    );
    assert!(
        private_separator_pos.unwrap() < display_name_pos,
        "Private separator should appear before private methods in impl block"
    );

    let module_content = read_file(&out_dir.join("module.simple.utils.internal.md"));
    assert!(
        module_content.contains("fn internal_helper(value: &str) -> String"),
        "Internal view should include private module functions"
    );
    assert!(
        module_content.contains("/// A helper function that formats a value."),
        "Internal view should include doc comments"
    );
    assert!(
        module_content.contains("// -- private --"),
        "Internal module view should have private separator"
    );
    assert!(
        module_content.contains("pub const UTIL_VERSION: u32 = 1u32;"),
        "Internal module view should include public constants"
    );
    assert!(
        module_content.contains("UTIL_LIMIT"),
        "Internal module view should include private constants"
    );
}
