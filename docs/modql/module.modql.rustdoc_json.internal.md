# Internal Module `modql::rustdoc_json`

[Surface view](module.modql.rustdoc_json.md)

## Structs

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
    pub packages: Vec<String>,
}
```

#### Fields

- `is_workspace`: True when the manifest defines a `[workspace]` with multiple members.
- `packages`: Package names discovered via `cargo metadata`.

## Impl Blocks

// Marker trait implementations

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
///
/// When `package` is `Some`, the `--package` flag is forwarded to cargo.
pub fn generate_rustdoc_json(opts: &RustdocOptions, package: Option<&str>) -> Result<Crate>;

/// Inspect the manifest via `cargo metadata` and decide whether we are dealing
/// with a single crate or a workspace with multiple members.
pub fn resolve_workspace_info(manifest_path: &Path) -> Result<WorkspaceInfo>;

```

---

## Functions (private)

```rust
/// Invoke `cargo +<nightly> rustdoc` with the appropriate flags.
pub(in ::rustdoc_json) fn invoke_cargo_rustdoc(opts: &RustdocOptions, package: Option<&str>) -> Result<()>;

/// Find the rustdoc JSON file in the target directory.
pub(in ::rustdoc_json) fn locate_json(target_dir: &Path, crate_name: &str) -> Result<PathBuf>;

/// Read the JSON file and deserialize into `rustdoc_types::Crate`.
pub(in ::rustdoc_json) fn read_and_parse(json_path: &Path) -> Result<Crate>;

/// Use `cargo metadata` to find the target directory and crate name.
pub(in ::rustdoc_json) fn resolve_metadata(manifest_path: &Path, package: Option<&str>) -> Result<(PathBuf, String)>;

pub(in ::rustdoc_json) fn run_cargo_metadata(manifest_path: &Path) -> Result<Value>;

```

