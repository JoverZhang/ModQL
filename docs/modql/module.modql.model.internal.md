# Internal Module `modql::model`

[Surface view](module.modql.model.md)

## Structs

### `ConstantDoc`

A constant with its type and value.

```rust
pub struct ConstantDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub is_public: bool,
}
```

### `CrateDoc`

Internal documentation model, independent from rustdoc JSON types.

This model represents the documentation structure that will be rendered
to Markdown. It is produced by the conversion layer from rustdoc JSON.
Top-level crate documentation.

```rust
pub struct CrateDoc {
    pub name: String,
    pub docs: Option<String>,
    pub modules: Vec<ModuleDoc>,
    pub impls: Vec<ImplDoc>,
    pub structs: Vec<StructDoc>,
    pub enums: Vec<EnumDoc>,
    pub traits: Vec<TraitDoc>,
    pub functions: Vec<FunctionDoc>,
    pub type_aliases: Vec<TypeAliasDoc>,
    pub constants: Vec<ConstantDoc>,
    pub statics: Vec<StaticDoc>,
}
```

### `EnumDoc`

An enum with its variants, inherent methods, and signature.

```rust
pub struct EnumDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub variants: Vec<VariantDoc>,
    pub is_public: bool,
}
```

### `FieldDoc`

A struct field with its type and documentation.

```rust
pub struct FieldDoc {
    pub name: String,
    pub type_str: String,
    pub docs: Option<String>,
    pub is_public: bool,
}
```

### `FunctionDoc`

A free function (not a method).

```rust
pub struct FunctionDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub is_public: bool,
}
```

### `ImplDoc`

An impl block defined in a module.

```rust
pub struct ImplDoc {
    pub header: String,
    pub docs: Option<String>,
    pub methods: Vec<MethodDoc>,
    pub target_name: String,
}
```

### `MethodDoc`

A method belonging to a type or trait.

```rust
pub struct MethodDoc {
    pub name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub is_public: bool,
}
```

### `ModuleDoc`

A module and its contents.

```rust
pub struct ModuleDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub modules: Vec<ModuleDoc>,
    pub impls: Vec<ImplDoc>,
    pub structs: Vec<StructDoc>,
    pub enums: Vec<EnumDoc>,
    pub traits: Vec<TraitDoc>,
    pub functions: Vec<FunctionDoc>,
    pub type_aliases: Vec<TypeAliasDoc>,
    pub constants: Vec<ConstantDoc>,
    pub statics: Vec<StaticDoc>,
}
```

### `StaticDoc`

A static variable with its type.

```rust
pub struct StaticDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub is_public: bool,
}
```

### `StructDoc`

A struct with its fields, inherent methods, and signature.

```rust
pub struct StructDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub fields: Vec<FieldDoc>,
    pub is_public: bool,
}
```

### `TraitDoc`

A trait with its required and provided methods.

```rust
pub struct TraitDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub methods: Vec<MethodDoc>,
    pub is_public: bool,
}
```

### `TypeAliasDoc`

A type alias with its definition.

```rust
pub struct TypeAliasDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub is_public: bool,
}
```

### `VariantDoc`

An enum variant with its documentation.

```rust
pub struct VariantDoc {
    pub name: String,
    pub docs: Option<String>,
    pub kind: VariantKind,
}
```

## Enums

### `VariantKind`

Kind of enum variant.

```rust
pub enum VariantKind {
    Plain,
    Tuple(Vec<String>),
    Struct(Vec<FieldDoc>),
}
```

#### Variants

- `Plain`: A plain variant with no data (e.g. `Foo`).
- `Tuple`: A tuple variant (e.g. `Foo(u32, String)`).
- `Struct`: A struct variant (e.g. `Foo { bar: u32 }`).

## Impl Blocks

### `impl Clone for ConstantDoc`

```rust
impl Clone for ConstantDoc {
    fn clone(&self) -> ConstantDoc;

}
```

### `impl Debug for ConstantDoc`

```rust
impl Debug for ConstantDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Clone for CrateDoc`

```rust
impl Clone for CrateDoc {
    fn clone(&self) -> CrateDoc;

}
```

### `impl Debug for CrateDoc`

```rust
impl Debug for CrateDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl ItemContainer for CrateDoc`

