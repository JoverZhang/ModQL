# Internal Module `modql::render_md`

[Surface view](module.modql.render_md.md)

## Functions

```rust
/// Render all documentation pages to the output directory.
pub fn render(surface_doc: &CrateDoc, internal_doc: &CrateDoc, out_dir: &Path) -> Result<()>;

```

---

## Structs (private)

### `InternalZoneItems`

Items collected for a single zone (public or private).

```rust
pub(in ::render_md) struct InternalZoneItems<'a> {
    structs: Vec<&'a StructDoc>,
    enums: Vec<&'a EnumDoc>,
    traits: Vec<&'a TraitDoc>,
    impls: Vec<RenderedImplZone<'a>>,
    functions: Vec<&'a FunctionDoc>,
    type_aliases: Vec<&'a TypeAliasDoc>,
    constants: Vec<&'a ConstantDoc>,
    statics: Vec<&'a StaticDoc>,
}
```

### `InternalZones`

```rust
pub(in ::render_md) struct InternalZones<'a> {
    public: InternalZoneItems<'a>,
    private: InternalZoneItems<'a>,
}
```

### `RenderedImplZone`

A view of an impl block filtered to only certain methods.

```rust
pub(in ::render_md) struct RenderedImplZone<'a> {
    impl_doc: &'a ImplDoc,
    methods: Vec<&'a MethodDoc>,
}
```

## Enums (private)

### `ImplCategory`

Category for ordering impl blocks within a zone.

```rust
pub(in ::render_md) enum ImplCategory {
    Inherent,
    ManualTrait,
    DerivedTrait,
    MarkerTrait,
}
```

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

## Impl Blocks (private)

### `impl<'a> InternalZoneItems<'a>`

```rust
impl<'a> InternalZoneItems<'a> {
    pub(in ::render_md) fn has_any(&self) -> bool;

}
```

### `impl ViewKind`

```rust
impl ViewKind {
    pub(in ::render_md) fn file_name_for_module(self, qualified_name: &str) -> String;

    pub(in ::render_md) fn title_prefix(self) -> &'static str;

}
```

// Trait implementations

### `impl StructuralPartialEq for ImplCategory`

```rust
impl StructuralPartialEq for ImplCategory;
```

### `impl StructuralPartialEq for TypeKind`

```rust
impl StructuralPartialEq for TypeKind;
```

### `impl StructuralPartialEq for ViewKind`

```rust
impl StructuralPartialEq for ViewKind;
```

// Derived trait implementations

### `impl Clone for ImplCategory`

```rust
impl Clone for ImplCategory {
    fn clone(&self) -> ImplCategory;

}
```

### `impl Copy for ImplCategory`

```rust
impl Copy for ImplCategory;
```

### `impl Debug for ImplCategory`

```rust
impl Debug for ImplCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Eq for ImplCategory`

```rust
impl Eq for ImplCategory;
```

### `impl Ord for ImplCategory`

```rust
impl Ord for ImplCategory {
    fn cmp(&self, other: &ImplCategory) -> Ordering;

}
```

### `impl PartialEq for ImplCategory`

```rust
impl PartialEq for ImplCategory {
    fn eq(&self, other: &ImplCategory) -> bool;

}
```

### `impl PartialOrd for ImplCategory`

```rust
impl PartialOrd for ImplCategory {
    fn partial_cmp(&self, other: &ImplCategory) -> Option<Ordering>;

}
```

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

// Marker trait implementations

### `impl Send for ImplCategory`

```rust
impl Send for ImplCategory;
```

### `impl Sync for ImplCategory`

```rust
impl Sync for ImplCategory;
```

### `impl Unpin for ImplCategory`

```rust
impl Unpin for ImplCategory;
```

### `impl<'a> Send for InternalZoneItems<'a>`

```rust
impl<'a> Send for InternalZoneItems<'a>;
```

### `impl<'a> Sync for InternalZoneItems<'a>`

```rust
impl<'a> Sync for InternalZoneItems<'a>;
```

### `impl<'a> Unpin for InternalZoneItems<'a>`

```rust
impl<'a> Unpin for InternalZoneItems<'a>;
```

### `impl<'a> Send for InternalZones<'a>`

```rust
impl<'a> Send for InternalZones<'a>;
```

### `impl<'a> Sync for InternalZones<'a>`

```rust
impl<'a> Sync for InternalZones<'a>;
```

### `impl<'a> Unpin for InternalZones<'a>`

```rust
impl<'a> Unpin for InternalZones<'a>;
```

### `impl<'a> Send for RenderedImplZone<'a>`

```rust
impl<'a> Send for RenderedImplZone<'a>;
```

### `impl<'a> Sync for RenderedImplZone<'a>`

```rust
impl<'a> Sync for RenderedImplZone<'a>;
```

### `impl<'a> Unpin for RenderedImplZone<'a>`

```rust
impl<'a> Unpin for RenderedImplZone<'a>;
```

### `impl Send for TypeKind`

```rust
impl Send for TypeKind;
```

### `impl Sync for TypeKind`

```rust
impl Sync for TypeKind;
```

### `impl Unpin for TypeKind`

```rust
impl Unpin for TypeKind;
```

### `impl Send for ViewKind`

```rust
impl Send for ViewKind;
```

### `impl Sync for ViewKind`

```rust
impl Sync for ViewKind;
```

### `impl Unpin for ViewKind`

```rust
impl Unpin for ViewKind;
```

## Functions (private)

