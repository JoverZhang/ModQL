# Crate `modql`

[Internal view](index.internal.md)

## Modules

| Module | Summary | Internal |
|---|---|---|
| [`cli`](module.modql.cli.md) |  | [internal](module.modql.cli.internal.md) |
| [`convert`](module.modql.convert.md) |  | [internal](module.modql.convert.internal.md) |
| [`model`](module.modql.model.md) |  | [internal](module.modql.model.internal.md) |
| [`naming`](module.modql.naming.md) |  | [internal](module.modql.naming.internal.md) |
| [`render_md`](module.modql.render_md.md) |  | [internal](module.modql.render_md.internal.md) |
| [`rustdoc_json`](module.modql.rustdoc_json.md) |  | [internal](module.modql.rustdoc_json.internal.md) |

## Functions

```rust
pub(crate) fn generate_for_package(opts: &RustdocOptions, pkg: &PackageInfo, out_dir: &Path, is_workspace: bool, prefix: &str) -> Result<()>;

pub(crate) fn main() -> Result<()>;

```

