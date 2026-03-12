/// Convert rustdoc JSON types into our internal documentation model.
use anyhow::{Context, Result};
use rustdoc_types::{
    Crate, GenericArg, GenericArgs, GenericBound, GenericParamDef, GenericParamDefKind, Id, Item,
    ItemEnum, Term, Type, Visibility, WherePredicate,
};

use crate::model::{
    ConstantDoc, CrateDoc, EnumDoc, FieldDoc, FunctionDoc, ImplDoc, MethodDoc, ModuleDoc,
    StaticDoc, StructDoc, TraitDoc, TypeAliasDoc, VariantDoc, VariantKind,
};

/// Controls which symbols are included in the converted document tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvertMode {
    Surface,
    Internal,
}

impl ConvertMode {
    fn include_private_items(self) -> bool {
        matches!(self, Self::Internal)
    }

    fn include_private_members(self) -> bool {
        matches!(self, Self::Internal)
    }
}

/// Convert a rustdoc JSON `Crate` into our internal `CrateDoc` model.
pub fn convert(krate: &Crate, mode: ConvertMode) -> Result<CrateDoc> {
    let root_item = krate
        .index
        .get(&krate.root)
        .context("Root item not found in rustdoc JSON index")?;

    let crate_name = root_item
        .name
        .as_deref()
        .context("Root item has no name")?
        .to_string();

    let root_module = match &root_item.inner {
        ItemEnum::Module(m) => m,
        _ => anyhow::bail!("Root item is not a module"),
    };

    let mut crate_doc = CrateDoc {
        name: crate_name.clone(),
        docs: root_item.docs.clone(),
        modules: Vec::new(),
        impls: Vec::new(),
        structs: Vec::new(),
        enums: Vec::new(),
        traits: Vec::new(),
        functions: Vec::new(),
        type_aliases: Vec::new(),
        constants: Vec::new(),
        statics: Vec::new(),
    };

    for item_id in &root_module.items {
        if let Some(item) = krate.index.get(item_id) {
            dispatch_item(krate, item, &crate_name, &mut crate_doc, mode);
        }
    }

    sort_items(&mut crate_doc);

    Ok(crate_doc)
}

/// Sort all items alphabetically by name for stable output.
fn sort_items(crate_doc: &mut CrateDoc) {
    crate_doc
        .modules
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    crate_doc.impls.sort_by(|a, b| {
        a.target_name
            .cmp(&b.target_name)
            .then_with(|| a.header.cmp(&b.header))
    });
    crate_doc.impls.dedup_by(|a, b| a.header == b.header);
    crate_doc
        .structs
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    crate_doc
        .enums
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    crate_doc
        .traits
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    crate_doc
        .functions
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    crate_doc
        .type_aliases
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    crate_doc
        .constants
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    crate_doc
        .statics
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
}

