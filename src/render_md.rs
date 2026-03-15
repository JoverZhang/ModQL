/// Render the internal documentation model to Markdown files.
///
/// Output structure: one surface index, one internal index, and paired module
/// files for the public surface and internal symbol view.
use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

use crate::convert::strip_visibility_prefix;
use crate::model::{
    ConstantDoc, CrateDoc, EnumDoc, FunctionDoc, ImplDoc, MethodDoc, ModuleDoc, StaticDoc,
    StructDoc, TraitDoc, TypeAliasDoc,
};
use crate::naming;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ViewKind {
    Surface,
    Internal,
}

impl ViewKind {
    fn title_prefix(self) -> &'static str {
        match self {
            Self::Surface => "",
            Self::Internal => "Internal ",
        }
    }

    fn file_name_for_module(self, qualified_name: &str) -> String {
        match self {
            Self::Surface => naming::module_file_name(qualified_name),
            Self::Internal => naming::internal_module_file_name(qualified_name),
        }
    }
}

/// Render all documentation pages to the output directory.
pub fn render(surface_doc: &CrateDoc, internal_doc: &CrateDoc, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("Failed to create output directory: {}", out_dir.display()))?;

    if out_dir.exists() {
        for entry in fs::read_dir(out_dir)
            .with_context(|| format!("Failed to read output directory: {}", out_dir.display()))?
        {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                fs::remove_file(&path)
                    .with_context(|| format!("Failed to remove stale file: {}", path.display()))?;
            }
        }
    }

    let surface_modules = collect_module_names(&surface_doc.modules);

    // Crate-level index is a single merged page (internal view, no separate
    // surface/internal split). Module pages still get the split.
    write_page(
        out_dir,
        naming::crate_index_file(),
        &render_crate_page(internal_doc, &surface_modules),
    )?;

    for module in &surface_doc.modules {
        render_module_file(module, out_dir, ViewKind::Surface, &surface_modules)?;
    }
    for module in &internal_doc.modules {
        render_module_file(module, out_dir, ViewKind::Internal, &surface_modules)?;
    }

    Ok(())
}

fn collect_module_names(modules: &[ModuleDoc]) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    collect_module_names_into(modules, &mut names);
    names
}

fn collect_module_names_into(modules: &[ModuleDoc], names: &mut BTreeSet<String>) {
    for module in modules {
        names.insert(module.qualified_name.clone());
        collect_module_names_into(&module.modules, names);
    }
}

fn render_module_file(
    module: &ModuleDoc,
    out_dir: &Path,
    view: ViewKind,
    surface_modules: &BTreeSet<String>,
) -> Result<()> {
    let content = render_module_page(module, view, surface_modules);
    let filename = view.file_name_for_module(&module.qualified_name);
    write_page(out_dir, &filename, &content)?;

    for sub in &module.modules {
        render_module_file(sub, out_dir, view, surface_modules)?;
    }

    Ok(())
}

