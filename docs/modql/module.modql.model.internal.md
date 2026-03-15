# Internal Module `modql::model`

[Surface view](module.modql.model.md)

## Structs

```rust
/// A constant with its type and value.
pub struct ConstantDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub is_public: bool,
}
```

```rust
/// Internal documentation model, independent from rustdoc JSON types.
///
/// This model represents the documentation structure that will be rendered
/// to Markdown. It is produced by the conversion layer from rustdoc JSON.
/// Top-level crate documentation.
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

```rust
/// An enum with its variants, inherent methods, and signature.
pub struct EnumDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub variants: Vec<VariantDoc>,
    pub is_public: bool,
    pub derived_traits: Vec<String>,
}
```

```rust
/// A struct field with its type and documentation.
pub struct FieldDoc {
    pub name: String,
    pub type_str: String,
    pub docs: Option<String>,
    pub is_public: bool,
}
```

```rust
/// A free function (not a method).
pub struct FunctionDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub is_public: bool,
}
```

```rust
/// An impl block defined in a module.
pub struct ImplDoc {
    pub header: String,
    pub docs: Option<String>,
    pub methods: Vec<MethodDoc>,
    pub target_name: String,
    pub target_is_public: bool,
    pub trait_name: Option<String>,
    pub trait_is_public: Option<bool>,
}
```

```rust
/// A method belonging to a type or trait.
pub struct MethodDoc {
    pub name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub is_public: bool,
}
```

```rust
/// A module and its contents.
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

```rust
/// A static variable with its type.
pub struct StaticDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub is_public: bool,
}
```

```rust
/// A struct with its fields, inherent methods, and signature.
pub struct StructDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub fields: Vec<FieldDoc>,
    pub is_public: bool,
    pub derived_traits: Vec<String>,
}
```

```rust
/// A trait with its required and provided methods.
pub struct TraitDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub methods: Vec<MethodDoc>,
    pub is_public: bool,
}
```

```rust
/// A type alias with its definition.
pub struct TypeAliasDoc {
    pub qualified_name: String,
    pub docs: Option<String>,
    pub signature: String,
    pub is_public: bool,
}
```

```rust
/// An enum variant with its documentation.
pub struct VariantDoc {
    pub name: String,
    pub docs: Option<String>,
    pub kind: VariantKind,
}
```

## Enums

```rust
/// Kind of enum variant.
pub enum VariantKind {
    Plain,
    Tuple(Vec<String>),
    Struct(Vec<FieldDoc>),
}
```

---

