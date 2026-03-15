# Internal Module `modql::rustdoc_json`

[Surface view](module.modql.rustdoc_json.md)

## Structs

```rust
/// Metadata about a single Cargo package extracted from `cargo metadata`.
pub struct PackageInfo {
    pub name: String,
    pub lib_target: Option<String>,
    pub bin_target: Option<String>,
}
```

```rust
/// Options for generating rustdoc JSON.
pub struct RustdocOptions {
    pub manifest_path: PathBuf,
    pub nightly: String,
}
```

```rust
/// Information about the workspace / package layout.
pub struct WorkspaceInfo {
    pub is_workspace: bool,
    pub packages: Vec<PackageInfo>,
}
```

## Impl

```rust
impl PackageInfo {
    /// The target name that will be documented by `cargo rustdoc`.
    /// Prefers the lib target; falls back to the first bin target.
    pub fn doc_target_name(&self) -> Option<&str>;

}
```

## Functions

```rust
/// Run `cargo +<nightly> rustdoc` for a single package and return the
/// deserialized `rustdoc_types::Crate`.
pub fn generate_rustdoc_json(opts: &RustdocOptions, pkg: &PackageInfo, is_workspace: bool) -> Result<Crate>;

/// Inspect the manifest via `cargo metadata` and decide whether we are dealing
/// with a single crate or a workspace with multiple members.
pub fn resolve_workspace_info(manifest_path: &Path) -> Result<WorkspaceInfo>;

```

---

## Functions (private)

```rust
/// Invoke `cargo +<nightly> rustdoc` with the appropriate flags.
pub(in ::rustdoc_json) fn invoke_cargo_rustdoc(opts: &RustdocOptions, pkg: &PackageInfo, is_workspace: bool) -> Result<()>;

/// Find the rustdoc JSON file in the target directory.
pub(in ::rustdoc_json) fn locate_json(target_dir: &Path, target_name: &str) -> Result<PathBuf>;

/// Read the JSON file and deserialize into `rustdoc_types::Crate`.
pub(in ::rustdoc_json) fn read_and_parse(json_path: &Path) -> Result<Crate>;

/// Extract just the target directory from `cargo metadata`.
pub(in ::rustdoc_json) fn resolve_target_dir(manifest_path: &Path) -> Result<PathBuf>;

pub(in ::rustdoc_json) fn run_cargo_metadata(manifest_path: &Path) -> Result<Value>;

```

