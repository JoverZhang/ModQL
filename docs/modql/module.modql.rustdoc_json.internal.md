# Internal Module `modql::rustdoc_json`

[Surface view](module.modql.rustdoc_json.md)

## Structs

### `PackageInfo`

Metadata about a single Cargo package extracted from `cargo metadata`.

```rust
pub struct PackageInfo {
    pub name: String,
    pub lib_target: Option<String>,
    pub bin_target: Option<String>,
}
```

#### Fields

- `name`: The package name (e.g. `"mira-cli"`).
- `lib_target`: The library target name, if present (e.g. `"mira_core"`).
- `bin_target`: The first binary target name, if present (e.g. `"mira"`).

### `RustdocOptions`

Options for generating rustdoc JSON.

```rust
pub struct RustdocOptions {
    pub manifest_path: PathBuf,
    pub nightly: String,
}
```

### `WorkspaceInfo`

Information about the workspace / package layout.

```rust
pub struct WorkspaceInfo {
    pub is_workspace: bool,
    pub packages: Vec<PackageInfo>,
}
```

#### Fields

- `is_workspace`: True when the manifest defines a `[workspace]` with multiple members.
- `packages`: Packages discovered via `cargo metadata`.

## Impl Blocks

### `impl PackageInfo`

```rust
impl PackageInfo {
    /// The target name that will be documented by `cargo rustdoc`.
    /// Prefers the lib target; falls back to the first bin target.
    pub fn doc_target_name(&self) -> Option<&str>;

}
```

// Marker trait implementations

### `impl Send for PackageInfo`

```rust
impl Send for PackageInfo;
```

### `impl Sync for PackageInfo`

```rust
impl Sync for PackageInfo;
```

### `impl Unpin for PackageInfo`

```rust
impl Unpin for PackageInfo;
```

### `impl Send for RustdocOptions`

```rust
impl Send for RustdocOptions;
```

### `impl Sync for RustdocOptions`

```rust
impl Sync for RustdocOptions;
```

### `impl Unpin for RustdocOptions`

```rust
impl Unpin for RustdocOptions;
```

### `impl Send for WorkspaceInfo`

```rust
impl Send for WorkspaceInfo;
```

### `impl Sync for WorkspaceInfo`

```rust
impl Sync for WorkspaceInfo;
```

### `impl Unpin for WorkspaceInfo`

```rust
impl Unpin for WorkspaceInfo;
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

