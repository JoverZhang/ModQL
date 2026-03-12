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

## Impl Blocks

```rust
impl Clone for ConvertMode;
impl Copy for ConvertMode;
impl Debug for ConvertMode;
impl Eq for ConvertMode;
impl PartialEq for ConvertMode;
impl StructuralPartialEq for ConvertMode;
```

