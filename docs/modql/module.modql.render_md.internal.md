# Internal Module `modql::render_md`

[Surface view](module.modql.render_md.md)

## Enums

### `TypeKind`

```rust
pub(in ::render_md) enum TypeKind {
    Struct,
    Enum,
    Trait,
}
```

### `ViewKind`

```rust
pub(in ::render_md) enum ViewKind {
    Surface,
    Internal,
}
```

## Impl Blocks

### `impl Clone for TypeKind`

```rust
impl Clone for TypeKind {
    fn clone(&self) -> TypeKind;

}
```

### `impl Copy for TypeKind`

```rust
impl Copy for TypeKind;
```

### `impl Debug for TypeKind`

```rust
impl Debug for TypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Eq for TypeKind`

```rust
impl Eq for TypeKind;
```

### `impl PartialEq for TypeKind`

```rust
impl PartialEq for TypeKind {
    fn eq(&self, other: &TypeKind) -> bool;

}
```

### `impl StructuralPartialEq for TypeKind`

```rust
impl StructuralPartialEq for TypeKind;
```

### `impl Clone for ViewKind`

```rust
impl Clone for ViewKind {
    fn clone(&self) -> ViewKind;

}
```

### `impl Copy for ViewKind`

```rust
impl Copy for ViewKind;
```

### `impl Debug for ViewKind`

```rust
impl Debug for ViewKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Eq for ViewKind`

```rust
impl Eq for ViewKind;
```

### `impl PartialEq for ViewKind`

```rust
impl PartialEq for ViewKind {
    fn eq(&self, other: &ViewKind) -> bool;

}
```

### `impl StructuralPartialEq for ViewKind`

```rust
impl StructuralPartialEq for ViewKind;
```

### `impl ViewKind`

```rust
impl ViewKind {
    pub(in ::render_md) fn file_name_for_module(self, qualified_name: &str) -> String;

    pub(in ::render_md) fn title_prefix(self) -> &'static str;

}
```

## Functions

```rust
pub(in ::render_md) fn collect_module_names(modules: &[ModuleDoc]) -> BTreeSet<String>;

pub(in ::render_md) fn collect_module_names_into(modules: &[ModuleDoc], names: &mut BTreeSet<String>);

pub(in ::render_md) fn ensure_decl_terminated(signature: &str) -> String;

pub(in ::render_md) fn find_decl_delimiter(signature: &str) -> Option<usize>;

/// Render all documentation pages to the output directory.
pub fn render(surface_doc: &CrateDoc, internal_doc: &CrateDoc, out_dir: &Path) -> Result<()>;

pub(in ::render_md) fn render_body_sections(out: &mut String, crate_doc: &CrateDoc, view: ViewKind);

pub(in ::render_md) fn render_constants_section(out: &mut String, items: &[ConstantDoc]);

pub(in ::render_md) fn render_crate_page(crate_doc: &CrateDoc, view: ViewKind, surface_modules: &BTreeSet<String>) -> String;

pub(in ::render_md) fn render_docs_paragraph(out: &mut String, docs: &str);

pub(in ::render_md) fn render_enums_section(out: &mut String, enums: &[EnumDoc]);

pub(in ::render_md) fn render_field_notes(out: &mut String, fields: &[FieldDoc]);

pub(in ::render_md) fn render_functions_section(out: &mut String, functions: &[FunctionDoc]);

pub(in ::render_md) fn render_impl_block(impl_doc: &ImplDoc) -> String;

pub(in ::render_md) fn render_impl_headers_section(out: &mut String, impls: &[ImplDoc]);

pub(in ::render_md) fn render_impls_section(out: &mut String, impls: &[ImplDoc]);

pub(in ::render_md) fn render_internal_sections(out: &mut String, structs: &[StructDoc], enums: &[EnumDoc], traits: &[TraitDoc], impls: &[ImplDoc], functions: &[FunctionDoc], type_aliases: &[TypeAliasDoc], constants: &[ConstantDoc], statics: &[StaticDoc]);

pub(in ::render_md) fn render_method_notes(out: &mut String, methods: &[MethodDoc]);

pub(in ::render_md) fn render_module_file(module: &ModuleDoc, out_dir: &Path, view: ViewKind, surface_modules: &BTreeSet<String>) -> Result<()>;

pub(in ::render_md) fn render_module_listing(out: &mut String, modules: &[ModuleDoc], view: ViewKind, surface_modules: &BTreeSet<String>);

pub(in ::render_md) fn render_module_page(module: &ModuleDoc, view: ViewKind, surface_modules: &BTreeSet<String>) -> String;

pub(in ::render_md) fn render_module_sections(out: &mut String, module: &ModuleDoc, view: ViewKind);

pub(in ::render_md) fn render_signature_block_section<'a, I>(out: &mut String, title: &str, items: I, include_docs: bool)
where
    I: IntoIterator<Item = (Option<&'a str>, &'a str)>,;

pub(in ::render_md) fn render_statics_section(out: &mut String, items: &[StaticDoc]);

pub(in ::render_md) fn render_structs_section(out: &mut String, structs: &[StructDoc]);

pub(in ::render_md) fn render_surface_sections(out: &mut String, structs: &[StructDoc], enums: &[EnumDoc], traits: &[TraitDoc], impls: &[ImplDoc], functions: &[FunctionDoc], type_aliases: &[TypeAliasDoc], constants: &[ConstantDoc], statics: &[StaticDoc]);

pub(in ::render_md) fn render_traits_section(out: &mut String, traits: &[TraitDoc]);

pub(in ::render_md) fn render_type_aliases_section(out: &mut String, items: &[TypeAliasDoc]);

pub(in ::render_md) fn render_types_summary_section(out: &mut String, structs: &[StructDoc], enums: &[EnumDoc], traits: &[TraitDoc]);

pub(in ::render_md) fn render_variant_notes(out: &mut String, variants: &[VariantDoc]);

pub(in ::render_md) fn render_view_link(out: &mut String, qualified_name: Option<&str>, view: ViewKind, has_surface: bool);

pub(in ::render_md) fn summarize_type_signature(signature: &str, kind: TypeKind) -> String;

pub(in ::render_md) fn synopsis_text(docs: &Option<String>) -> Option<String>;

pub(in ::render_md) fn write_doc_comments(out: &mut String, docs: Option<&str>, indent: &str);

pub(in ::render_md) fn write_page(out_dir: &Path, filename: &str, content: &str) -> Result<()>;

```