/// Container trait for dispatching items into the right collection.
trait ItemContainer {
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

impl ItemContainer for CrateDoc {
    fn modules_mut(&mut self) -> &mut Vec<ModuleDoc> {
        &mut self.modules
    }
    fn impls_mut(&mut self) -> &mut Vec<ImplDoc> {
        &mut self.impls
    }
    fn structs_mut(&mut self) -> &mut Vec<StructDoc> {
        &mut self.structs
    }
    fn enums_mut(&mut self) -> &mut Vec<EnumDoc> {
        &mut self.enums
    }
    fn traits_mut(&mut self) -> &mut Vec<TraitDoc> {
        &mut self.traits
    }
    fn functions_mut(&mut self) -> &mut Vec<FunctionDoc> {
        &mut self.functions
    }
    fn type_aliases_mut(&mut self) -> &mut Vec<TypeAliasDoc> {
        &mut self.type_aliases
    }
    fn constants_mut(&mut self) -> &mut Vec<ConstantDoc> {
        &mut self.constants
    }
    fn statics_mut(&mut self) -> &mut Vec<StaticDoc> {
        &mut self.statics
    }
}

impl ItemContainer for ModuleDoc {
    fn modules_mut(&mut self) -> &mut Vec<ModuleDoc> {
        &mut self.modules
    }
    fn impls_mut(&mut self) -> &mut Vec<ImplDoc> {
        &mut self.impls
    }
    fn structs_mut(&mut self) -> &mut Vec<StructDoc> {
        &mut self.structs
    }
    fn enums_mut(&mut self) -> &mut Vec<EnumDoc> {
        &mut self.enums
    }
    fn traits_mut(&mut self) -> &mut Vec<TraitDoc> {
        &mut self.traits
    }
    fn functions_mut(&mut self) -> &mut Vec<FunctionDoc> {
        &mut self.functions
    }
    fn type_aliases_mut(&mut self) -> &mut Vec<TypeAliasDoc> {
        &mut self.type_aliases
    }
    fn constants_mut(&mut self) -> &mut Vec<ConstantDoc> {
        &mut self.constants
    }
    fn statics_mut(&mut self) -> &mut Vec<StaticDoc> {
        &mut self.statics
    }
}

/// Dispatch an item into the appropriate collection on the container.
fn dispatch_item<C: ItemContainer>(
    krate: &Crate,
    item: &Item,
    parent_path: &str,
    container: &mut C,
    mode: ConvertMode,
) {
    let name = match &item.name {
        Some(n) => n.as_str(),
        None => return,
    };

    if !should_include_item(item, mode.include_private_items()) {
        return;
    }

    let qualified = format!("{parent_path}::{name}");

    match &item.inner {
        ItemEnum::Module(m) => {
            let mut module_doc = ModuleDoc {
                qualified_name: qualified.clone(),
                docs: item.docs.clone(),
                modules: Vec::new(),
                impls: Vec::new(),
                structs: Vec::new(),
                enums: Vec::new(),
                traits: Vec::new(),
                functions: Vec::new(),
                type_aliases: Vec::new(),
                constants: Vec::new(),
                statics: Vec::new(),
            };

            for child_id in &m.items {
                if let Some(child) = krate.index.get(child_id) {
                    dispatch_item(
                        krate,
                        child,
                        &qualified,
                        &mut module_doc,
                        mode,
                    );
                }
            }

            sort_module_items(&mut module_doc);
            container.modules_mut().push(module_doc);
        }
        ItemEnum::Struct(s) => {
            let show_members = mode.include_private_members() || is_public_visibility(&item.visibility);
            let fields =
                collect_struct_fields(krate, s, show_members, mode.include_private_members());
            let sig = render_struct_sig(name, s, &fields, &item.visibility);
            container
                .impls_mut()
                .extend(collect_assoc_impl_docs(krate, &s.impls, mode));
            container.structs_mut().push(StructDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
                signature: sig,
                fields,
            });
        }
        ItemEnum::Enum(e) => {
            let variants = collect_enum_variants(krate, e);
            let sig = render_enum_sig(name, e, &variants, &item.visibility);
            container
                .impls_mut()
                .extend(collect_assoc_impl_docs(krate, &e.impls, mode));
            container.enums_mut().push(EnumDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
                signature: sig,
                variants,
            });
        }
        ItemEnum::Trait(t) => {
            let show_members = mode.include_private_members() || is_public_visibility(&item.visibility);
            let methods = collect_trait_methods(
                krate,
                t,
                show_members,
            );
            let sig = render_trait_sig(name, t, &methods, &item.visibility);
            container
                .impls_mut()
                .extend(collect_assoc_impl_docs(krate, &t.implementations, mode));
            container.traits_mut().push(TraitDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
                signature: sig,
                methods,
            });
        }
        ItemEnum::Function(f) => {
            let sig = render_function_sig(name, f, &item.visibility);
            container.functions_mut().push(FunctionDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
                signature: sig,
            });
        }
        ItemEnum::TypeAlias(ta) => {
            let sig = render_type_alias_sig(name, ta, &item.visibility);
            container.type_aliases_mut().push(TypeAliasDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
                signature: sig,
            });
        }
        ItemEnum::Constant { type_, const_ } => {
            let sig = render_constant_sig(name, type_, const_, &item.visibility);
            container.constants_mut().push(ConstantDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
                signature: sig,
            });
        }
        ItemEnum::Static(s) => {
            let sig = render_static_sig(name, s, &item.visibility);
            container.statics_mut().push(StaticDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
                signature: sig,
            });
        }
        ItemEnum::Impl(impl_) => {
            if let Some(doc) = collect_impl_doc(krate, item, impl_, mode) {
                container.impls_mut().push(doc);
            }
        }
        _ => {
            // Skip items we don't handle (Use, Union, etc.)
        }
    }
}

fn sort_module_items(m: &mut ModuleDoc) {
    m.modules
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    m.impls.sort_by(|a, b| {
        a.target_name
            .cmp(&b.target_name)
            .then_with(|| a.header.cmp(&b.header))
    });
    m.impls.dedup_by(|a, b| a.header == b.header);
    m.structs
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    m.enums
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    m.traits
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    m.functions
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    m.type_aliases
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    m.constants
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
    m.statics
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
}

fn should_include_item(item: &Item, include_private_items: bool) -> bool {
    should_include_visibility(&item.visibility, include_private_items)
}

fn should_include_visibility(visibility: &Visibility, include_private_items: bool) -> bool {
    include_private_items || is_public_visibility(visibility)
}

fn is_public_visibility(visibility: &Visibility) -> bool {
    matches!(visibility, Visibility::Public)
}

fn should_include_member_item(item: &Item, include_private_members: bool) -> bool {
    include_private_members || is_public_visibility(&item.visibility)
}

// ---------------------------------------------------------------------------
// Struct field collection
// ---------------------------------------------------------------------------

