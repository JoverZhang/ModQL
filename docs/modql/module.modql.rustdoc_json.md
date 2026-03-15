# Module `modql::rustdoc_json`

[Internal view](module.modql.rustdoc_json.internal.md)

## Types

```rust
pub struct RustdocOptions;
pub struct WorkspaceInfo;
```

## Functions

```rust
pub fn generate_rustdoc_json(opts: &RustdocOptions, package: Option<&str>) -> Result<Crate>;

pub fn resolve_workspace_info(manifest_path: &Path) -> Result<WorkspaceInfo>;

```

