/// Convert rustdoc JSON types into our internal documentation model.
use anyhow::{Context, Result};
use rustdoc_types::{
    Crate, GenericArg, GenericArgs, GenericBound, GenericParamDef, GenericParamDefKind, Id, Item,
    ItemEnum, Term, Type, WherePredicate,
};

use crate::model::{
    ConstantDoc, CrateDoc, EnumDoc, FunctionDoc, MethodDoc, ModuleDoc, StaticDoc, StructDoc,
    TraitDoc, TypeAliasDoc,
};

/// Convert a rustdoc JSON `Crate` into our internal `CrateDoc` model.
pub fn convert(krate: &Crate) -> Result<CrateDoc> {
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
            dispatch_item(krate, item, &crate_name, &mut crate_doc);
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
) {
    let name = match &item.name {
        Some(n) => n.as_str(),
        None => return,
    };

    let qualified = format!("{parent_path}::{name}");

    match &item.inner {
        ItemEnum::Module(m) => {
            let mut module_doc = ModuleDoc {
                qualified_name: qualified.clone(),
                docs: item.docs.clone(),
                modules: Vec::new(),
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
                    dispatch_item(krate, child, &qualified, &mut module_doc);
                }
            }

            sort_module_items(&mut module_doc);
            container.modules_mut().push(module_doc);
        }
        ItemEnum::Struct(s) => {
            let sig = render_struct_sig(name, s, item);
            let methods = collect_inherent_methods(krate, &item.id);
            container.structs_mut().push(StructDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
                signature: sig,
                methods,
            });
        }
        ItemEnum::Enum(e) => {
            let sig = render_enum_sig(krate, name, e, item);
            let methods = collect_inherent_methods(krate, &item.id);
            container.enums_mut().push(EnumDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
                signature: sig,
                methods,
            });
        }
        ItemEnum::Trait(t) => {
            let sig = render_trait_sig(name, t, item);
            let methods = collect_trait_methods(krate, t);
            container.traits_mut().push(TraitDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
                signature: sig,
                methods,
            });
        }
        ItemEnum::Function(f) => {
            let sig = render_function_sig(name, f);
            container.functions_mut().push(FunctionDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
                signature: sig,
            });
        }
        ItemEnum::TypeAlias(_ta) => {
            container.type_aliases_mut().push(TypeAliasDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
            });
        }
        ItemEnum::Constant {
            type_: _,
            const_: _,
        } => {
            container.constants_mut().push(ConstantDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
            });
        }
        ItemEnum::Static(_s) => {
            container.statics_mut().push(StaticDoc {
                qualified_name: qualified,
                docs: item.docs.clone(),
            });
        }
        _ => {
            // Skip items we don't handle in the MVP (Use, Union, etc.)
        }
    }
}