```rust
pub(in ::render_md) fn collect_module_names(modules: &[ModuleDoc]) -> BTreeSet<String>;

pub(in ::render_md) fn collect_module_names_into(modules: &[ModuleDoc], names: &mut BTreeSet<String>);

pub(in ::render_md) fn ensure_decl_terminated(signature: &str) -> String;

pub(in ::render_md) fn find_decl_delimiter(signature: &str) -> Option<usize>;

pub(in ::render_md) fn impl_category(impl_doc: &ImplDoc) -> ImplCategory;

pub(in ::render_md) fn partition_internal_page<'a>(structs: &'a [StructDoc], enums: &'a [EnumDoc], traits: &'a [TraitDoc], impls: &'a [ImplDoc], functions: &'a [FunctionDoc], type_aliases: &'a [TypeAliasDoc], constants: &'a [ConstantDoc], statics: &'a [StaticDoc]) -> InternalZones<'a>;

pub(in ::render_md) fn render_body_sections(out: &mut String, crate_doc: &CrateDoc, view: ViewKind);

pub(in ::render_md) fn render_crate_page(crate_doc: &CrateDoc, view: ViewKind, surface_modules: &BTreeSet<String>) -> String;

pub(in ::render_md) fn render_docs_paragraph(out: &mut String, docs: &str);

pub(in ::render_md) fn render_field_notes(out: &mut String, fields: &[FieldDoc]);

pub(in ::render_md) fn render_impl_block_from_zone(zone: &RenderedImplZone<'_>) -> String;

pub(in ::render_md) fn render_impl_headers_section(out: &mut String, impls: &[ImplDoc]);

pub(in ::render_md) fn render_internal_sections(out: &mut String, structs: &[StructDoc], enums: &[EnumDoc], traits: &[TraitDoc], impls: &[ImplDoc], functions: &[FunctionDoc], type_aliases: &[TypeAliasDoc], constants: &[ConstantDoc], statics: &[StaticDoc]);

pub(in ::render_md) fn render_internal_zone(out: &mut String, zone: &InternalZoneItems<'_>, is_private: bool);

pub(in ::render_md) fn render_method_notes(out: &mut String, methods: &[MethodDoc]);

pub(in ::render_md) fn render_module_file(module: &ModuleDoc, out_dir: &Path, view: ViewKind, surface_modules: &BTreeSet<String>) -> Result<()>;

pub(in ::render_md) fn render_module_listing(out: &mut String, modules: &[ModuleDoc], view: ViewKind, surface_modules: &BTreeSet<String>);

pub(in ::render_md) fn render_module_page(module: &ModuleDoc, view: ViewKind, surface_modules: &BTreeSet<String>) -> String;

pub(in ::render_md) fn render_module_sections(out: &mut String, module: &ModuleDoc, view: ViewKind);

pub(in ::render_md) fn render_signature_block_section<'a, I>(out: &mut String, title: &str, items: I, include_docs: bool)
where
    I: IntoIterator<Item = (Option<&'a str>, &'a str, bool)>,;

pub(in ::render_md) fn render_surface_sections(out: &mut String, structs: &[StructDoc], enums: &[EnumDoc], traits: &[TraitDoc], impls: &[ImplDoc], functions: &[FunctionDoc], type_aliases: &[TypeAliasDoc], constants: &[ConstantDoc], statics: &[StaticDoc]);

pub(in ::render_md) fn render_types_summary_section(out: &mut String, structs: &[StructDoc], enums: &[EnumDoc], traits: &[TraitDoc]);

pub(in ::render_md) fn render_variant_notes(out: &mut String, variants: &[VariantDoc]);

pub(in ::render_md) fn render_view_link(out: &mut String, qualified_name: Option<&str>, view: ViewKind, has_surface: bool);

pub(in ::render_md) fn render_zone_constants_section(out: &mut String, items: &[&ConstantDoc], is_private: bool);

pub(in ::render_md) fn render_zone_enums_section(out: &mut String, enums: &[&EnumDoc], is_private: bool);

pub(in ::render_md) fn render_zone_functions_section(out: &mut String, functions: &[&FunctionDoc], is_private: bool);

pub(in ::render_md) fn render_zone_impls_section(out: &mut String, impls: &[RenderedImplZone<'_>], is_private: bool);

pub(in ::render_md) fn render_zone_statics_section(out: &mut String, items: &[&StaticDoc], is_private: bool);

pub(in ::render_md) fn render_zone_structs_section(out: &mut String, structs: &[&StructDoc], is_private: bool);

pub(in ::render_md) fn render_zone_traits_section(out: &mut String, traits: &[&TraitDoc], is_private: bool);

pub(in ::render_md) fn render_zone_type_aliases_section(out: &mut String, items: &[&TypeAliasDoc], is_private: bool);

pub(in ::render_md) fn split_impls_for_internal_zones<'a>(impls: &'a [ImplDoc]) -> (Vec<RenderedImplZone<'a>>, Vec<RenderedImplZone<'a>>);

pub(in ::render_md) fn summarize_type_signature(signature: &str, kind: TypeKind) -> String;

pub(in ::render_md) fn synopsis_text(docs: &Option<String>) -> Option<String>;

pub(in ::render_md) fn write_doc_comments(out: &mut String, docs: Option<&str>, indent: &str);

pub(in ::render_md) fn write_page(out_dir: &Path, filename: &str, content: &str) -> Result<()>;

pub(in ::render_md) fn zone_section_title(base: &str, is_private_zone: bool) -> String;

```

## Constants (private)

```rust
/// Derived traits used for impl categorization.
pub(in ::render_md) const DERIVED_TRAITS: &[&str];

/// Marker/auto traits used for impl categorization.
pub(in ::render_md) const MARKER_TRAITS: &[&str];

```

