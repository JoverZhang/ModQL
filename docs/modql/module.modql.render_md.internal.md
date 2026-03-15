# Internal Module `modql::render_md`

[Surface view](module.modql.render_md.md)

## Functions

```rust
/// Render all documentation pages to the output directory.
pub fn render(surface_doc: &CrateDoc, internal_doc: &CrateDoc, out_dir: &Path) -> Result<()>;

```

---

## Structs (private)

```rust
/// Items collected for a single zone (public or private).
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

```rust
pub(in ::render_md) struct InternalZones<'a> {
    public: InternalZoneItems<'a>,
    private: InternalZoneItems<'a>,
}
```

```rust
/// A view of an impl block filtered to only certain methods.
pub(in ::render_md) struct RenderedImplZone<'a> {
    impl_doc: &'a ImplDoc,
    methods: Vec<&'a MethodDoc>,
}
```

## Enums (private)

```rust
pub(in ::render_md) enum TypeKind {
    Struct,
    Enum,
}
```

```rust
impl StructuralPartialEq for TypeKind;
```

```rust
pub(in ::render_md) enum ViewKind {
    Surface,
    Internal,
}
```

```rust
impl StructuralPartialEq for ViewKind;
```

## Impl (private)

```rust
impl<'a> InternalZoneItems<'a> {
    pub(in ::render_md) fn has_any(&self) -> bool;

}
```

```rust
impl ViewKind {
    pub(in ::render_md) fn file_name_for_module(self, qualified_name: &str) -> String;

    pub(in ::render_md) fn title_prefix(self) -> &'static str;

}
```

## Functions (private)

```rust
pub(in ::render_md) fn collect_module_names(modules: &[ModuleDoc]) -> BTreeSet<String>;

pub(in ::render_md) fn collect_module_names_into(modules: &[ModuleDoc], names: &mut BTreeSet<String>);

pub(in ::render_md) fn ensure_decl_terminated(signature: &str) -> String;

pub(in ::render_md) fn find_decl_delimiter(signature: &str) -> Option<usize>;

pub(in ::render_md) fn partition_internal_page<'a>(structs: &'a [StructDoc], enums: &'a [EnumDoc], traits: &'a [TraitDoc], impls: &'a [ImplDoc], functions: &'a [FunctionDoc], type_aliases: &'a [TypeAliasDoc], constants: &'a [ConstantDoc], statics: &'a [StaticDoc]) -> InternalZones<'a>;

pub(in ::render_md) fn render_body_sections(out: &mut String, crate_doc: &CrateDoc, view: ViewKind);

pub(in ::render_md) fn render_crate_page(crate_doc: &CrateDoc, surface_modules: &BTreeSet<String>) -> String;

pub(in ::render_md) fn render_docs_paragraph(out: &mut String, docs: &str);

pub(in ::render_md) fn render_impl_block_from_zone(zone: &RenderedImplZone<'_>) -> String;

pub(in ::render_md) fn render_internal_sections(out: &mut String, structs: &[StructDoc], enums: &[EnumDoc], traits: &[TraitDoc], impls: &[ImplDoc], functions: &[FunctionDoc], type_aliases: &[TypeAliasDoc], constants: &[ConstantDoc], statics: &[StaticDoc]);

pub(in ::render_md) fn render_internal_zone(out: &mut String, zone: &InternalZoneItems<'_>, is_private: bool);

pub(in ::render_md) fn render_module_file(module: &ModuleDoc, out_dir: &Path, view: ViewKind, surface_modules: &BTreeSet<String>) -> Result<()>;

pub(in ::render_md) fn render_module_listing(out: &mut String, modules: &[ModuleDoc], view: ViewKind, surface_modules: &BTreeSet<String>);

pub(in ::render_md) fn render_module_page(module: &ModuleDoc, view: ViewKind, surface_modules: &BTreeSet<String>) -> String;

pub(in ::render_md) fn render_module_sections(out: &mut String, module: &ModuleDoc, view: ViewKind);

pub(in ::render_md) fn render_signature_block_section<'a, I>(out: &mut String, title: &str, items: I, include_docs: bool)
where
    I: IntoIterator<Item = (Option<&'a str>, &'a str, bool)>,;

/// Render inherent impl blocks (no trait impls) with methods expanded for surface view.
pub(in ::render_md) fn render_surface_inherent_impls_section(out: &mut String, impls: &[ImplDoc]);

pub(in ::render_md) fn render_surface_sections(out: &mut String, structs: &[StructDoc], enums: &[EnumDoc], traits: &[TraitDoc], impls: &[ImplDoc], functions: &[FunctionDoc], type_aliases: &[TypeAliasDoc], constants: &[ConstantDoc], statics: &[StaticDoc]);

pub(in ::render_md) fn render_types_summary_section(out: &mut String, structs: &[StructDoc], enums: &[EnumDoc], traits: &[TraitDoc]);

pub(in ::render_md) fn render_view_link(out: &mut String, qualified_name: &str, view: ViewKind, has_surface: bool);

pub(in ::render_md) fn render_zone_constants_section(out: &mut String, items: &[&ConstantDoc], is_private: bool);

pub(in ::render_md) fn render_zone_enums_section(out: &mut String, enums: &[&EnumDoc], impls: &[RenderedImplZone<'_>], is_private: bool);

pub(in ::render_md) fn render_zone_functions_section(out: &mut String, functions: &[&FunctionDoc], is_private: bool);

pub(in ::render_md) fn render_zone_impls_section(out: &mut String, impls: &[RenderedImplZone<'_>], is_private: bool);

pub(in ::render_md) fn render_zone_statics_section(out: &mut String, items: &[&StaticDoc], is_private: bool);

pub(in ::render_md) fn render_zone_structs_section(out: &mut String, structs: &[&StructDoc], impls: &[RenderedImplZone<'_>], is_private: bool);

pub(in ::render_md) fn render_zone_traits_section(out: &mut String, traits: &[&TraitDoc], is_private: bool);

pub(in ::render_md) fn render_zone_type_aliases_section(out: &mut String, items: &[&TypeAliasDoc], is_private: bool);

pub(in ::render_md) fn split_impls_for_internal_zones<'a>(impls: &'a [ImplDoc]) -> (Vec<RenderedImplZone<'a>>, Vec<RenderedImplZone<'a>>);

pub(in ::render_md) fn summarize_type_signature(signature: &str, kind: TypeKind) -> String;

pub(in ::render_md) fn synopsis_text(docs: &Option<String>) -> Option<String>;

pub(in ::render_md) fn write_doc_comments(out: &mut String, docs: Option<&str>, indent: &str);

pub(in ::render_md) fn write_page(out_dir: &Path, filename: &str, content: &str) -> Result<()>;

pub(in ::render_md) fn zone_section_title(base: &str, is_private_zone: bool) -> String;

```

