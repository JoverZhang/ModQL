# Internal Module `modql::convert`

## Enums

### `ConvertMode`

Controls which symbols are included in the converted document tree.

```rust
pub enum ConvertMode {
    Surface,
    Internal,
}
```

## Traits

### `ItemContainer`

Container trait for dispatching items into the right collection.

```rust
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

## Impl Blocks

### `impl Clone for ConvertMode`

```rust
impl Clone for ConvertMode {
    fn clone(&self) -> ConvertMode;

}
```

### `impl ConvertMode`

```rust
impl ConvertMode {
    pub(in ::convert) fn include_private_items(self) -> bool;

    pub(in ::convert) fn include_private_members(self) -> bool;

}
```

### `impl Copy for ConvertMode`

```rust
impl Copy for ConvertMode;
```

### `impl Debug for ConvertMode`

```rust
impl Debug for ConvertMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Eq for ConvertMode`

```rust
impl Eq for ConvertMode;
```

### `impl PartialEq for ConvertMode`

```rust
impl PartialEq for ConvertMode {
    fn eq(&self, other: &ConvertMode) -> bool;

}
```

### `impl StructuralPartialEq for ConvertMode`

```rust
impl StructuralPartialEq for ConvertMode;
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

## Functions

```rust
pub(in ::convert) fn collect_assoc_impl_docs(krate: &Crate, impl_ids: &[Id], mode: ConvertMode) -> Vec<ImplDoc>;

/// Collect enum variants with their fields resolved from the rustdoc index.
pub(in ::convert) fn collect_enum_variants(krate: &Crate, e: &Enum) -> Vec<VariantDoc>;

pub(in ::convert) fn collect_impl_doc(krate: &Crate, item: &Item, impl_: &Impl, mode: ConvertMode) -> Option<ImplDoc>;

/// Collect struct fields with their types resolved from the rustdoc index.
pub(in ::convert) fn collect_struct_fields(krate: &Crate, s: &Struct, show_members: bool, include_private_members: bool) -> Vec<FieldDoc>;

/// Collect methods defined in a trait.
pub(in ::convert) fn collect_trait_methods(krate: &Crate, trait_: &Trait, show_members: bool) -> Vec<MethodDoc>;

/// Convert a rustdoc JSON `Crate` into our internal `CrateDoc` model.
pub fn convert(krate: &Crate, mode: ConvertMode) -> Result<CrateDoc>;

/// Dispatch an item into the appropriate collection on the container.
pub(in ::convert) fn dispatch_item<C: ItemContainer>(krate: &Crate, item: &Item, parent_path: &str, container: &mut C, mode: ConvertMode);

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

pub fn render_type(ty: &Type) -> String;

pub(in ::convert) fn render_type_alias_sig(name: &str, ta: &TypeAlias, visibility: &Visibility) -> String;

pub(in ::convert) fn render_visibility_prefix(visibility: &Visibility) -> String;

pub(in ::convert) fn render_where_clause(generics: &Generics) -> String;

/// Shorten a fully qualified type path to its last meaningful segment.
/// e.g., "std::string::String" -> "String", "crate::model::StructDoc" -> "StructDoc"
/// but preserve paths like "Vec" as-is.
pub(in ::convert) fn short_type_path(path: &str) -> String;

pub(in ::convert) fn should_include_impl(krate: &Crate, impl_: &Impl, mode: ConvertMode) -> bool;

pub(in ::convert) fn should_include_item(item: &Item, include_private_items: bool) -> bool;

pub(in ::convert) fn should_include_member_item(item: &Item, include_private_members: bool) -> bool;

pub(in ::convert) fn should_include_visibility(visibility: &Visibility, include_private_items: bool) -> bool;

/// Sort all items alphabetically by name for stable output.
pub(in ::convert) fn sort_items(crate_doc: &mut CrateDoc);

pub(in ::convert) fn sort_module_items(m: &mut ModuleDoc);

pub(in ::convert) fn strip_visibility_prefix(signature: &str) -> &str;

```