fn write_page(out_dir: &Path, filename: &str, content: &str) -> Result<()> {
    let path = out_dir.join(filename);
    fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

fn render_crate_page(crate_doc: &CrateDoc, surface_modules: &BTreeSet<String>) -> String {
    let mut out = String::new();

    let _ = writeln!(out, "# Crate `{}`", crate_doc.name);
    out.push('\n');

    if let Some(ref docs) = crate_doc.docs {
        render_docs_paragraph(&mut out, docs);
    }

    if !crate_doc.modules.is_empty() {
        let _ = writeln!(out, "## Modules\n");
        render_module_listing(
            &mut out,
            &crate_doc.modules,
            ViewKind::Internal,
            surface_modules,
        );
        out.push('\n');
    }

    render_body_sections(&mut out, crate_doc, ViewKind::Internal);

    out
}

fn render_module_page(
    module: &ModuleDoc,
    view: ViewKind,
    surface_modules: &BTreeSet<String>,
) -> String {
    let mut out = String::new();

    let _ = writeln!(
        out,
        "# {}Module `{}`",
        view.title_prefix(),
        module.qualified_name
    );
    out.push('\n');
    let has_surface = surface_modules.contains(&module.qualified_name);
    render_view_link(&mut out, &module.qualified_name, view, has_surface);

    if let Some(ref docs) = module.docs {
        render_docs_paragraph(&mut out, docs);
    }

    if !module.modules.is_empty() {
        let _ = writeln!(out, "## Sub-modules\n");
        render_module_listing(&mut out, &module.modules, view, surface_modules);
        out.push('\n');
    }

    render_module_sections(&mut out, module, view);

    out
}

fn render_view_link(out: &mut String, qualified_name: &str, view: ViewKind, has_surface: bool) {
    match view {
        ViewKind::Surface => {
            let path = naming::internal_module_file_name(qualified_name);
            let _ = writeln!(out, "[Internal view]({path})\n");
        }
        ViewKind::Internal if has_surface => {
            let path = naming::module_file_name(qualified_name);
            let _ = writeln!(out, "[Surface view]({path})\n");
        }
        ViewKind::Internal => {}
    }
}

fn render_docs_paragraph(out: &mut String, docs: &str) {
    let trimmed = docs.trim();
    if trimmed.is_empty() {
        return;
    }
    out.push_str(trimmed);
    out.push_str("\n\n");
}

fn render_module_listing(
    out: &mut String,
    modules: &[ModuleDoc],
    view: ViewKind,
    surface_modules: &BTreeSet<String>,
) {
    let third_column = match view {
        ViewKind::Surface => "Internal",
        ViewKind::Internal => "Surface",
    };
    let _ = writeln!(out, "| Module | Summary | {third_column} |");
    let _ = writeln!(out, "|---|---|---|");

    for module in modules {
        let short = naming::short_name(&module.qualified_name);
        let summary = synopsis_text(&module.docs);
        let current_link = match view {
            ViewKind::Surface => naming::module_file_name(&module.qualified_name),
            ViewKind::Internal => naming::internal_module_file_name(&module.qualified_name),
        };
        let alternate = match view {
            ViewKind::Surface => format!(
                "[internal]({})",
                naming::internal_module_file_name(&module.qualified_name)
            ),
            ViewKind::Internal if surface_modules.contains(&module.qualified_name) => {
                format!(
                    "[surface]({})",
                    naming::module_file_name(&module.qualified_name)
                )
            }
            ViewKind::Internal => "-".to_string(),
        };
        let _ = writeln!(
            out,
            "| [`{short}`]({current_link}) | {} | {alternate} |",
            summary.unwrap_or_default()
        );
    }
}

fn render_body_sections(out: &mut String, crate_doc: &CrateDoc, view: ViewKind) {
    match view {
        ViewKind::Surface => render_surface_sections(
            out,
            &crate_doc.structs,
            &crate_doc.enums,
            &crate_doc.traits,
            &crate_doc.impls,
            &crate_doc.functions,
            &crate_doc.type_aliases,
            &crate_doc.constants,
            &crate_doc.statics,
        ),
        ViewKind::Internal => render_internal_sections(
            out,
            &crate_doc.structs,
            &crate_doc.enums,
            &crate_doc.traits,
            &crate_doc.impls,
            &crate_doc.functions,
            &crate_doc.type_aliases,
            &crate_doc.constants,
            &crate_doc.statics,
        ),
    }
}

fn render_module_sections(out: &mut String, module: &ModuleDoc, view: ViewKind) {
    match view {
        ViewKind::Surface => render_surface_sections(
            out,
            &module.structs,
            &module.enums,
            &module.traits,
            &module.impls,
            &module.functions,
            &module.type_aliases,
            &module.constants,
            &module.statics,
        ),
        ViewKind::Internal => render_internal_sections(
            out,
            &module.structs,
            &module.enums,
            &module.traits,
            &module.impls,
            &module.functions,
            &module.type_aliases,
            &module.constants,
            &module.statics,
        ),
    }
}

fn render_surface_sections(
    out: &mut String,
    structs: &[StructDoc],
    enums: &[EnumDoc],
    traits: &[TraitDoc],
    impls: &[ImplDoc],
    functions: &[FunctionDoc],
    type_aliases: &[TypeAliasDoc],
    constants: &[ConstantDoc],
    statics: &[StaticDoc],
) {
    // Type aliases and constants first, then types
    render_signature_block_section(
        out,
        "Type Aliases",
        type_aliases
            .iter()
            .map(|item| (None, item.signature.as_str(), true)),
        false,
    );
    render_signature_block_section(
        out,
        "Constants",
        constants
            .iter()
            .map(|item| (None, item.signature.as_str(), true)),
        false,
    );
    render_types_summary_section(out, structs, enums, traits);
    render_signature_block_section(
        out,
        "Functions",
        functions
            .iter()
            .map(|function| (None, function.signature.as_str(), true)),
        false,
    );
    render_surface_inherent_impls_section(out, impls);
    render_signature_block_section(
        out,
        "Statics",
        statics
            .iter()
            .map(|item| (None, item.signature.as_str(), true)),
        false,
    );
}

// ---------------------------------------------------------------------------
// Two-zone internal rendering
// ---------------------------------------------------------------------------

/// A view of an impl block filtered to only certain methods.
struct RenderedImplZone<'a> {
    impl_doc: &'a ImplDoc,
    methods: Vec<&'a MethodDoc>,
}