```rust
impl ItemContainer for CrateDoc {
    fn constants_mut(&mut self) -> &mut Vec<ConstantDoc>;

    fn enums_mut(&mut self) -> &mut Vec<EnumDoc>;

    fn functions_mut(&mut self) -> &mut Vec<FunctionDoc>;

    fn impls_mut(&mut self) -> &mut Vec<ImplDoc>;

    fn modules_mut(&mut self) -> &mut Vec<ModuleDoc>;

    fn statics_mut(&mut self) -> &mut Vec<StaticDoc>;

    fn structs_mut(&mut self) -> &mut Vec<StructDoc>;

    fn traits_mut(&mut self) -> &mut Vec<TraitDoc>;

    fn type_aliases_mut(&mut self) -> &mut Vec<TypeAliasDoc>;

}
```

### `impl Clone for EnumDoc`

```rust
impl Clone for EnumDoc {
    fn clone(&self) -> EnumDoc;

}
```

### `impl Debug for EnumDoc`

```rust
impl Debug for EnumDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Clone for FieldDoc`

```rust
impl Clone for FieldDoc {
    fn clone(&self) -> FieldDoc;

}
```

### `impl Debug for FieldDoc`

```rust
impl Debug for FieldDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Clone for FunctionDoc`

```rust
impl Clone for FunctionDoc {
    fn clone(&self) -> FunctionDoc;

}
```

### `impl Debug for FunctionDoc`

```rust
impl Debug for FunctionDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Clone for ImplDoc`

```rust
impl Clone for ImplDoc {
    fn clone(&self) -> ImplDoc;

}
```

### `impl Debug for ImplDoc`

```rust
impl Debug for ImplDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Clone for MethodDoc`

```rust
impl Clone for MethodDoc {
    fn clone(&self) -> MethodDoc;

}
```

### `impl Debug for MethodDoc`

```rust
impl Debug for MethodDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Clone for ModuleDoc`

```rust
impl Clone for ModuleDoc {
    fn clone(&self) -> ModuleDoc;

}
```

### `impl Debug for ModuleDoc`

```rust
impl Debug for ModuleDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl ItemContainer for ModuleDoc`

```rust
impl ItemContainer for ModuleDoc {
    fn constants_mut(&mut self) -> &mut Vec<ConstantDoc>;

    fn enums_mut(&mut self) -> &mut Vec<EnumDoc>;

    fn functions_mut(&mut self) -> &mut Vec<FunctionDoc>;

    fn impls_mut(&mut self) -> &mut Vec<ImplDoc>;

    fn modules_mut(&mut self) -> &mut Vec<ModuleDoc>;

    fn statics_mut(&mut self) -> &mut Vec<StaticDoc>;

    fn structs_mut(&mut self) -> &mut Vec<StructDoc>;

    fn traits_mut(&mut self) -> &mut Vec<TraitDoc>;

    fn type_aliases_mut(&mut self) -> &mut Vec<TypeAliasDoc>;

}
```

### `impl Clone for StaticDoc`

```rust
impl Clone for StaticDoc {
    fn clone(&self) -> StaticDoc;

}
```

### `impl Debug for StaticDoc`

```rust
impl Debug for StaticDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Clone for StructDoc`

```rust
impl Clone for StructDoc {
    fn clone(&self) -> StructDoc;

}
```

### `impl Debug for StructDoc`

```rust
impl Debug for StructDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Clone for TraitDoc`

```rust
impl Clone for TraitDoc {
    fn clone(&self) -> TraitDoc;

}
```

### `impl Debug for TraitDoc`

```rust
impl Debug for TraitDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Clone for TypeAliasDoc`

```rust
impl Clone for TypeAliasDoc {
    fn clone(&self) -> TypeAliasDoc;

}
```

### `impl Debug for TypeAliasDoc`

```rust
impl Debug for TypeAliasDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Clone for VariantDoc`

```rust
impl Clone for VariantDoc {
    fn clone(&self) -> VariantDoc;

}
```

### `impl Debug for VariantDoc`

```rust
impl Debug for VariantDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Clone for VariantKind`

```rust
impl Clone for VariantKind {
    fn clone(&self) -> VariantKind;

}
```

### `impl Debug for VariantKind`

```rust
impl Debug for VariantKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

