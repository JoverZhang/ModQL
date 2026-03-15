# Module `modql::convert`

[Internal view](module.modql.convert.internal.md)

## Types

```rust
pub enum ConvertMode;
```

## Functions

```rust
pub fn convert(krate: &Crate, mode: ConvertMode) -> Result<CrateDoc>;

pub fn render_type(ty: &Type) -> String;

```