/// Items collected for a single zone (public or private).
struct InternalZoneItems<'a> {
    structs: Vec<&'a StructDoc>,
    enums: Vec<&'a EnumDoc>,
    traits: Vec<&'a TraitDoc>,
    impls: Vec<RenderedImplZone<'a>>,
    functions: Vec<&'a FunctionDoc>,
    type_aliases: Vec<&'a TypeAliasDoc>,
    constants: Vec<&'a ConstantDoc>,
    statics: Vec<&'a StaticDoc>,
}

impl<'a> InternalZoneItems<'a> {
    fn has_any(&self) -> bool {
        !self.structs.is_empty()
            || !self.enums.is_empty()
            || !self.traits.is_empty()
            || !self.impls.is_empty()
            || !self.functions.is_empty()
            || !self.type_aliases.is_empty()
            || !self.constants.is_empty()
            || !self.statics.is_empty()
    }
}

struct InternalZones<'a> {
    public: InternalZoneItems<'a>,
    private: InternalZoneItems<'a>,
}

fn split_impls_for_internal_zones<'a>(
    impls: &'a [ImplDoc],
) -> (Vec<RenderedImplZone<'a>>, Vec<RenderedImplZone<'a>>) {
    let mut public_impls = Vec::new();
    let mut private_impls = Vec::new();

    for impl_doc in impls {
        if impl_doc.trait_name.is_none() {
            // Inherent impl: split by method visibility
            let public_methods: Vec<_> = impl_doc.methods.iter().filter(|m| m.is_public).collect();
            let private_methods: Vec<_> =
                impl_doc.methods.iter().filter(|m| !m.is_public).collect();

            if !public_methods.is_empty() {
                public_impls.push(RenderedImplZone {
                    impl_doc,
                    methods: public_methods,
                });
            }
            if !private_methods.is_empty() {
                private_impls.push(RenderedImplZone {
                    impl_doc,
                    methods: private_methods,
                });
            }
            continue;
        }

        // Trait impl: assign whole block to a zone
        let public_trait_impl =
            impl_doc.target_is_public && impl_doc.trait_is_public.unwrap_or(true);
        let rendered = RenderedImplZone {
            impl_doc,
            methods: impl_doc.methods.iter().collect(),
        };

        if public_trait_impl {
            public_impls.push(rendered);
        } else {
            private_impls.push(rendered);
        }
    }

    (public_impls, private_impls)
}

fn partition_internal_page<'a>(
    structs: &'a [StructDoc],
    enums: &'a [EnumDoc],
    traits: &'a [TraitDoc],
    impls: &'a [ImplDoc],
    functions: &'a [FunctionDoc],
    type_aliases: &'a [TypeAliasDoc],
    constants: &'a [ConstantDoc],
    statics: &'a [StaticDoc],
) -> InternalZones<'a> {
    let (public_impls, private_impls) = split_impls_for_internal_zones(impls);

    InternalZones {
        public: InternalZoneItems {
            structs: structs.iter().filter(|s| s.is_public).collect(),
            enums: enums.iter().filter(|e| e.is_public).collect(),
            traits: traits.iter().filter(|t| t.is_public).collect(),
            impls: public_impls,
            functions: functions.iter().filter(|f| f.is_public).collect(),
            type_aliases: type_aliases.iter().filter(|t| t.is_public).collect(),
            constants: constants.iter().filter(|c| c.is_public).collect(),
            statics: statics.iter().filter(|s| s.is_public).collect(),
        },
        private: InternalZoneItems {
            structs: structs.iter().filter(|s| !s.is_public).collect(),
            enums: enums.iter().filter(|e| !e.is_public).collect(),
            traits: traits.iter().filter(|t| !t.is_public).collect(),
            impls: private_impls,
            functions: functions.iter().filter(|f| !f.is_public).collect(),
            type_aliases: type_aliases.iter().filter(|t| !t.is_public).collect(),
            constants: constants.iter().filter(|c| !c.is_public).collect(),
            statics: statics.iter().filter(|s| !s.is_public).collect(),
        },
    }
}

fn zone_section_title(base: &str, is_private_zone: bool) -> String {
    if is_private_zone {
        format!("{base} (private)")
    } else {
        base.to_string()
    }
}