fn sort_module_items(m: &mut ModuleDoc) {
    m.modules
        .sort_by(|a, b| a.qualified_name.cmp(&b.qualified_name));
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

// ---------------------------------------------------------------------------
// Inherent and trait method collection
// ---------------------------------------------------------------------------

/// Find all inherent impl methods for a given type (by its Id).
fn collect_inherent_methods(krate: &Crate, type_id: &Id) -> Vec<MethodDoc> {
    let mut methods = Vec::new();

    for item in krate.index.values() {
        if let ItemEnum::Impl(impl_) = &item.inner {
            // Only inherent impls (no trait)
            if impl_.trait_.is_some() {
                continue;
            }
            // Check if this impl is for our type
            if !impl_is_for_type(krate, impl_, type_id) {
                continue;
            }
            for method_id in &impl_.items {
                if let Some(method_item) = krate.index.get(method_id) {
                    if let ItemEnum::Function(f) = &method_item.inner {
                        let method_name = method_item.name.as_deref().unwrap_or("_");
                        methods.push(MethodDoc {
                            name: method_name.to_string(),
                            docs: method_item.docs.clone(),
                            signature: render_function_sig(method_name, f),
                        });
                    }
                }
            }
        }
    }

    methods.sort_by(|a, b| a.name.cmp(&b.name));
    methods
}

/// Check if an impl block is for a specific type (matched by Id).
fn impl_is_for_type(krate: &Crate, impl_: &rustdoc_types::Impl, type_id: &Id) -> bool {
    // The impl's `for_` field is a Type. For resolved paths, we can check the id.
    if let Type::ResolvedPath(path) = &impl_.for_ {
        return path.id == *type_id;
    }
    // Fallback: check if this impl's parent path matches
    // For MVP, the ResolvedPath check covers the common case
    let _ = krate; // suppress unused warning
    false
}

/// Collect methods defined in a trait.
fn collect_trait_methods(krate: &Crate, trait_: &rustdoc_types::Trait) -> Vec<MethodDoc> {
    let mut methods = Vec::new();

    for method_id in &trait_.items {
        if let Some(method_item) = krate.index.get(method_id) {
            if let ItemEnum::Function(f) = &method_item.inner {
                let method_name = method_item.name.as_deref().unwrap_or("_");
                methods.push(MethodDoc {
                    name: method_name.to_string(),
                    docs: method_item.docs.clone(),
                    signature: render_function_sig(method_name, f),
                });
            }
        }
    }

    methods
}

// ---------------------------------------------------------------------------
// Signature rendering
// ---------------------------------------------------------------------------

fn render_function_sig(name: &str, func: &rustdoc_types::Function) -> String {
    let generics_str = render_generics(&func.generics);
    let params = render_fn_params(&func.sig);
    let ret = render_fn_return(&func.sig);
    let where_clause = render_where_clause(&func.generics);
    let header = render_fn_header(&func.header);

    let mut sig = format!("{header}fn {name}{generics_str}({params}){ret}");
    if !where_clause.is_empty() {
        sig.push_str(&format!("\nwhere\n{where_clause}"));
    }
    sig
}

fn render_fn_header(header: &rustdoc_types::FunctionHeader) -> String {
    let mut parts = Vec::new();
    if header.is_unsafe {
        parts.push("unsafe ");
    }
    if header.is_const {
        parts.push("const ");
    }
    if header.is_async {
        parts.push("async ");
    }
    // We always show pub for simplicity in docs
    parts.insert(0, "pub ");
    parts.join("")
}

fn render_fn_params(sig: &rustdoc_types::FunctionSignature) -> String {
    sig.inputs
        .iter()
        .map(|(name, ty)| {
            let type_str = render_type(ty);
            if name == "self" {
                // self parameters: render as-is from the type
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
                format!("{name}: {type_str}")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_fn_return(sig: &rustdoc_types::FunctionSignature) -> String {
    match &sig.output {
        Some(ty) => {
            let type_str = render_type(ty);
            format!(" -> {type_str}")
        }
        None => String::new(),
    }
}

fn render_struct_sig(name: &str, s: &rustdoc_types::Struct, _item: &Item) -> String {
    let generics_str = render_generics(&s.generics);
    let where_clause = render_where_clause(&s.generics);

    let fields = match &s.kind {
        rustdoc_types::StructKind::Unit => {
            let mut sig = format!("pub struct {name}{generics_str}");
            if !where_clause.is_empty() {
                sig.push_str(&format!("\nwhere\n{where_clause}"));
            }
            sig.push(';');
            return sig;
        }
        rustdoc_types::StructKind::Tuple(fields) => {
            let field_strs: Vec<String> = fields
                .iter()
                .map(|f| {
                    f.as_ref()
                        .map(|_| "_".to_string())
                        .unwrap_or_else(|| "_".to_string())
                })
                .collect();
            let mut sig = format!("pub struct {name}{generics_str}({})", field_strs.join(", "));
            if !where_clause.is_empty() {
                sig.push_str(&format!("\nwhere\n{where_clause}"));
            }
            sig.push(';');
            return sig;
        }
        rustdoc_types::StructKind::Plain {
            fields,
            has_stripped_fields,
        } => (fields, *has_stripped_fields),
    };

    let (field_ids, has_stripped) = fields;
    let mut sig = format!("pub struct {name}{generics_str}");
    if !where_clause.is_empty() {
        sig.push_str(&format!("\nwhere\n{where_clause}"));
    }
    sig.push_str(" {\n");

    for _field_id in field_ids {
        // For MVP, we don't resolve field types from the index
        // since that requires looking up each field Id
        sig.push_str("    // ...\n");
        break; // Just indicate there are fields
    }

    if has_stripped {
        sig.push_str("    // some fields omitted\n");
    }

    sig.push('}');
    sig
}

fn render_enum_sig(krate: &Crate, name: &str, e: &rustdoc_types::Enum, _item: &Item) -> String {
    let generics_str = render_generics(&e.generics);
    let where_clause = render_where_clause(&e.generics);

    let mut sig = format!("pub enum {name}{generics_str}");
    if !where_clause.is_empty() {
        sig.push_str(&format!("\nwhere\n{where_clause}"));
    }
    sig.push_str(" {\n");

    for variant_id in &e.variants {
        if let Some(variant_item) = krate.index.get(variant_id) {
            let variant_name = variant_item.name.as_deref().unwrap_or("_");
            match &variant_item.inner {
                ItemEnum::Variant(v) => match &v.kind {
                    rustdoc_types::VariantKind::Plain => {
                        sig.push_str(&format!("    {variant_name},\n"));
                    }
                    rustdoc_types::VariantKind::Tuple(_) => {
                        sig.push_str(&format!("    {variant_name}(/* ... */),\n"));
                    }
                    rustdoc_types::VariantKind::Struct { .. } => {
                        sig.push_str(&format!("    {variant_name} {{ /* ... */ }},\n"));
                    }
                },
                _ => {
                    sig.push_str(&format!("    {variant_name},\n"));
                }
            }
        }
    }

    sig.push('}');
    sig
}

fn render_trait_sig(name: &str, t: &rustdoc_types::Trait, _item: &Item) -> String {
    let generics_str = render_generics(&t.generics);
    let where_clause = render_where_clause(&t.generics);

    let bounds = if t.bounds.is_empty() {
        String::new()
    } else {
        let bound_strs: Vec<String> = t.bounds.iter().map(render_generic_bound).collect();
        format!(": {}", bound_strs.join(" + "))
    };

    let mut sig = format!("pub trait {name}{generics_str}{bounds}");
    if !where_clause.is_empty() {
        sig.push_str(&format!("\nwhere\n{where_clause}"));
    }
    sig.push_str(" {\n    // ...\n}");
    sig
}

// ---------------------------------------------------------------------------
// Type rendering
// ---------------------------------------------------------------------------

pub fn render_type(ty: &Type) -> String {
    match ty {
        Type::ResolvedPath(path) => {
            let mut result = path.path.clone();
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
                // Synthetic params (from `impl Trait` in arg position) - skip for cleaner output
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
}
