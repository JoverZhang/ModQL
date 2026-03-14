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

// ---------------------------------------------------------------------------
// Parameterized helpers for new fixtures
// ---------------------------------------------------------------------------

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

/// Run modql generate against a fixture and compare all expected files to golden output.
/// Returns the output directory for further assertions.
fn generate_and_compare(
    fixture_name: &str,
    golden_name: &str,
    expected_files: &[&str],
    package: Option<&str>,
) -> PathBuf {
    let out_dir = test_output_dir(golden_name);
    let _ = std::fs::remove_dir_all(&out_dir);

    let manifest = fixture_root_for(fixture_name).join("Cargo.toml");
    let manifest_str = manifest
        .to_str()
        .unwrap_or_else(|| panic!("Non-UTF8 manifest path: {}", manifest.display()));
    let out_dir_str = out_dir
        .to_str()
        .unwrap_or_else(|| panic!("Non-UTF8 output path: {}", out_dir.display()));

    let mut args = vec![
        "generate",
        "--manifest-path",
        manifest_str,
        "--out",
        out_dir_str,
    ];
    if let Some(pkg) = package {
        args.push("--package");
        args.push(pkg);
    }

    run_modql(&args, Path::new(env!("CARGO_MANIFEST_DIR")));

    let golden = golden_dir_for(golden_name);
    for filename in expected_files {
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
            "Generated file does not match golden: {} (golden: {})",
            filename,
            golden_name
        );
    }

    out_dir
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

    // Two-zone layout assertions for internal view
    assert!(
        index_content.contains("---\n"),
        "Internal view should have a single page-level horizontal rule between public and private zones"
    );
    assert!(
        !index_content.contains("// -- private --"),
        "Internal view should not have old per-section private separators"
    );

    // Verify private zone section headers
    assert!(
        index_content.contains("## Structs (private)"),
        "Internal view should have private structs section header"
    );
    assert!(
        index_content.contains("## Enums (private)"),
        "Internal view should have private enums section header"
    );
    assert!(
        index_content.contains("## Traits (private)"),
        "Internal view should have private traits section header"
    );
    assert!(
        index_content.contains("## Impl Blocks (private)"),
        "Internal view should have private impl blocks section header"
    );
    assert!(
        index_content.contains("## Functions (private)"),
        "Internal view should have private functions section header"
    );
    assert!(
        index_content.contains("## Type Aliases (private)"),
        "Internal view should have private type aliases section header"
    );
    assert!(
        index_content.contains("## Constants (private)"),
        "Internal view should have private constants section header"
    );
    assert!(
        index_content.contains("## Statics (private)"),
        "Internal view should have private statics section header"
    );

    // Verify public zone precedes private zone
    let separator_pos = index_content
        .find("\n---\n")
        .expect("Should have page-level separator");
    let pub_structs_pos = index_content
        .find("## Structs\n")
        .expect("Should have public structs");
    let priv_structs_pos = index_content
        .find("## Structs (private)")
        .expect("Should have private structs");
    assert!(
        pub_structs_pos < separator_pos,
        "Public structs should appear before the separator"
    );
    assert!(
        separator_pos < priv_structs_pos,
        "Private structs should appear after the separator"
    );

    // Verify impl Greeter appears in both zones with different method subsets
    let pub_impl_greeter = index_content
        .find("## Impl Blocks\n")
        .expect("Should have public impl blocks");
    let priv_impl_greeter = index_content
        .find("## Impl Blocks (private)")
        .expect("Should have private impl blocks");
    let pub_greet = index_content
        .find("pub fn greet(&self) -> String;")
        .expect("Should have greet method");
    let priv_display_name = index_content
        .find("pub(crate) fn display_name(&self) -> &str;")
        .expect("Should have display_name method");
    assert!(
        pub_greet > pub_impl_greeter && pub_greet < separator_pos,
        "Public greet method should be in the public zone"
    );
    assert!(
        priv_display_name > priv_impl_greeter,
        "Private display_name method should be in the private zone"
    );

    // Verify impl Render for Greeter remains in the public zone
    let render_impl_pos = index_content
        .find("impl Render for Greeter")
        .expect("Should have Render impl");
    assert!(
        render_impl_pos > pub_impl_greeter && render_impl_pos < separator_pos,
        "impl Render for Greeter should be in the public impl zone"
    );

    // Verify impl group dividers
    assert!(
        index_content.contains("// Trait implementations"),
        "Internal view should have trait implementations divider"
    );
    assert!(
        index_content.contains("// Marker trait implementations"),
        "Internal view should have marker trait implementations divider"
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
        !module_content.contains("// -- private --"),
        "Internal module view should not have old private separator"
    );
    assert!(
        module_content.contains("---\n"),
        "Internal module view should have page-level separator"
    );
    assert!(
        module_content.contains("## Functions (private)"),
        "Internal module view should have private functions header"
    );
    assert!(
        module_content.contains("## Constants (private)"),
        "Internal module view should have private constants header"
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

// ===========================================================================
// Workspace fixture: multi-crate with derives, associated types, nested modules
// ===========================================================================

#[test]
fn test_workspace_core_matches_golden() {
    let out_dir = generate_and_compare(
        "workspace",
        "workspace-core",
        &["index.md", "index.internal.md"],
        Some("core"),
    );

    let surface = read_file(&out_dir.join("index.md"));

    // Derived trait impls appear in surface view
    assert!(
        surface.contains("impl Clone for Id;"),
        "Surface should show derived Clone for Id"
    );
    assert!(
        surface.contains("impl Debug for Id;"),
        "Surface should show derived Debug for Id"
    );
    assert!(
        surface.contains("impl Hash for Id;"),
        "Surface should show derived Hash for Id"
    );
    assert!(
        surface.contains("impl Clone for User;"),
        "Surface should show derived Clone for User"
    );
    assert!(
        surface.contains("impl PartialEq for Status;"),
        "Surface should show derived PartialEq for Status"
    );

    // Traits rendered
    assert!(
        surface.contains("pub trait Repository;"),
        "Surface should include Repository trait"
    );
    assert!(
        surface.contains("pub trait Describable;"),
        "Surface should include Describable trait"
    );

    // Type alias and constant
    assert!(
        surface.contains("pub type Timestamp = u64;"),
        "Surface should include Timestamp type alias"
    );
    assert!(
        surface.contains("pub const DEFAULT_PAGE_SIZE: usize"),
        "Surface should include DEFAULT_PAGE_SIZE constant"
    );

    let internal = read_file(&out_dir.join("index.internal.md"));

    // Associated types rendered in trait definition
    assert!(
        internal.contains("pub trait Repository"),
        "Internal should include Repository trait"
    );

    // Derived trait implementations divider
    assert!(
        internal.contains("// Derived trait implementations"),
        "Internal should have derived trait implementations divider"
    );

    // Derived impl bodies rendered
    assert!(
        internal.contains("impl Clone for Id"),
        "Internal should show derived Clone impl for Id"
    );
    assert!(
        internal.contains("impl Debug for Id"),
        "Internal should show derived Debug impl for Id"
    );
    assert!(
        internal.contains("impl Hash for Id"),
        "Internal should show derived Hash impl for Id"
    );

    // User struct with private field
    assert!(
        internal.contains("email: String"),
        "Internal should show private email field on User"
    );

    // Describable trait default method
    assert!(
        internal.contains("fn describe(&self) -> String;"),
        "Internal should show Describable default method"
    );
}

#[test]
fn test_workspace_service_matches_golden() {
    let out_dir = generate_and_compare(
        "workspace",
        "workspace-service",
        &[
            "index.md",
            "index.internal.md",
            "module.service.handler.md",
            "module.service.handler.internal.md",
            "module.service.handler.auth.md",
            "module.service.handler.auth.internal.md",
        ],
        Some("service"),
    );

    let surface = read_file(&out_dir.join("index.md"));

    // Cross-crate trait impls in surface view
    assert!(
        surface.contains("impl Describable for UserStore;"),
        "Surface should show cross-crate Describable impl"
    );
    assert!(
        surface.contains("impl Repository for UserStore;"),
        "Surface should show cross-crate Repository impl"
    );

    // Module table with nested handler module
    assert!(
        surface.contains("[`handler`](module.service.handler.md)"),
        "Surface should link to handler module"
    );

    // Private struct in surface
    assert!(
        surface.contains("pub(crate) struct ServiceConfig;"),
        "Surface should show private ServiceConfig"
    );

    let internal = read_file(&out_dir.join("index.internal.md"));

    // Cross-crate trait impl with associated type equality
    assert!(
        internal.contains("impl Repository for UserStore"),
        "Internal should show Repository impl for UserStore"
    );

    // Private struct in private zone
    assert!(
        internal.contains("## Structs (private)"),
        "Internal should have private structs section"
    );
    assert!(
        internal.contains("pub(crate) struct ServiceConfig"),
        "Internal should show ServiceConfig in private zone"
    );
    assert!(
        internal.contains("max_concurrent: usize"),
        "Internal should show ServiceConfig private fields"
    );

    // Two-zone layout
    assert!(
        internal.contains("---\n"),
        "Internal should have page-level separator"
    );

    // 3-level deep module nesting
    let handler = read_file(&out_dir.join("module.service.handler.md"));
    assert!(
        handler.contains("# Module `service::handler`"),
        "Handler module should have correct title"
    );
    assert!(
        handler.contains("[`auth`](module.service.handler.auth.md)"),
        "Handler module should link to nested auth module"
    );

    let auth = read_file(&out_dir.join("module.service.handler.auth.md"));
    assert!(
        auth.contains("# Module `service::handler::auth`"),
        "Auth module should have correct 3-level path in title"
    );
    assert!(
        auth.contains("pub struct Token;"),
        "Auth surface should show Token struct"
    );
    assert!(
        auth.contains("pub fn login("),
        "Auth surface should show login function"
    );
    assert!(
        auth.contains("pub fn verify("),
        "Auth surface should show verify function"
    );
    assert!(
        !auth.contains("revoke"),
        "Auth surface should not show private revoke function"
    );

    let auth_internal = read_file(&out_dir.join("module.service.handler.auth.internal.md"));
    assert!(
        auth_internal.contains("## Functions (private)"),
        "Auth internal should have private functions section"
    );
    assert!(
        auth_internal.contains("revoke"),
        "Auth internal should include private revoke function"
    );
    assert!(
        auth_internal.contains("pub value: String"),
        "Auth internal should show Token fields"
    );

    // Handler internal shows private log_event
    let handler_internal = read_file(&out_dir.join("module.service.handler.internal.md"));
    assert!(
        handler_internal.contains("log_event"),
        "Handler internal should include private log_event function"
    );
    assert!(
        handler_internal.contains("## Functions (private)"),
        "Handler internal should have private functions section"
    );
}

// ===========================================================================
// Advanced fixture: lifetimes, where clauses, async, const generics, etc.
// ===========================================================================

#[test]
fn test_advanced_fixture_matches_golden() {
    let out_dir = generate_and_compare(
        "advanced",
        "advanced",
        &["index.md", "index.internal.md"],
        None,
    );

    let surface = read_file(&out_dir.join("index.md"));

    // Lifetimes
    assert!(
        surface.contains("pub struct Parser<'a>;"),
        "Surface should show Parser with lifetime parameter"
    );
    assert!(
        surface.contains("pub struct Pair<'k, 'v>;"),
        "Surface should show Pair with multiple lifetime parameters"
    );

    // Where clauses
    assert!(
        surface.contains("pub fn serialize<T>(value: &T) -> String"),
        "Surface should show serialize function"
    );
    assert!(
        surface.contains("T: Display + Clone,"),
        "Surface should show where clause on serialize"
    );
    assert!(
        surface.contains("pub struct Sendable<T>"),
        "Surface should show Sendable struct"
    );

    // Async function
    assert!(
        surface.contains("pub async fn fetch(url: &str) -> Result<String, String>;"),
        "Surface should show async function with async keyword"
    );

    // Const generics
    assert!(
        surface.contains("pub struct Buffer<const N: usize>;"),
        "Surface should show Buffer with const generic"
    );

    // impl Trait return type
    assert!(
        surface.contains("pub fn create_greeting() -> impl Display;"),
        "Surface should show impl Trait return type"
    );

    // dyn Trait parameter type
    assert!(
        surface.contains("pub fn debug_format(value: &dyn Debug) -> String;"),
        "Surface should show dyn Trait parameter type"
    );

    // Undocumented items still appear
    assert!(
        surface.contains("pub struct Undocumented;"),
        "Surface should include undocumented struct"
    );
    assert!(
        surface.contains("pub fn no_docs(x: i32) -> i32;"),
        "Surface should include undocumented function"
    );
    assert!(
        surface.contains("pub enum Bare;"),
        "Surface should include undocumented enum"
    );
    assert!(
        surface.contains("pub trait Unmarked;"),
        "Surface should include undocumented trait"
    );
    assert!(
        surface.contains("pub const MAGIC: u32 = 42u32;"),
        "Surface should include undocumented constant"
    );
    assert!(
        surface.contains("pub type Pair2 = (String, String);"),
        "Surface should include undocumented type alias"
    );

    // Multiple impl blocks for Container
    assert!(
        surface.contains("impl<T: Default> Container<T>;"),
        "Surface should show Container impl with Default bound"
    );
    assert!(
        surface.contains("impl<T: Display + Clone> Container<T>;"),
        "Surface should show Container impl with Display + Clone bounds"
    );

    // Multiple impls for Multi
    assert!(
        surface.contains("impl Multi;"),
        "Surface should show inherent impl for Multi"
    );
    assert!(
        surface.contains("impl Display for Multi;"),
        "Surface should show Display impl for Multi"
    );

    // Lifetime on trait
    assert!(
        surface.contains("pub trait Processor<'a>;"),
        "Surface should show trait with lifetime parameter"
    );

    // Lifetime on impl block
    assert!(
        surface.contains("impl<'a> Parser<'a>;"),
        "Surface should show impl block with lifetime"
    );

    // Const generic impl
    assert!(
        surface.contains("impl<const N: usize> Buffer<"),
        "Surface should show impl block with const generic"
    );

    // Private items
    assert!(
        surface.contains("pub(crate) struct HiddenUndocumented;"),
        "Surface should show private undocumented struct"
    );
    assert!(
        surface.contains("pub(crate) fn hidden_no_docs() -> bool;"),
        "Surface should show private undocumented function"
    );
    assert!(
        surface.contains("pub(crate) async fn resolve(name: &str) -> String;"),
        "Surface should show private async function"
    );
}

#[test]
fn test_advanced_internal_view() {
    let out_dir = test_output_dir("advanced");
    // The previous test already generated this; if running standalone, generate again
    if !out_dir.join("index.internal.md").exists() {
        generate_and_compare(
            "advanced",
            "advanced",
            &["index.md", "index.internal.md"],
            None,
        );
    }

    let internal = read_file(&out_dir.join("index.internal.md"));

    // Lifetime struct with private field visible
    assert!(
        internal.contains("pub struct Parser<'a>"),
        "Internal should show Parser with lifetime"
    );
    assert!(
        internal.contains("cursor: usize"),
        "Internal should show private cursor field on Parser"
    );
    assert!(
        internal.contains("pub input: &'a str"),
        "Internal should show borrowed field with lifetime"
    );

    // Multi-lifetime struct
    assert!(
        internal.contains("pub struct Pair<'k, 'v>"),
        "Internal should show Pair with multiple lifetimes"
    );
    assert!(
        internal.contains("pub key: &'k str"),
        "Internal should show key field with lifetime"
    );

    // Where clause on struct
    assert!(
        internal.contains("pub struct Sendable<T>"),
        "Internal should show Sendable struct"
    );
    assert!(
        internal.contains("T: Send + Sync,"),
        "Internal should show where clause on Sendable"
    );

    // Const generic struct
    assert!(
        internal.contains("pub struct Buffer<const N: usize>"),
        "Internal should show Buffer with const generic"
    );
    assert!(
        internal.contains("pub data: [u8; N]"),
        "Internal should show const-generic array field"
    );

    // Function pointer field types
    assert!(
        internal.contains("pub struct Callback"),
        "Internal should show Callback struct"
    );
    assert!(
        internal.contains("handler: fn("),
        "Internal should show function pointer field"
    );
    assert!(
        internal.contains("finalizer: Option<fn()>"),
        "Internal should show Option<fn()> field"
    );

    // Async function
    assert!(
        internal.contains("pub async fn fetch(url: &str) -> Result<String, String>;"),
        "Internal should show async function"
    );

    // impl Trait return type
    assert!(
        internal.contains("pub fn create_greeting() -> impl Display;"),
        "Internal should show impl Trait return type"
    );

    // dyn Trait parameter
    assert!(
        internal.contains("pub fn debug_format(value: &dyn Debug) -> String;"),
        "Internal should show dyn Trait parameter"
    );

    // Where clause on function
    assert!(
        internal.contains("pub fn serialize<T>(value: &T) -> String"),
        "Internal should show serialize function"
    );

    // Undocumented items have no doc comment lines
    // (Undocumented struct: no "///" before its declaration in internal view)
    let undoc_pos = internal
        .find("### `Undocumented`")
        .expect("Should have Undocumented section");
    let next_section = internal[undoc_pos + 20..]
        .find("### ")
        .or_else(|| internal[undoc_pos + 20..].find("## "))
        .map(|p| p + undoc_pos + 20)
        .unwrap_or(internal.len());
    let undoc_section = &internal[undoc_pos..next_section];
    assert!(
        !undoc_section.contains("///"),
        "Undocumented struct section should have no doc comment lines"
    );

    // Bare enum has no doc comments
    let bare_pos = internal
        .find("### `Bare`")
        .expect("Should have Bare section");
    let bare_next = internal[bare_pos + 10..]
        .find("### ")
        .or_else(|| internal[bare_pos + 10..].find("## "))
        .map(|p| p + bare_pos + 10)
        .unwrap_or(internal.len());
    let bare_section = &internal[bare_pos..bare_next];
    assert!(
        !bare_section.contains("///"),
        "Bare enum section should have no doc comment lines"
    );

    // Unmarked trait has no doc comments
    let unmarked_pos = internal
        .find("### `Unmarked`")
        .expect("Should have Unmarked section");
    let unmarked_next = internal[unmarked_pos + 14..]
        .find("### ")
        .or_else(|| internal[unmarked_pos + 14..].find("## "))
        .map(|p| p + unmarked_pos + 14)
        .unwrap_or(internal.len());
    let unmarked_section = &internal[unmarked_pos..unmarked_next];
    assert!(
        !unmarked_section.contains("///"),
        "Unmarked trait section should have no doc comment lines"
    );

    // Generic impl blocks with bounds
    assert!(
        internal.contains("impl<T: Display + Clone> Container<T>"),
        "Internal should show Container impl with Display + Clone bounds"
    );
    assert!(
        internal.contains("impl<T: Default> Container<T>"),
        "Internal should show Container impl with Default bound"
    );

    // Multiple impl blocks for Multi
    assert!(
        internal.contains("impl Multi"),
        "Internal should show inherent impl for Multi"
    );
    assert!(
        internal.contains("impl Display for Multi"),
        "Internal should show Display impl for Multi"
    );

    // Trait implementations divider
    assert!(
        internal.contains("// Trait implementations"),
        "Internal should have trait implementations divider"
    );

    // Lifetime on impl block
    assert!(
        internal.contains("impl<'a> Parser<'a>"),
        "Internal should show impl block with lifetime"
    );

    // Const generic impl block
    assert!(
        internal.contains("impl<const N: usize> Buffer<"),
        "Internal should show impl block with const generic"
    );

    // Two-zone layout
    assert!(
        internal.contains("---\n"),
        "Internal should have page-level separator"
    );

    // Private zone items
    assert!(
        internal.contains("## Structs (private)"),
        "Internal should have private structs section"
    );
    assert!(
        internal.contains("pub(crate) struct HiddenUndocumented"),
        "Internal should show HiddenUndocumented in private zone"
    );
    assert!(
        internal.contains("## Functions (private)"),
        "Internal should have private functions section"
    );
    assert!(
        internal.contains("pub(crate) fn hidden_no_docs() -> bool;"),
        "Internal should show hidden_no_docs in private zone"
    );
    assert!(
        internal.contains("pub(crate) async fn resolve(name: &str) -> String;"),
        "Internal should show private async fn in private zone"
    );

    // Verify two-zone ordering: public items before separator, private after
    let separator_pos = internal
        .find("\n---\n")
        .expect("Should have page-level separator");
    let pub_structs_pos = internal
        .find("## Structs\n")
        .expect("Should have public structs");
    let priv_structs_pos = internal
        .find("## Structs (private)")
        .expect("Should have private structs");
    assert!(
        pub_structs_pos < separator_pos,
        "Public structs should appear before the separator"
    );
    assert!(
        separator_pos < priv_structs_pos,
        "Private structs should appear after the separator"
    );
}
