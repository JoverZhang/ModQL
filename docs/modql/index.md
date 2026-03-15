# Crate `modql`

## Modules

| Module | Summary | Surface |
|---|---|---|
| [`cli`](module.modql.cli.internal.md) |  | [surface](module.modql.cli.md) |
| [`convert`](module.modql.convert.internal.md) |  | [surface](module.modql.convert.md) |
| [`model`](module.modql.model.internal.md) |  | [surface](module.modql.model.md) |
| [`naming`](module.modql.naming.internal.md) |  | [surface](module.modql.naming.md) |
| [`render_md`](module.modql.render_md.internal.md) |  | [surface](module.modql.render_md.md) |
| [`rustdoc_json`](module.modql.rustdoc_json.internal.md) |  | [surface](module.modql.rustdoc_json.md) |

## Functions (private)

```rust
pub(crate) fn generate_for_package(opts: &RustdocOptions, pkg: &PackageInfo, out_dir: &Path, is_workspace: bool, prefix: &str) -> Result<()>;

pub(crate) fn main() -> Result<()>;

```