fn render_internal_sections(
    out: &mut String,
    structs: &[StructDoc],
    enums: &[EnumDoc],
    traits: &[TraitDoc],
    impls: &[ImplDoc],
    functions: &[FunctionDoc],
    type_aliases: &[TypeAliasDoc],
    constants: &[ConstantDoc],
    statics: &[StaticDoc],
) {
    let zones = partition_internal_page(
        structs,
        enums,
        traits,
        impls,
        functions,
        type_aliases,
        constants,
        statics,
    );

    render_internal_zone(out, &zones.public, false);
    if zones.public.has_any() && zones.private.has_any() {
        out.push_str("---\n\n");
    }
    render_internal_zone(out, &zones.private, true);
}

fn render_internal_zone(out: &mut String, zone: &InternalZoneItems<'_>, is_private: bool) {
    render_zone_type_aliases_section(out, &zone.type_aliases, is_private);
    render_zone_constants_section(out, &zone.constants, is_private);
    render_zone_structs_section(out, &zone.structs, &zone.impls, is_private);
    render_zone_enums_section(out, &zone.enums, &zone.impls, is_private);
    render_zone_traits_section(out, &zone.traits, is_private);
    render_zone_impls_section(out, &zone.impls, is_private);
    render_zone_functions_section(out, &zone.functions, is_private);
    render_zone_statics_section(out, &zone.statics, is_private);
}

