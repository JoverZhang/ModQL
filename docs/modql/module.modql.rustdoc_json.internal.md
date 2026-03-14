# Internal Module `modql::rustdoc_json`

[Surface view](module.modql.rustdoc_json.md)

## Structs

### `RustdocOptions`

Options for generating rustdoc JSON.

```rust
pub struct RustdocOptions {
    pub manifest_path: PathBuf,
    pub package: Option<String>,
    pub nightly: String,
}
```

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

## Functions

```rust
/// Run `cargo +<nightly> rustdoc` and return the deserialized `rustdoc_types::Crate`.
pub fn generate_rustdoc_json(opts: &RustdocOptions) -> Result<Crate>;

```

---

## Functions (private)

```rust
/// Invoke `cargo +<nightly> rustdoc` with the appropriate flags.
pub(in ::rustdoc_json) fn invoke_cargo_rustdoc(opts: &RustdocOptions) -> Result<()>;

/// Find the rustdoc JSON file in the target directory.
pub(in ::rustdoc_json) fn locate_json(target_dir: &Path, crate_name: &str) -> Result<PathBuf>;

/// Read the JSON file and deserialize into `rustdoc_types::Crate`.
pub(in ::rustdoc_json) fn read_and_parse(json_path: &Path) -> Result<Crate>;

/// Use `cargo metadata` to find the target directory and crate name.
pub(in ::rustdoc_json) fn resolve_metadata(opts: &RustdocOptions) -> Result<(PathBuf, String)>;

```

