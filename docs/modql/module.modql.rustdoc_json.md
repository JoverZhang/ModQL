# Module `modql::rustdoc_json`

[Internal view](module.modql.rustdoc_json.internal.md)

## Types

```rust
pub struct PackageInfo;
pub struct RustdocOptions;
pub struct WorkspaceInfo;
```

## Functions

```rust
pub fn generate_rustdoc_json(opts: &RustdocOptions, pkg: &PackageInfo, is_workspace: bool) -> Result<Crate>;

pub fn resolve_workspace_info(manifest_path: &Path) -> Result<WorkspaceInfo>;

```

## Impl

```rust
impl PackageInfo {
    pub fn doc_target_name(&self) -> Option<&str>;
}
```