fn render_zone_structs_section(
    out: &mut String,
    structs: &[&StructDoc],
    impls: &[RenderedImplZone<'_>],
    is_private: bool,
) {
    if structs.is_empty() {
        return;
    }
    let _ = writeln!(out, "## {}\n", zone_section_title("Structs", is_private));
    for item in structs {
        let _ = writeln!(out, "```rust");
        if !item.derived_traits.is_empty() {
            let _ = writeln!(out, "#[derive({})]", item.derived_traits.join(", "));
        }
        if let Some(ref docs) = item.docs {
            write_doc_comments(out, Some(docs), "");
        }
        let _ = writeln!(out, "{}", item.signature);
        let _ = writeln!(out, "```\n");

        // Group trait implementations under this type
        let short_name = naming::short_name(&item.qualified_name);
        for zone in impls {
            if zone.impl_doc.trait_name.is_some() && zone.impl_doc.target_name == short_name {
                let _ = writeln!(out, "```rust");
                let _ = write!(out, "{}", render_impl_block_from_zone(zone));
                out.push('\n');
                let _ = writeln!(out, "```\n");
            }
        }
    }
}

fn render_zone_enums_section(
    out: &mut String,
    enums: &[&EnumDoc],
    impls: &[RenderedImplZone<'_>],
    is_private: bool,
) {
    if enums.is_empty() {
        return;
    }
    let _ = writeln!(out, "## {}\n", zone_section_title("Enums", is_private));
    for item in enums {
        let _ = writeln!(out, "```rust");
        if !item.derived_traits.is_empty() {
            let _ = writeln!(out, "#[derive({})]", item.derived_traits.join(", "));
        }
        if let Some(ref docs) = item.docs {
            write_doc_comments(out, Some(docs), "");
        }
        let _ = writeln!(out, "{}", item.signature);
        let _ = writeln!(out, "```\n");

        // Group trait implementations under this type
        let short_name = naming::short_name(&item.qualified_name);
        for zone in impls {
            if zone.impl_doc.trait_name.is_some() && zone.impl_doc.target_name == short_name {
                let _ = writeln!(out, "```rust");
                let _ = write!(out, "{}", render_impl_block_from_zone(zone));
                out.push('\n');
                let _ = writeln!(out, "```\n");
            }
        }
    }
}

fn render_zone_traits_section(out: &mut String, traits: &[&TraitDoc], is_private: bool) {
    if traits.is_empty() {
        return;
    }
    let _ = writeln!(out, "## {}\n", zone_section_title("Traits", is_private));
    for item in traits {
        let _ = writeln!(out, "```rust");
        if let Some(ref docs) = item.docs {
            write_doc_comments(out, Some(docs), "");
        }
        let _ = writeln!(out, "{}", item.signature);
        let _ = writeln!(out, "```\n");
    }
}

fn render_zone_impls_section(out: &mut String, impls: &[RenderedImplZone<'_>], is_private: bool) {
    // Only render inherent impls here; trait impls are grouped under their target types.
    let inherent: Vec<_> = impls
        .iter()
        .filter(|z| z.impl_doc.trait_name.is_none())
        .collect();

    if inherent.is_empty() {
        return;
    }

    let _ = writeln!(out, "## {}\n", zone_section_title("Impl", is_private));

    for zone in &inherent {
        let _ = writeln!(out, "```rust");
        let _ = write!(out, "{}", render_impl_block_from_zone(zone));
        out.push('\n');
        let _ = writeln!(out, "```\n");
    }
}

fn render_impl_block_from_zone(zone: &RenderedImplZone<'_>) -> String {
    if zone.methods.is_empty() {
        return ensure_decl_terminated(&zone.impl_doc.header);
    }

    let mut out = String::new();
    out.push_str(&zone.impl_doc.header);
    out.push_str(" {\n");
    for method in &zone.methods {
        write_doc_comments(&mut out, method.docs.as_deref(), "    ");
        for line in ensure_decl_terminated(&method.signature).lines() {
            out.push_str("    ");
            out.push_str(line);
            out.push('\n');
        }
        out.push('\n');
    }
    out.push('}');
    out
}

fn render_zone_functions_section(out: &mut String, functions: &[&FunctionDoc], is_private: bool) {
    if functions.is_empty() {
        return;
    }
    let _ = writeln!(out, "## {}\n", zone_section_title("Functions", is_private));
    let _ = writeln!(out, "```rust");
    for function in functions {
        write_doc_comments(out, function.docs.as_deref(), "");
        let _ = writeln!(out, "{}", ensure_decl_terminated(&function.signature));
        out.push('\n');
    }
    let _ = writeln!(out, "```");
    out.push('\n');
}

fn render_zone_type_aliases_section(out: &mut String, items: &[&TypeAliasDoc], is_private: bool) {
    if items.is_empty() {
        return;
    }
    let _ = writeln!(
        out,
        "## {}\n",
        zone_section_title("Type Aliases", is_private)
    );
    let _ = writeln!(out, "```rust");
    for item in items {
        write_doc_comments(out, item.docs.as_deref(), "");
        let _ = writeln!(out, "{}", item.signature);
        out.push('\n');
    }
    let _ = writeln!(out, "```");
    out.push('\n');
}

fn render_zone_constants_section(out: &mut String, items: &[&ConstantDoc], is_private: bool) {
    if items.is_empty() {
        return;
    }
    let _ = writeln!(out, "## {}\n", zone_section_title("Constants", is_private));
    let _ = writeln!(out, "```rust");
    for item in items {
        write_doc_comments(out, item.docs.as_deref(), "");
        let _ = writeln!(out, "{}", item.signature);
        out.push('\n');
    }
    let _ = writeln!(out, "```");
    out.push('\n');
}

fn render_zone_statics_section(out: &mut String, items: &[&StaticDoc], is_private: bool) {
    if items.is_empty() {
        return;
    }
    let _ = writeln!(out, "## {}\n", zone_section_title("Statics", is_private));
    let _ = writeln!(out, "```rust");
    for item in items {
        write_doc_comments(out, item.docs.as_deref(), "");
        let _ = writeln!(out, "{}", item.signature);
        out.push('\n');
    }
    let _ = writeln!(out, "```");
    out.push('\n');
}

fn render_types_summary_section(
    out: &mut String,
    structs: &[StructDoc],
    enums: &[EnumDoc],
    traits: &[TraitDoc],
) {
    if structs.is_empty() && enums.is_empty() && traits.is_empty() {
        return;
    }

    let _ = writeln!(out, "## Types\n");
    let _ = writeln!(out, "```rust");

    // Traits with methods inline
    for item in traits {
        let _ = write!(out, "{}", item.signature);
        out.push('\n');
    }

    // Structs with #[derive(...)] annotation
    for item in structs {
        if !item.derived_traits.is_empty() {
            let _ = writeln!(out, "#[derive({})]", item.derived_traits.join(", "));
        }
        let _ = writeln!(
            out,
            "{}",
            summarize_type_signature(&item.signature, TypeKind::Struct)
        );
    }

    // Enums with #[derive(...)] annotation
    for item in enums {
        if !item.derived_traits.is_empty() {
            let _ = writeln!(out, "#[derive({})]", item.derived_traits.join(", "));
        }
        let _ = writeln!(
            out,
            "{}",
            summarize_type_signature(&item.signature, TypeKind::Enum)
        );
    }

    let _ = writeln!(out, "```");
    out.push('\n');
}

/// Render inherent impl blocks (no trait impls) with methods expanded for surface view.
fn render_surface_inherent_impls_section(out: &mut String, impls: &[ImplDoc]) {
    let inherent: Vec<_> = impls
        .iter()
        .filter(|imp| imp.trait_name.is_none())
        .filter(|imp| !imp.methods.is_empty())
        .collect();

    if inherent.is_empty() {
        return;
    }

    let _ = writeln!(out, "## Impl\n");
    for impl_doc in &inherent {
        let _ = writeln!(out, "```rust");
        let _ = write!(out, "{}", impl_doc.header);
        out.push_str(" {\n");
        for method in &impl_doc.methods {
            let _ = writeln!(out, "    {}", ensure_decl_terminated(&method.signature));
        }
        out.push_str("}\n");
        let _ = writeln!(out, "```\n");
    }
}

fn render_signature_block_section<'a, I>(
    out: &mut String,
    title: &str,
    items: I,
    include_docs: bool,
) where
    I: IntoIterator<Item = (Option<&'a str>, &'a str, bool)>,
{
    let items: Vec<_> = items.into_iter().collect();
    if items.is_empty() {
        return;
    }

    let has_pub = items.iter().any(|(_, _, is_pub)| *is_pub);
    let has_private = items.iter().any(|(_, _, is_pub)| !*is_pub);

    let _ = writeln!(out, "## {title}\n");
    let _ = writeln!(out, "```rust");
    let mut seen_private = false;
    for (docs, signature, is_public) in items {
        if !seen_private && !is_public && has_pub && has_private && include_docs {
            seen_private = true;
            let _ = writeln!(out, "// -- private --\n");
        }
        if include_docs {
            write_doc_comments(out, docs, "");
        }
        let rendered = if include_docs {
            signature.to_string()
        } else {
            ensure_decl_terminated(signature)
        };
        let _ = writeln!(out, "{rendered}");
        out.push('\n');
    }
    let _ = writeln!(out, "```");
    out.push('\n');
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TypeKind {
    Struct,
    Enum,
}

fn summarize_type_signature(signature: &str, kind: TypeKind) -> String {
    let trimmed = signature.trim();
    let body_delim = match kind {
        TypeKind::Struct => {
            // Skip the visibility prefix to avoid matching the parentheses
            // in pub(crate) or pub(in ...) as struct body delimiters.
            let without_vis = strip_visibility_prefix(trimmed);
            let offset = trimmed.len() - without_vis.len();
            find_decl_delimiter(without_vis)
                .map(|d| d + offset)
                .unwrap_or(trimmed.len())
        }
        TypeKind::Enum => trimmed.find('{').unwrap_or(trimmed.len()),
    };
    let summary = trimmed[..body_delim].trim_end();
    let mut summary = summary.to_string();
    if !summary.ends_with(';') {
        summary.push(';');
    }
    summary
}

fn find_decl_delimiter(signature: &str) -> Option<usize> {
    let mut generic_depth = 0;
    for (idx, ch) in signature.char_indices() {
        match ch {
            '<' => generic_depth += 1,
            '>' if generic_depth > 0 => generic_depth -= 1,
            '{' | '(' if generic_depth == 0 => return Some(idx),
            _ => {}
        }
    }
    None
}

fn ensure_decl_terminated(signature: &str) -> String {
    let mut signature = signature.trim().to_string();
    if !signature.ends_with(';') {
        signature.push(';');
    }
    signature
}

fn write_doc_comments(out: &mut String, docs: Option<&str>, indent: &str) {
    let Some(docs) = docs else {
        return;
    };
    let trimmed = docs.trim();
    if trimmed.is_empty() {
        return;
    }

    for line in trimmed.lines() {
        if line.trim().is_empty() {
            let _ = writeln!(out, "{indent}///");
        } else {
            let _ = writeln!(out, "{indent}/// {}", line.trim());
        }
    }
}

fn synopsis_text(docs: &Option<String>) -> Option<String> {
    naming::synopsis(docs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeSet;

    fn empty_crate_doc(name: &str) -> CrateDoc {
        CrateDoc {
            name: name.to_string(),
            docs: None,
            modules: Vec::new(),
            impls: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            traits: Vec::new(),
            functions: Vec::new(),
            type_aliases: Vec::new(),
            constants: Vec::new(),
            statics: Vec::new(),
        }
    }

    #[test]
    fn test_crate_page_no_items() {
        let doc = empty_crate_doc("mycrate");
        let page = render_crate_page(&doc, &BTreeSet::new());
        assert_eq!(page, "# Crate `mycrate`\n\n");
    }

    #[test]
    fn test_render_module_listing_surface() {
        let module = ModuleDoc {
            qualified_name: "mycrate::utils".to_string(),
            docs: Some("Utility helpers.".to_string()),
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
        let mut out = String::new();
        render_module_listing(
            &mut out,
            &[module],
            ViewKind::Surface,
            &BTreeSet::from(["mycrate::utils".to_string()]),
        );
        assert!(out.contains("| [`utils`](module.mycrate.utils.md) | Utility helpers. | [internal](module.mycrate.utils.internal.md) |"));
    }

    #[test]
    fn test_render_impl_block_from_zone() {
        let impl_doc = ImplDoc {
            header: "impl Greeter".to_string(),
            docs: None,
            target_name: "Greeter".to_string(),
            target_is_public: true,
            trait_name: None,
            trait_is_public: None,
            methods: vec![MethodDoc {
                name: "new".to_string(),
                docs: Some("Create a greeter.".to_string()),
                signature: "pub fn new(name: &str) -> Self".to_string(),
                is_public: true,
            }],
        };
        let zone = RenderedImplZone {
            impl_doc: &impl_doc,
            methods: impl_doc.methods.iter().collect(),
        };
        let block = render_impl_block_from_zone(&zone);
        assert!(block.contains("impl Greeter {"));
        assert!(block.contains("/// Create a greeter."));
        assert!(block.contains("pub fn new(name: &str) -> Self;"));
    }

    #[test]
    fn test_ensure_decl_terminated() {
        assert_eq!(ensure_decl_terminated("pub fn run()"), "pub fn run();");
        assert_eq!(ensure_decl_terminated("pub fn run();"), "pub fn run();");
    }

    #[test]
    fn test_crate_page_renders_internal_view() {
        let mut doc = empty_crate_doc("mycrate");
        doc.structs.push(StructDoc {
            qualified_name: "mycrate::Greeter".to_string(),
            docs: Some("Greeter docs.".to_string()),
            signature: "pub struct Greeter {\n    pub name: String,\n}".to_string(),
            fields: vec![FieldDoc {
                name: "name".to_string(),
                type_str: "String".to_string(),
                docs: Some("Field docs.".to_string()),
                is_public: true,
            }],
            is_public: true,
            derived_traits: Vec::new(),
        });
        doc.functions.push(FunctionDoc {
            qualified_name: "mycrate::run".to_string(),
            docs: Some("Run docs.".to_string()),
            signature: "pub fn run() -> String".to_string(),
            is_public: true,
        });
        doc.impls.push(ImplDoc {
            header: "impl Greeter".to_string(),
            docs: Some("Impl docs.".to_string()),
            target_name: "Greeter".to_string(),
            target_is_public: true,
            trait_name: None,
            trait_is_public: None,
            methods: vec![MethodDoc {
                name: "new".to_string(),
                docs: Some("Method docs.".to_string()),
                signature: "pub fn new(name: &str) -> Self".to_string(),
                is_public: true,
            }],
        });

        let page = render_crate_page(&doc, &BTreeSet::new());
        // Title has no "Internal" prefix
        assert!(page.contains("# Crate `mycrate`"));
        assert!(!page.contains("Internal Crate"));
        // No cross-link to internal view
        assert!(!page.contains("[Internal view]"));
        // Internal format: full detail with inline doc comments
        assert!(page.contains("## Structs"));
        assert!(page.contains("/// Greeter docs."));
        assert!(page.contains("pub struct Greeter"));
        assert!(page.contains("impl Greeter {"));
        assert!(page.contains("/// Method docs."));
        assert!(page.contains("pub fn new(name: &str) -> Self;"));
        assert!(!page.contains("#### Fields"));
    }

    #[test]
    fn test_crate_page_detailed_sections() {
        let mut doc = empty_crate_doc("mycrate");
        doc.structs.push(StructDoc {
            qualified_name: "mycrate::Greeter".to_string(),
            docs: Some("Greeter docs.".to_string()),
            signature: "pub struct Greeter {\n    pub name: String,\n}".to_string(),
            fields: vec![FieldDoc {
                name: "name".to_string(),
                type_str: "String".to_string(),
                docs: Some("Field docs.".to_string()),
                is_public: true,
            }],
            is_public: true,
            derived_traits: Vec::new(),
        });

        let page = render_crate_page(&doc, &BTreeSet::new());
        assert!(page.contains("## Structs"));
        // Internal view renders docs as inline /// comments in code blocks
        assert!(page.contains("/// Greeter docs."));
        assert!(page.contains("pub struct Greeter"));
        // No ### or #### headings in the new format
        assert!(!page.contains("#### Fields"));
    }

    #[test]
    fn test_zone_section_title() {
        assert_eq!(zone_section_title("Structs", false), "Structs");
        assert_eq!(zone_section_title("Structs", true), "Structs (private)");
        assert_eq!(
            zone_section_title("Impl Blocks", true),
            "Impl Blocks (private)"
        );
    }

    #[test]
    fn test_split_impls_inherent_mixed() {
        let impl_doc = ImplDoc {
            header: "impl Foo".to_string(),
            docs: None,
            target_name: "Foo".to_string(),
            target_is_public: true,
            trait_name: None,
            trait_is_public: None,
            methods: vec![
                MethodDoc {
                    name: "pub_method".to_string(),
                    docs: None,
                    signature: "pub fn pub_method()".to_string(),
                    is_public: true,
                },
                MethodDoc {
                    name: "priv_method".to_string(),
                    docs: None,
                    signature: "fn priv_method()".to_string(),
                    is_public: false,
                },
            ],
        };
        let impls = [impl_doc];
        let (public, private) = split_impls_for_internal_zones(&impls);
        assert_eq!(public.len(), 1, "Should have one public zone entry");
        assert_eq!(private.len(), 1, "Should have one private zone entry");
        assert_eq!(public[0].methods.len(), 1);
        assert_eq!(public[0].methods[0].name, "pub_method");
        assert_eq!(private[0].methods.len(), 1);
        assert_eq!(private[0].methods[0].name, "priv_method");
    }

    #[test]
    fn test_split_impls_trait_impl_zone_assignment() {
        let pub_trait_pub_target = ImplDoc {
            header: "impl Render for Foo".to_string(),
            docs: None,
            target_name: "Foo".to_string(),
            target_is_public: true,
            trait_name: Some("Render".to_string()),
            trait_is_public: Some(true),
            methods: vec![MethodDoc {
                name: "render".to_string(),
                docs: None,
                signature: "fn render(&self) -> String".to_string(),
                is_public: false, // trait impl methods don't have pub
            }],
        };
        let priv_trait = ImplDoc {
            header: "impl Validate for Foo".to_string(),
            docs: None,
            target_name: "Foo".to_string(),
            target_is_public: true,
            trait_name: Some("Validate".to_string()),
            trait_is_public: Some(false),
            methods: Vec::new(),
        };
        let impls = [pub_trait_pub_target, priv_trait];
        let (public, private) = split_impls_for_internal_zones(&impls);
        assert_eq!(
            public.len(),
            1,
            "Public trait impl should go to public zone"
        );
        assert_eq!(
            private.len(),
            1,
            "Private trait impl should go to private zone"
        );
        assert_eq!(public[0].impl_doc.header, "impl Render for Foo");
        assert_eq!(private[0].impl_doc.header, "impl Validate for Foo");
    }

    #[test]
    fn test_partition_page_no_private_items() {
        let structs = vec![StructDoc {
            qualified_name: "mycrate::Foo".to_string(),
            docs: None,
            signature: "pub struct Foo;".to_string(),
            fields: Vec::new(),
            is_public: true,
            derived_traits: Vec::new(),
        }];
        let zones = partition_internal_page(&structs, &[], &[], &[], &[], &[], &[], &[]);
        assert!(zones.public.has_any());
        assert!(!zones.private.has_any());
    }

    #[test]
    fn test_two_zone_page_rendering() {
        let mut doc = empty_crate_doc("mycrate");
        doc.functions.push(FunctionDoc {
            qualified_name: "mycrate::pub_fn".to_string(),
            docs: None,
            signature: "pub fn pub_fn()".to_string(),
            is_public: true,
        });
        doc.functions.push(FunctionDoc {
            qualified_name: "mycrate::priv_fn".to_string(),
            docs: None,
            signature: "fn priv_fn()".to_string(),
            is_public: false,
        });
        let page = render_crate_page(&doc, &BTreeSet::new());
        assert!(
            page.contains("## Functions\n"),
            "Should have public functions section"
        );
        assert!(
            page.contains("## Functions (private)\n"),
            "Should have private functions section"
        );
        assert!(page.contains("\n---\n"), "Should have page-level separator");
        assert!(
            !page.contains("// -- private --"),
            "Should not have old separators"
        );
        // Verify order
        let pub_pos = page.find("## Functions\n").unwrap();
        let sep_pos = page.find("\n---\n").unwrap();
        let priv_pos = page.find("## Functions (private)\n").unwrap();
        assert!(pub_pos < sep_pos);
        assert!(sep_pos < priv_pos);
    }
}
