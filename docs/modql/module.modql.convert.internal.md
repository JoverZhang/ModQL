# Internal Module `modql::convert`

[Surface view](module.modql.convert.md)

## Enums

```rust
/// Controls which symbols are included in the converted document tree.
pub enum ConvertMode {
    Surface,
    Internal,
}
```

```rust
impl StructuralPartialEq for ConvertMode;
```

## Functions

```rust
/// Convert a rustdoc JSON `Crate` into our internal `CrateDoc` model.
pub fn convert(krate: &Crate, mode: ConvertMode) -> Result<CrateDoc>;

pub fn render_type(ty: &Type) -> String;

```

---

## Constants (private)

```rust
/// Derived traits that should be allowed through the synthetic/span filter.
pub(in ::convert) const DERIVED_TRAITS: &[&str];

/// Marker/auto traits that should be allowed through the synthetic/span filter.
pub(in ::convert) const MARKER_TRAITS: &[&str];

```

## Structs (private)

```rust
pub(in ::convert) struct CollectPolicy {
    include_private_items: bool,
    include_private_members: bool,
}
```

```rust
impl StructuralPartialEq for CollectPolicy;
```

## Traits (private)

```rust
/// Container trait for dispatching items into the right collection.
pub(in ::convert) trait ItemContainer {
    fn modules_mut(&mut self) -> &mut Vec<ModuleDoc>;
    fn impls_mut(&mut self) -> &mut Vec<ImplDoc>;
    fn structs_mut(&mut self) -> &mut Vec<StructDoc>;
    fn enums_mut(&mut self) -> &mut Vec<EnumDoc>;
    fn traits_mut(&mut self) -> &mut Vec<TraitDoc>;
    fn functions_mut(&mut self) -> &mut Vec<FunctionDoc>;
    fn type_aliases_mut(&mut self) -> &mut Vec<TypeAliasDoc>;
    fn constants_mut(&mut self) -> &mut Vec<ConstantDoc>;
    fn statics_mut(&mut self) -> &mut Vec<StaticDoc>;
}
```

## Impl (private)

```rust
impl ConvertMode {
    pub(in ::convert) fn include_private_items(self) -> bool;

    pub(in ::convert) fn include_private_members(self) -> bool;

    pub(in ::convert) fn nested_policy(self) -> CollectPolicy;

    pub(in ::convert) fn root_policy(self) -> CollectPolicy;

}
```

## Functions (private)

```rust
pub(in ::convert) fn collect_assoc_impl_docs(krate: &Crate, impl_ids: &[Id], mode: ConvertMode, policy: CollectPolicy) -> Vec<ImplDoc>;

/// Scan associated impl IDs and collect trait names for derived (synthetic) impls.
pub(in ::convert) fn collect_derived_trait_names(krate: &Crate, impl_ids: &[Id]) -> Vec<String>;

/// Collect enum variants with their fields resolved from the rustdoc index.
pub(in ::convert) fn collect_enum_variants(krate: &Crate, e: &Enum) -> Vec<VariantDoc>;

pub(in ::convert) fn collect_impl_doc(krate: &Crate, item: &Item, impl_: &Impl, mode: ConvertMode, policy: CollectPolicy) -> Option<ImplDoc>;

/// Collect struct fields with their types resolved from the rustdoc index.
pub(in ::convert) fn collect_struct_fields(krate: &Crate, s: &Struct, show_members: bool, include_private_members: bool) -> Vec<FieldDoc>;

/// Collect methods defined in a trait.
pub(in ::convert) fn collect_trait_methods(krate: &Crate, trait_: &Trait, show_members: bool) -> Vec<MethodDoc>;

/// Dispatch an item into the appropriate collection on the container.
pub(in ::convert) fn dispatch_item<C: ItemContainer>(krate: &Crate, item: &Item, parent_path: &str, container: &mut C, mode: ConvertMode, policy: CollectPolicy);

/// Check whether an `ImplDoc` is a derived or marker trait implementation.
pub(in ::convert) fn is_derived_or_marker_impl(imp: &ImplDoc) -> bool;

pub(in ::convert) fn is_public_visibility(visibility: &Visibility) -> bool;

pub(in ::convert) fn item_is_public(krate: &Crate, id: &Id) -> bool;

pub(in ::convert) fn render_constant_sig(name: &str, type_: &Type, const_: &Constant, visibility: &Visibility) -> String;

pub(in ::convert) fn render_enum_sig(name: &str, e: &Enum, variants: &[VariantDoc], visibility: &Visibility) -> String;

pub(in ::convert) fn render_fn_header(header: &FunctionHeader, visibility: &Visibility) -> String;

pub(in ::convert) fn render_fn_params(sig: &FunctionSignature) -> String;

pub(in ::convert) fn render_fn_return(sig: &FunctionSignature) -> String;

pub(in ::convert) fn render_function_sig(name: &str, func: &Function, visibility: &Visibility) -> String;

pub(in ::convert) fn render_generic_args(args: &GenericArgs) -> String;

pub(in ::convert) fn render_generic_bound(bound: &GenericBound) -> String;

pub(in ::convert) fn render_generic_param(param: &GenericParamDef) -> String;

pub(in ::convert) fn render_generics(generics: &Generics) -> String;

pub(in ::convert) fn render_impl_header(impl_: &Impl) -> String;

pub(in ::convert) fn render_static_sig(name: &str, s: &Static, visibility: &Visibility) -> String;

pub(in ::convert) fn render_struct_sig(name: &str, s: &Struct, fields: &[FieldDoc], visibility: &Visibility) -> String;

pub(in ::convert) fn render_trait_method_sig(signature: &str) -> String;

pub(in ::convert) fn render_trait_sig(name: &str, t: &Trait, methods: &[MethodDoc], visibility: &Visibility) -> String;

pub(in ::convert) fn render_type_alias_sig(name: &str, ta: &TypeAlias, visibility: &Visibility) -> String;

pub(in ::convert) fn render_visibility_prefix(visibility: &Visibility) -> String;

pub(in ::convert) fn render_where_clause(generics: &Generics) -> String;

/// Shorten a fully qualified type path to its last meaningful segment.
/// e.g., "std::string::String" -> "String", "crate::model::StructDoc" -> "StructDoc"
/// but preserve paths like "Vec" as-is.
pub(in ::convert) fn short_type_path(path: &str) -> String;

pub(in ::convert) fn should_include_impl(krate: &Crate, impl_: &Impl, mode: ConvertMode, policy: CollectPolicy) -> bool;

pub(in ::convert) fn should_include_item(item: &Item, include_private_items: bool) -> bool;

pub(in ::convert) fn should_include_member_item(item: &Item, include_private_members: bool) -> bool;

pub(in ::convert) fn should_include_visibility(visibility: &Visibility, include_private_items: bool) -> bool;

/// Sort all items: public first, then private, alphabetically within each group.
pub(in ::convert) fn sort_items(crate_doc: &mut CrateDoc);

pub(in ::convert) fn sort_module_items(m: &mut ModuleDoc);

pub(crate) fn strip_visibility_prefix(signature: &str) -> &str;

```