/// Collect struct fields with their types resolved from the rustdoc index.
fn collect_struct_fields(
    krate: &Crate,
    s: &rustdoc_types::Struct,
    show_members: bool,
    include_private_members: bool,
) -> Vec<FieldDoc> {
    if !show_members {
        return Vec::new();
    }

    match &s.kind {
        rustdoc_types::StructKind::Plain {
            fields,
            has_stripped_fields: _,
        } => {
            let mut field_docs = Vec::new();
            for field_id in fields {
                if let Some(field_item) = krate.index.get(field_id) {
                    if !should_include_member_item(field_item, include_private_members) {
                        continue;
                    }
                    if let ItemEnum::StructField(ty) = &field_item.inner {
                        let field_name = field_item.name.as_deref().unwrap_or("_").to_string();
                        field_docs.push(FieldDoc {
                            name: field_name,
                            type_str: render_type(ty),
                            docs: field_item.docs.clone(),
                            is_public: is_public_visibility(&field_item.visibility),
                        });
                    }
                }
            }
            field_docs
        }
        rustdoc_types::StructKind::Tuple(fields) => {
            let mut field_docs = Vec::new();
            for (i, field_opt) in fields.iter().enumerate() {
                if let Some(field_id) = field_opt {
                    if let Some(field_item) = krate.index.get(field_id) {
                        if !should_include_member_item(field_item, include_private_members) {
                            continue;
                        }
                        if let ItemEnum::StructField(ty) = &field_item.inner {
                            field_docs.push(FieldDoc {
                                name: format!("{i}"),
                                type_str: render_type(ty),
                                docs: field_item.docs.clone(),
                                is_public: is_public_visibility(&field_item.visibility),
                            });
                        }
                    }
                }
            }
            field_docs
        }
        rustdoc_types::StructKind::Unit => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Enum variant collection
// ---------------------------------------------------------------------------

/// Collect enum variants with their fields resolved from the rustdoc index.
fn collect_enum_variants(krate: &Crate, e: &rustdoc_types::Enum) -> Vec<VariantDoc> {
    let mut variants = Vec::new();

    for variant_id in &e.variants {
        if let Some(variant_item) = krate.index.get(variant_id) {
            let variant_name = variant_item.name.as_deref().unwrap_or("_").to_string();
            if let ItemEnum::Variant(v) = &variant_item.inner {
                let kind = match &v.kind {
                    rustdoc_types::VariantKind::Plain => VariantKind::Plain,
                    rustdoc_types::VariantKind::Tuple(fields) => {
                        let mut type_strs = Vec::new();
                        for field_opt in fields {
                            if let Some(field_id) = field_opt {
                                if let Some(field_item) = krate.index.get(field_id) {
                                    if let ItemEnum::StructField(ty) = &field_item.inner {
                                        type_strs.push(render_type(ty));
                                    } else {
                                        type_strs.push("_".to_string());
                                    }
                                } else {
                                    type_strs.push("_".to_string());
                                }
                            } else {
                                type_strs.push("_".to_string());
                            }
                        }
                        VariantKind::Tuple(type_strs)
                    }
                    rustdoc_types::VariantKind::Struct {
                        fields,
                        has_stripped_fields: _,
                    } => {
                        let mut field_docs = Vec::new();
                        for field_id in fields {
                            if let Some(field_item) = krate.index.get(field_id) {
                                if let ItemEnum::StructField(ty) = &field_item.inner {
                                    let field_name =
                                        field_item.name.as_deref().unwrap_or("_").to_string();
                                    field_docs.push(FieldDoc {
                                        name: field_name,
                                        type_str: render_type(ty),
                                        docs: field_item.docs.clone(),
                                        is_public: true,
                                    });
                                }
                            }
                        }
                        VariantKind::Struct(field_docs)
                    }
                };
                variants.push(VariantDoc {
                    name: variant_name,
                    docs: variant_item.docs.clone(),
                    kind,
                });
            }
        }
    }

    variants
}

fn collect_impl_doc(
    krate: &Crate,
    item: &Item,
    impl_: &rustdoc_types::Impl,
    mode: ConvertMode,
) -> Option<ImplDoc> {
    if item.span.is_none() || impl_.is_synthetic || !should_include_impl(krate, impl_, mode) {
        return None;
    }

    let mut methods = Vec::new();
    for method_id in &impl_.items {
        let Some(method_item) = krate.index.get(method_id) else {
            continue;
        };
        let ItemEnum::Function(function) = &method_item.inner else {
            continue;
        };

        let include_method = if mode.include_private_members() || impl_.trait_.is_some() {
            true
        } else {
            should_include_member_item(method_item, false)
        };
        if !include_method {
            continue;
        }

        let method_name = method_item.name.as_deref().unwrap_or("_");
        methods.push(MethodDoc {
            name: method_name.to_string(),
            docs: method_item.docs.clone(),
            signature: render_function_sig(method_name, function, &method_item.visibility),
        });
    }
    methods.sort_by(|a, b| a.name.cmp(&b.name));

    if !mode.include_private_items() && impl_.trait_.is_none() && methods.is_empty() {
        return None;
    }

    Some(ImplDoc {
        header: render_impl_header(impl_),
        docs: item.docs.clone(),
        methods,
        target_name: render_type(&impl_.for_),
    })
}

fn should_include_impl(krate: &Crate, impl_: &rustdoc_types::Impl, mode: ConvertMode) -> bool {
    if mode.include_private_items() {
        return true;
    }

    let target_visible = match &impl_.for_ {
        Type::ResolvedPath(path) => item_is_public(krate, &path.id),
        _ => true,
    };
    if !target_visible {
        return false;
    }

    match &impl_.trait_ {
        Some(path) => krate
            .index
            .get(&path.id)
            .map(|item| is_public_visibility(&item.visibility))
            .unwrap_or(true),
        None => true,
    }
}

fn item_is_public(krate: &Crate, id: &Id) -> bool {
    krate.index
        .get(id)
        .map(|item| is_public_visibility(&item.visibility))
        .unwrap_or(true)
}

fn collect_assoc_impl_docs(krate: &Crate, impl_ids: &[Id], mode: ConvertMode) -> Vec<ImplDoc> {
    impl_ids
        .iter()
        .filter_map(|impl_id| krate.index.get(impl_id))
        .filter_map(|item| match &item.inner {
            ItemEnum::Impl(impl_) => collect_impl_doc(krate, item, impl_, mode),
            _ => None,
        })
        .collect()
}

/// Collect methods defined in a trait.
fn collect_trait_methods(
    krate: &Crate,
    trait_: &rustdoc_types::Trait,
    show_members: bool,
) -> Vec<MethodDoc> {
    if !show_members {
        return Vec::new();
    }

    let mut methods = Vec::new();

    for method_id in &trait_.items {
        if let Some(method_item) = krate.index.get(method_id) {
            if let ItemEnum::Function(f) = &method_item.inner {
                let method_name = method_item.name.as_deref().unwrap_or("_");
                methods.push(MethodDoc {
                    name: method_name.to_string(),
                    docs: method_item.docs.clone(),
                    signature: render_function_sig(method_name, f, &method_item.visibility),
                });
            }
        }
    }

    methods
}

// ---------------------------------------------------------------------------
// Signature rendering
// ---------------------------------------------------------------------------

fn render_function_sig(
    name: &str,
    func: &rustdoc_types::Function,
    visibility: &Visibility,
) -> String {
    let generics_str = render_generics(&func.generics);
    let params = render_fn_params(&func.sig);
    let ret = render_fn_return(&func.sig);
    let where_clause = render_where_clause(&func.generics);
    let header = render_fn_header(&func.header, visibility);

    let mut sig = format!("{header}fn {name}{generics_str}({params}){ret}");
    if !where_clause.is_empty() {
        sig.push_str(&format!("\nwhere\n{where_clause}"));
    }
    sig
}

fn render_visibility_prefix(visibility: &Visibility) -> String {
    match visibility {
        Visibility::Public => "pub ".to_string(),
        Visibility::Default => String::new(),
        Visibility::Crate => "pub(crate) ".to_string(),
        Visibility::Restricted { path, .. } => format!("pub(in {path}) "),
    }
}

fn render_fn_header(header: &rustdoc_types::FunctionHeader, visibility: &Visibility) -> String {
    let mut sig = render_visibility_prefix(visibility);
    if header.is_unsafe {
        sig.push_str("unsafe ");
    }
    if header.is_const {
        sig.push_str("const ");
    }
    if header.is_async {
        sig.push_str("async ");
    }
    sig
}

fn render_fn_params(sig: &rustdoc_types::FunctionSignature) -> String {
    sig.inputs
        .iter()
        .map(|(name, ty)| {
            if name == "self" {
                match ty {
                    Type::BorrowedRef {
                        lifetime,
                        is_mutable,
                        ..
                    } => {
                        let lt = lifetime
                            .as_deref()
                            .map(|l| format!("{l} "))
                            .unwrap_or_default();
                        if *is_mutable {
                            format!("&{lt}mut self")
                        } else {
                            format!("&{lt}self")
                        }
                    }
                    _ => "self".to_string(),
                }
            } else {
                format!("{name}: {}", render_type(ty))
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_fn_return(sig: &rustdoc_types::FunctionSignature) -> String {
    match &sig.output {
        Some(ty) => format!(" -> {}", render_type(ty)),
        None => String::new(),
    }
}

fn render_struct_sig(
    name: &str,
    s: &rustdoc_types::Struct,
    fields: &[FieldDoc],
    visibility: &Visibility,
) -> String {
    let generics_str = render_generics(&s.generics);
    let where_clause = render_where_clause(&s.generics);
    let visibility = render_visibility_prefix(visibility);

    match &s.kind {
        rustdoc_types::StructKind::Unit => {
            let mut sig = format!("{visibility}struct {name}{generics_str}");
            if !where_clause.is_empty() {
                sig.push_str(&format!("\nwhere\n{where_clause}"));
            }
            sig.push(';');
            sig
        }
        rustdoc_types::StructKind::Tuple(tuple_fields) => {
            let mut field_strs: Vec<String> = fields
                .iter()
                .map(|field| {
                    let vis = if field.is_public { "pub " } else { "" };
                    format!("{vis}{}", field.type_str)
                })
                .collect();

            if tuple_fields.len() > fields.len() {
                field_strs.push("/* private fields omitted */".to_string());
            }

            let mut sig = format!(
                "{visibility}struct {name}{generics_str}({})",
                field_strs.join(", ")
            );
            if !where_clause.is_empty() {
                sig.push_str(&format!("\nwhere\n{where_clause}"));
            }
            sig.push(';');
            sig
        }
        rustdoc_types::StructKind::Plain {
            fields: plain_fields,
            has_stripped_fields,
        } => {
            let mut sig = format!("{visibility}struct {name}{generics_str}");
            if !where_clause.is_empty() {
                sig.push_str(&format!("\nwhere\n{where_clause}"));
            }
            sig.push_str(" {\n");

            for field in fields {
                let vis = if field.is_public { "pub " } else { "" };
                sig.push_str(&format!("    {vis}{}: {},\n", field.name, field.type_str));
            }

            let has_hidden_fields = plain_fields.len() > fields.len();

            if *has_stripped_fields || has_hidden_fields {
                sig.push_str("    // some fields omitted\n");
            }

            sig.push('}');
            sig
        }
    }
}

fn render_enum_sig(
    name: &str,
    e: &rustdoc_types::Enum,
    variants: &[VariantDoc],
    visibility: &Visibility,
) -> String {
    let generics_str = render_generics(&e.generics);
    let where_clause = render_where_clause(&e.generics);

    let mut sig = format!("{}enum {name}{generics_str}", render_visibility_prefix(visibility));
    if !where_clause.is_empty() {
        sig.push_str(&format!("\nwhere\n{where_clause}"));
    }
    sig.push_str(" {\n");

    for variant in variants {
        match &variant.kind {
            VariantKind::Plain => {
                sig.push_str(&format!("    {},\n", variant.name));
            }
            VariantKind::Tuple(types) => {
                sig.push_str(&format!("    {}({}),\n", variant.name, types.join(", ")));
            }
            VariantKind::Struct(fields) => {
                sig.push_str(&format!("    {} {{\n", variant.name));
                for field in fields {
                    sig.push_str(&format!("        {}: {},\n", field.name, field.type_str));
                }
                sig.push_str("    },\n");
            }
        }
    }

    sig.push('}');
    sig
}

fn render_trait_sig(
    name: &str,
    t: &rustdoc_types::Trait,
    methods: &[MethodDoc],
    visibility: &Visibility,
) -> String {
    let generics_str = render_generics(&t.generics);
    let where_clause = render_where_clause(&t.generics);

    let bounds = if t.bounds.is_empty() {
        String::new()
    } else {
        let bound_strs: Vec<String> = t.bounds.iter().map(render_generic_bound).collect();
        format!(": {}", bound_strs.join(" + "))
    };

    let mut sig = format!(
        "{}trait {name}{generics_str}{bounds}",
        render_visibility_prefix(visibility)
    );
    if !where_clause.is_empty() {
        sig.push_str(&format!("\nwhere\n{where_clause}"));
    }
    sig.push_str(" {\n");
    if methods.is_empty() {
        sig.push_str("    // ...\n");
    } else {
        for method in methods {
            for line in render_trait_method_sig(&method.signature).lines() {
                sig.push_str("    ");
                sig.push_str(line);
                sig.push('\n');
            }
        }
    }
    sig.push('}');
    sig
}

fn render_trait_method_sig(signature: &str) -> String {
    let mut sig = strip_visibility_prefix(signature.trim()).to_string();
    if !sig.ends_with(';') {
        sig.push(';');
    }
    sig
}

fn strip_visibility_prefix(signature: &str) -> &str {
    let signature = signature.strip_prefix("pub ").unwrap_or(signature);
    let signature = signature.strip_prefix("pub(crate) ").unwrap_or(signature);
    if let Some(rest) = signature.strip_prefix("pub(in ") {
        if let Some(idx) = rest.find(") ") {
            &rest[idx + 2..]
        } else {
            signature
        }
    } else {
        signature
    }
}

fn render_impl_header(impl_: &rustdoc_types::Impl) -> String {
    let generics = render_generics(&impl_.generics);
    let where_clause = render_where_clause(&impl_.generics);
    let unsafe_prefix = if impl_.is_unsafe { "unsafe " } else { "" };
    let negative = if impl_.is_negative { "!" } else { "" };
    let target = render_type(&impl_.for_);

    let mut header = match &impl_.trait_ {
        Some(trait_path) => format!(
            "{unsafe_prefix}impl{generics} {negative}{} for {target}",
            render_type(&Type::ResolvedPath(trait_path.clone()))
        ),
        None => format!("{unsafe_prefix}impl{generics} {target}"),
    };

    if !where_clause.is_empty() {
        header.push_str(&format!("\nwhere\n{where_clause}"));
    }

    header
}

fn render_type_alias_sig(name: &str, ta: &rustdoc_types::TypeAlias, visibility: &Visibility) -> String {
    let generics_str = render_generics(&ta.generics);
    let where_clause = render_where_clause(&ta.generics);
    let type_str = render_type(&ta.type_);
    let mut sig = format!(
        "{}type {name}{generics_str} = {type_str}",
        render_visibility_prefix(visibility)
    );
    if !where_clause.is_empty() {
        sig.push_str(&format!("\nwhere\n{where_clause}"));
    }
    sig.push(';');
    sig
}

fn render_constant_sig(
    name: &str,
    type_: &Type,
    const_: &rustdoc_types::Constant,
    visibility: &Visibility,
) -> String {
    let type_str = render_type(type_);
    let value = const_
        .value
        .as_deref()
        .map(|v| format!(" = {v}"))
        .unwrap_or_default();
    format!(
        "{}const {name}: {type_str}{value};",
        render_visibility_prefix(visibility)
    )
}

fn render_static_sig(name: &str, s: &rustdoc_types::Static, visibility: &Visibility) -> String {
    let type_str = render_type(&s.type_);
    let mutability = if s.is_mutable { "mut " } else { "" };
    format!(
        "{}static {mutability}{name}: {type_str};",
        render_visibility_prefix(visibility)
    )
}

// ---------------------------------------------------------------------------
// Type rendering
// ---------------------------------------------------------------------------

pub fn render_type(ty: &Type) -> String {
    match ty {
        Type::ResolvedPath(path) => {
            // Use only the last segment of the path for cleaner output
            let short = short_type_path(&path.path);
            let mut result = short;
            if let Some(args) = &path.args {
                result.push_str(&render_generic_args(args));
            }
            result
        }
        Type::DynTrait(dyn_trait) => {
            let traits: Vec<String> = dyn_trait
                .traits
                .iter()
                .map(|pt| {
                    let mut s = String::new();
                    if !pt.generic_params.is_empty() {
                        let params: Vec<String> =
                            pt.generic_params.iter().map(render_generic_param).collect();
                        s.push_str(&format!("for<{}> ", params.join(", ")));
                    }
                    s.push_str(&render_type(&Type::ResolvedPath(pt.trait_.clone())));
                    s
                })
                .collect();
            let lifetime = dyn_trait
                .lifetime
                .as_deref()
                .map(|l| format!(" + {l}"))
                .unwrap_or_default();
            format!("dyn {}{lifetime}", traits.join(" + "))
        }
        Type::Generic(s) => s.clone(),
        Type::Primitive(s) => s.clone(),
        Type::FunctionPointer(fp) => {
            let params: Vec<String> = fp
                .sig
                .inputs
                .iter()
                .map(|(name, ty)| {
                    let ty_str = render_type(ty);
                    if name.is_empty() {
                        ty_str
                    } else {
                        format!("{name}: {ty_str}")
                    }
                })
                .collect();
            let ret = match &fp.sig.output {
                Some(ty) => format!(" -> {}", render_type(ty)),
                None => String::new(),
            };
            format!("fn({}){ret}", params.join(", "))
        }
        Type::Tuple(types) => {
            if types.is_empty() {
                "()".to_string()
            } else {
                let inner: Vec<String> = types.iter().map(render_type).collect();
                format!("({})", inner.join(", "))
            }
        }
        Type::Slice(ty) => format!("[{}]", render_type(ty)),
        Type::Array { type_, len } => format!("[{}; {len}]", render_type(type_)),
        Type::Pat { type_, .. } => render_type(type_),
        Type::ImplTrait(bounds) => {
            let bound_strs: Vec<String> = bounds.iter().map(render_generic_bound).collect();
            format!("impl {}", bound_strs.join(" + "))
        }
        Type::Infer => "_".to_string(),
        Type::RawPointer { is_mutable, type_ } => {
            if *is_mutable {
                format!("*mut {}", render_type(type_))
            } else {
                format!("*const {}", render_type(type_))
            }
        }
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => {
            let lt = lifetime
                .as_deref()
                .map(|l| format!("{l} "))
                .unwrap_or_default();
            let mutability = if *is_mutable { "mut " } else { "" };
            format!("&{lt}{mutability}{}", render_type(type_))
        }
        Type::QualifiedPath {
            name,
            self_type,
            trait_,
            ..
        } => {
            let self_str = render_type(self_type);
            match trait_ {
                Some(trait_path) => {
                    let trait_str = render_type(&Type::ResolvedPath(trait_path.clone()));
                    format!("<{self_str} as {trait_str}>::{name}")
                }
                None => format!("{self_str}::{name}"),
            }
        }
    }
}

/// Shorten a fully qualified type path to its last meaningful segment.
/// e.g., "std::string::String" -> "String", "crate::model::StructDoc" -> "StructDoc"
/// but preserve paths like "Vec" as-is.
fn short_type_path(path: &str) -> String {
    // If the path contains :: separators, take the last segment
    if let Some(last) = path.rsplit("::").next() {
        last.to_string()
    } else {
        path.to_string()
    }
}

fn render_generic_args(args: &GenericArgs) -> String {
    match args {
        GenericArgs::AngleBracketed { args, constraints } => {
            if args.is_empty() && constraints.is_empty() {
                return String::new();
            }
            let mut parts: Vec<String> = args
                .iter()
                .map(|arg| match arg {
                    GenericArg::Lifetime(l) => l.clone(),
                    GenericArg::Type(ty) => render_type(ty),
                    GenericArg::Const(c) => c.value.as_deref().unwrap_or("_").to_string(),
                    GenericArg::Infer => "_".to_string(),
                })
                .collect();
            for constraint in constraints {
                let mut s = constraint.name.clone();
                match &constraint.binding {
                    rustdoc_types::AssocItemConstraintKind::Equality(term) => {
                        s.push_str(" = ");
                        match term {
                            Term::Type(ty) => s.push_str(&render_type(ty)),
                            Term::Constant(c) => s.push_str(c.value.as_deref().unwrap_or("_")),
                        }
                    }
                    rustdoc_types::AssocItemConstraintKind::Constraint(bounds) => {
                        if !bounds.is_empty() {
                            let bound_strs: Vec<String> =
                                bounds.iter().map(render_generic_bound).collect();
                            s.push_str(&format!(": {}", bound_strs.join(" + ")));
                        }
                    }
                }
                parts.push(s);
            }
            format!("<{}>", parts.join(", "))
        }
        GenericArgs::Parenthesized { inputs, output } => {
            let input_strs: Vec<String> = inputs.iter().map(render_type).collect();
            let ret = match output {
                Some(ty) => format!(" -> {}", render_type(ty)),
                None => String::new(),
            };
            format!("({}){ret}", input_strs.join(", "))
        }
        GenericArgs::ReturnTypeNotation => "(..)".to_string(),
    }
}

fn render_generics(generics: &rustdoc_types::Generics) -> String {
    if generics.params.is_empty() {
        return String::new();
    }
    let params: Vec<String> = generics.params.iter().map(render_generic_param).collect();
    format!("<{}>", params.join(", "))
}

fn render_generic_param(param: &GenericParamDef) -> String {
    match &param.kind {
        GenericParamDefKind::Lifetime { outlives } => {
            let mut s = param.name.clone();
            if !outlives.is_empty() {
                s.push_str(&format!(": {}", outlives.join(" + ")));
            }
            s
        }
        GenericParamDefKind::Type {
            bounds,
            default,
            is_synthetic,
        } => {
            if *is_synthetic {
                return param.name.clone();
            }
            let mut s = param.name.clone();
            if !bounds.is_empty() {
                let bound_strs: Vec<String> = bounds.iter().map(render_generic_bound).collect();
                s.push_str(&format!(": {}", bound_strs.join(" + ")));
            }
            if let Some(default) = default {
                s.push_str(&format!(" = {}", render_type(default)));
            }
            s
        }
        GenericParamDefKind::Const { type_, default } => {
            let mut s = format!("const {}: {}", param.name, render_type(type_));
            if let Some(default) = default {
                s.push_str(&format!(" = {default}"));
            }
            s
        }
    }
}

fn render_generic_bound(bound: &GenericBound) -> String {
    match bound {
        GenericBound::TraitBound {
            trait_,
            generic_params,
            modifier,
        } => {
            let mut s = String::new();
            match modifier {
                rustdoc_types::TraitBoundModifier::None => {}
                rustdoc_types::TraitBoundModifier::Maybe => s.push('?'),
                rustdoc_types::TraitBoundModifier::MaybeConst => s.push_str("~const "),
            }
            if !generic_params.is_empty() {
                let params: Vec<String> = generic_params.iter().map(render_generic_param).collect();
                s.push_str(&format!("for<{}> ", params.join(", ")));
            }
            s.push_str(&render_type(&Type::ResolvedPath(trait_.clone())));
            s
        }
        GenericBound::Outlives(lifetime) => lifetime.clone(),
        GenericBound::Use(_) => "use<..>".to_string(),
    }
}

fn render_where_clause(generics: &rustdoc_types::Generics) -> String {
    if generics.where_predicates.is_empty() {
        return String::new();
    }
    generics
        .where_predicates
        .iter()
        .map(|pred| {
            let s = match pred {
                WherePredicate::BoundPredicate {
                    type_,
                    bounds,
                    generic_params,
                } => {
                    let mut prefix = String::new();
                    if !generic_params.is_empty() {
                        let params: Vec<String> =
                            generic_params.iter().map(render_generic_param).collect();
                        prefix = format!("for<{}> ", params.join(", "));
                    }
                    let type_str = render_type(type_);
                    let bound_strs: Vec<String> = bounds.iter().map(render_generic_bound).collect();
                    format!("{prefix}{type_str}: {}", bound_strs.join(" + "))
                }
                WherePredicate::LifetimePredicate { lifetime, outlives } => {
                    format!("{}: {}", lifetime, outlives.join(" + "))
                }
                WherePredicate::EqPredicate { lhs, rhs } => {
                    let lhs_str = render_type(lhs);
                    let rhs_str = match rhs {
                        Term::Type(ty) => render_type(ty),
                        Term::Constant(c) => c.value.as_deref().unwrap_or("_").to_string(),
                    };
                    format!("{lhs_str} = {rhs_str}")
                }
            };
            format!("    {s},")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_primitive_type() {
        let ty = Type::Primitive("u32".to_string());
        assert_eq!(render_type(&ty), "u32");
    }

    #[test]
    fn test_render_generic_type() {
        let ty = Type::Generic("T".to_string());
        assert_eq!(render_type(&ty), "T");
    }

    #[test]
    fn test_render_tuple_type() {
        let ty = Type::Tuple(vec![
            Type::Primitive("u32".to_string()),
            Type::Primitive("String".to_string()),
        ]);
        assert_eq!(render_type(&ty), "(u32, String)");
    }

    #[test]
    fn test_render_unit_type() {
        let ty = Type::Tuple(vec![]);
        assert_eq!(render_type(&ty), "()");
    }

    #[test]
    fn test_render_slice_type() {
        let ty = Type::Slice(Box::new(Type::Primitive("u8".to_string())));
        assert_eq!(render_type(&ty), "[u8]");
    }

    #[test]
    fn test_render_array_type() {
        let ty = Type::Array {
            type_: Box::new(Type::Primitive("u8".to_string())),
            len: "16".to_string(),
        };
        assert_eq!(render_type(&ty), "[u8; 16]");
    }

    #[test]
    fn test_render_raw_pointer() {
        let ty = Type::RawPointer {
            is_mutable: false,
            type_: Box::new(Type::Primitive("u8".to_string())),
        };
        assert_eq!(render_type(&ty), "*const u8");

        let ty_mut = Type::RawPointer {
            is_mutable: true,
            type_: Box::new(Type::Primitive("u8".to_string())),
        };
        assert_eq!(render_type(&ty_mut), "*mut u8");
    }

    #[test]
    fn test_render_borrowed_ref() {
        let ty = Type::BorrowedRef {
            lifetime: None,
            is_mutable: false,
            type_: Box::new(Type::Primitive("str".to_string())),
        };
        assert_eq!(render_type(&ty), "&str");

        let ty_mut = Type::BorrowedRef {
            lifetime: Some("'a".to_string()),
            is_mutable: true,
            type_: Box::new(Type::Primitive("str".to_string())),
        };
        assert_eq!(render_type(&ty_mut), "&'a mut str");
    }

    #[test]
    fn test_render_infer() {
        assert_eq!(render_type(&Type::Infer), "_");
    }

    #[test]
    fn test_short_type_path() {
        assert_eq!(short_type_path("std::string::String"), "String");
        assert_eq!(short_type_path("crate::model::StructDoc"), "StructDoc");
        assert_eq!(short_type_path("Vec"), "Vec");
        assert_eq!(short_type_path("u32"), "u32");
    }

    #[test]
    fn test_should_include_visibility_without_private_items() {
        assert!(should_include_visibility(&Visibility::Public, false));
        assert!(!should_include_visibility(&Visibility::Default, false));
        assert!(!should_include_visibility(&Visibility::Crate, false));
    }

    #[test]
    fn test_should_include_visibility_with_private_items() {
        assert!(should_include_visibility(&Visibility::Default, true));
        assert!(should_include_visibility(&Visibility::Crate, true));
    }

    #[test]
    fn test_render_visibility_prefix() {
        assert_eq!(render_visibility_prefix(&Visibility::Public), "pub ");
        assert_eq!(render_visibility_prefix(&Visibility::Default), "");
        assert_eq!(render_visibility_prefix(&Visibility::Crate), "pub(crate) ");
        assert_eq!(
            render_visibility_prefix(&Visibility::Restricted {
                parent: Id(0),
                path: "crate::internal".to_string(),
            }),
            "pub(in crate::internal) "
        );
    }
}
