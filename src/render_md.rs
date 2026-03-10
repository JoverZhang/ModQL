/// Render the internal documentation model to Markdown files.
use anyhow::{Context, Result};
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

use crate::model::{
    ConstantDoc, CrateDoc, EnumDoc, FunctionDoc, MethodDoc, ModuleDoc, StaticDoc, StructDoc,
    TraitDoc, TypeAliasDoc,
};
use crate::naming;

/// Render all documentation pages to the output directory.
pub fn render(crate_doc: &CrateDoc, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("Failed to create output directory: {}", out_dir.display()))?;

    // Crate index page
    let index_content = render_crate_page(crate_doc);
    write_page(out_dir, naming::crate_index_file(), &index_content)?;

    // Recursively render all items
    render_items(crate_doc, out_dir)?;

    Ok(())
}

/// Render all items from the crate root.
fn render_items(crate_doc: &CrateDoc, out_dir: &Path) -> Result<()> {
    for module in &crate_doc.modules {
        render_module_recursive(module, &crate_doc.name, out_dir)?;
    }
    for s in &crate_doc.structs {
        let content = render_struct_page(s);
        let filename = naming::item_file_name("struct", &s.qualified_name);
        write_page(out_dir, &filename, &content)?;
    }
    for e in &crate_doc.enums {
        let content = render_enum_page(e);
        let filename = naming::item_file_name("enum", &e.qualified_name);
        write_page(out_dir, &filename, &content)?;
    }
    for t in &crate_doc.traits {
        let content = render_trait_page(t);
        let filename = naming::item_file_name("trait", &t.qualified_name);
        write_page(out_dir, &filename, &content)?;
    }
    for f in &crate_doc.functions {
        let content = render_function_page(f);
        let filename = naming::item_file_name("function", &f.qualified_name);
        write_page(out_dir, &filename, &content)?;
    }
    // Type aliases, constants, statics don't get their own pages in MVP
    // They are listed on the parent page
    Ok(())
}

/// Recursively render a module and its contents.
fn render_module_recursive(module: &ModuleDoc, crate_name: &str, out_dir: &Path) -> Result<()> {
    let content = render_module_page(module, crate_name);
    let filename = naming::item_file_name("module", &module.qualified_name);
    write_page(out_dir, &filename, &content)?;

    for sub in &module.modules {
        render_module_recursive(sub, crate_name, out_dir)?;
    }
    for s in &module.structs {
        let content = render_struct_page(s);
        let filename = naming::item_file_name("struct", &s.qualified_name);
        write_page(out_dir, &filename, &content)?;
    }
    for e in &module.enums {
        let content = render_enum_page(e);
        let filename = naming::item_file_name("enum", &e.qualified_name);
        write_page(out_dir, &filename, &content)?;
    }
    for t in &module.traits {
        let content = render_trait_page(t);
        let filename = naming::item_file_name("trait", &t.qualified_name);
        write_page(out_dir, &filename, &content)?;
    }
    for f in &module.functions {
        let content = render_function_page(f);
        let filename = naming::item_file_name("function", &f.qualified_name);
        write_page(out_dir, &filename, &content)?;
    }

    Ok(())
}

fn write_page(out_dir: &Path, filename: &str, content: &str) -> Result<()> {
    let path = out_dir.join(filename);
    fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Page rendering
// ---------------------------------------------------------------------------

pub fn render_crate_page(crate_doc: &CrateDoc) -> String {
    let mut out = String::new();

    let _ = writeln!(out, "# Crate `{}`", crate_doc.name);
    out.push('\n');

    if let Some(ref docs) = crate_doc.docs {
        if !docs.trim().is_empty() {
            out.push_str(docs.trim());
            out.push_str("\n\n");
        }
    }

    render_item_listing(
        &mut out,
        "Modules",
        &crate_doc.modules,
        |m| &m.qualified_name,
        |m| &m.docs,
        "module",
    );
    render_item_listing(
        &mut out,
        "Structs",
        &crate_doc.structs,
        |s| &s.qualified_name,
        |s| &s.docs,
        "struct",
    );
    render_item_listing(
        &mut out,
        "Enums",
        &crate_doc.enums,
        |e| &e.qualified_name,
        |e| &e.docs,
        "enum",
    );
    render_item_listing(
        &mut out,
        "Traits",
        &crate_doc.traits,
        |t| &t.qualified_name,
        |t| &t.docs,
        "trait",
    );
    render_item_listing(
        &mut out,
        "Functions",
        &crate_doc.functions,
        |f| &f.qualified_name,
        |f| &f.docs,
        "function",
    );
    render_type_alias_listing(&mut out, &crate_doc.type_aliases);
    render_constant_listing(&mut out, &crate_doc.constants);
    render_static_listing(&mut out, &crate_doc.statics);

    out
}

pub fn render_module_page(module: &ModuleDoc, _crate_name: &str) -> String {
    let mut out = String::new();

    let _ = writeln!(out, "# Module `{}`", module.qualified_name);
    out.push('\n');

    if let Some(ref docs) = module.docs {
        if !docs.trim().is_empty() {
            out.push_str(docs.trim());
            out.push_str("\n\n");
        }
    }

    render_item_listing(
        &mut out,
        "Modules",
        &module.modules,
        |m| &m.qualified_name,
        |m| &m.docs,
        "module",
    );
    render_item_listing(
        &mut out,
        "Structs",
        &module.structs,
        |s| &s.qualified_name,
        |s| &s.docs,
        "struct",
    );
    render_item_listing(
        &mut out,
        "Enums",
        &module.enums,
        |e| &e.qualified_name,
        |e| &e.docs,
        "enum",
    );
    render_item_listing(
        &mut out,
        "Traits",
        &module.traits,
        |t| &t.qualified_name,
        |t| &t.docs,
        "trait",
    );
    render_item_listing(
        &mut out,
        "Functions",
        &module.functions,
        |f| &f.qualified_name,
        |f| &f.docs,
        "function",
    );
    render_type_alias_listing(&mut out, &module.type_aliases);
    render_constant_listing(&mut out, &module.constants);
    render_static_listing(&mut out, &module.statics);

    out
}

pub fn render_struct_page(s: &StructDoc) -> String {
    let mut out = String::new();

    let _ = writeln!(out, "# Struct `{}`", s.qualified_name);
    out.push('\n');

    let _ = writeln!(out, "```rust\n{}\n```", s.signature);
    out.push('\n');

    if let Some(ref docs) = s.docs {
        if !docs.trim().is_empty() {
            out.push_str(docs.trim());
            out.push_str("\n\n");
        }
    }

    render_methods_section(&mut out, &s.methods);

    out
}

pub fn render_enum_page(e: &EnumDoc) -> String {
    let mut out = String::new();

    let _ = writeln!(out, "# Enum `{}`", e.qualified_name);
    out.push('\n');

    let _ = writeln!(out, "```rust\n{}\n```", e.signature);
    out.push('\n');

    if let Some(ref docs) = e.docs {
        if !docs.trim().is_empty() {
            out.push_str(docs.trim());
            out.push_str("\n\n");
        }
    }

    render_methods_section(&mut out, &e.methods);

    out
}

pub fn render_trait_page(t: &TraitDoc) -> String {
    let mut out = String::new();

    let _ = writeln!(out, "# Trait `{}`", t.qualified_name);
    out.push('\n');

    let _ = writeln!(out, "```rust\n{}\n```", t.signature);
    out.push('\n');

    if let Some(ref docs) = t.docs {
        if !docs.trim().is_empty() {
            out.push_str(docs.trim());
            out.push_str("\n\n");
        }
    }

    render_methods_section(&mut out, &t.methods);

    out
}

pub fn render_function_page(f: &FunctionDoc) -> String {
    let mut out = String::new();

    let _ = writeln!(out, "# Function `{}`", f.qualified_name);
    out.push('\n');

    let _ = writeln!(out, "```rust\n{}\n```", f.signature);
    out.push('\n');

    if let Some(ref docs) = f.docs {
        if !docs.trim().is_empty() {
            out.push_str(docs.trim());
            out.push_str("\n\n");
        }
    }

    out
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn render_item_listing<T, FName, FDocs>(
    out: &mut String,
    section_title: &str,
    items: &[T],
    get_name: FName,
    get_docs: FDocs,
    kind: &str,
) where
    FName: Fn(&T) -> &str,
    FDocs: Fn(&T) -> &Option<String>,
{
    if items.is_empty() {
        return;
    }

    let _ = writeln!(out, "## {section_title}\n");

    for item in items {
        let qualified = get_name(item);
        let short = naming::short_name(qualified);
        let link = naming::relative_link(kind, qualified);
        let synopsis = naming::synopsis(get_docs(item))
            .map(|s| format!(" - {s}"))
            .unwrap_or_default();

        let _ = writeln!(out, "- [`{short}`]({link}){synopsis}");
    }

    out.push('\n');
}

fn render_type_alias_listing(out: &mut String, items: &[TypeAliasDoc]) {
    if items.is_empty() {
        return;
    }

    let _ = writeln!(out, "## Type Aliases\n");
    for item in items {
        let short = naming::short_name(&item.qualified_name);
        let synopsis = naming::synopsis(&item.docs)
            .map(|s| format!(" - {s}"))
            .unwrap_or_default();
        let _ = writeln!(out, "- `{short}`{synopsis}");
    }
    out.push('\n');
}

fn render_constant_listing(out: &mut String, items: &[ConstantDoc]) {
    if items.is_empty() {
        return;
    }

    let _ = writeln!(out, "## Constants\n");
    for item in items {
        let short = naming::short_name(&item.qualified_name);
        let synopsis = naming::synopsis(&item.docs)
            .map(|s| format!(" - {s}"))
            .unwrap_or_default();
        let _ = writeln!(out, "- `{short}`{synopsis}");
    }
    out.push('\n');
}

fn render_static_listing(out: &mut String, items: &[StaticDoc]) {
    if items.is_empty() {
        return;
    }

    let _ = writeln!(out, "## Statics\n");
    for item in items {
        let short = naming::short_name(&item.qualified_name);
        let synopsis = naming::synopsis(&item.docs)
            .map(|s| format!(" - {s}"))
            .unwrap_or_default();
        let _ = writeln!(out, "- `{short}`{synopsis}");
    }
    out.push('\n');
}

fn render_methods_section(out: &mut String, methods: &[MethodDoc]) {
    if methods.is_empty() {
        return;
    }

    let _ = writeln!(out, "## Methods\n");

    for method in methods {
        let _ = writeln!(out, "### `{}`\n", method_heading_sig(&method.signature));
        let _ = writeln!(out, "```rust\n{}\n```\n", method.signature);

        if let Some(ref docs) = method.docs {
            if !docs.trim().is_empty() {
                out.push_str(docs.trim());
                out.push_str("\n\n");
            }
        }
    }
}

/// Extract a compact heading from a method signature.
/// e.g. "pub fn new(name: &str) -> Self" -> "fn new(name: &str) -> Self"
fn method_heading_sig(sig: &str) -> String {
    let s = sig.trim();
    // Strip pub/unsafe/const/async prefixes for the heading
    let s = s.strip_prefix("pub ").unwrap_or(s);
    s.lines().next().unwrap_or(s).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    use pretty_assertions::assert_eq;

    fn empty_crate_doc(name: &str) -> CrateDoc {
        CrateDoc {
            name: name.to_string(),
            docs: None,
            modules: Vec::new(),
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
        let page = render_crate_page(&doc);
        assert_eq!(page, "# Crate `mycrate`\n\n");
    }

    #[test]
    fn test_crate_page_with_docs() {
        let mut doc = empty_crate_doc("mycrate");
        doc.docs = Some("This is my crate.".to_string());
        let page = render_crate_page(&doc);
        assert!(page.contains("# Crate `mycrate`"));
        assert!(page.contains("This is my crate."));
    }

    #[test]
    fn test_crate_page_with_structs() {
        let mut doc = empty_crate_doc("mycrate");
        doc.structs.push(StructDoc {
            qualified_name: "mycrate::Foo".to_string(),
            docs: Some("A foo struct.".to_string()),
            signature: "pub struct Foo;".to_string(),
            methods: Vec::new(),
        });
        let page = render_crate_page(&doc);
        assert!(page.contains("## Structs"));
        assert!(page.contains("[`Foo`](struct.mycrate.Foo.md)"));
        assert!(page.contains("A foo struct."));
    }

    #[test]
    fn test_empty_sections_omitted() {
        let doc = empty_crate_doc("mycrate");
        let page = render_crate_page(&doc);
        assert!(!page.contains("## Modules"));
        assert!(!page.contains("## Structs"));
        assert!(!page.contains("## Enums"));
        assert!(!page.contains("## Traits"));
        assert!(!page.contains("## Functions"));
    }

    #[test]
    fn test_struct_page_with_methods() {
        let s = StructDoc {
            qualified_name: "mycrate::Foo".to_string(),
            docs: Some("A foo struct.".to_string()),
            signature: "pub struct Foo;".to_string(),
            methods: vec![MethodDoc {
                name: "new".to_string(),
                docs: Some("Create a new Foo.".to_string()),
                signature: "pub fn new() -> Self".to_string(),
            }],
        };
        let page = render_struct_page(&s);
        assert!(page.contains("# Struct `mycrate::Foo`"));
        assert!(page.contains("```rust\npub struct Foo;\n```"));
        assert!(page.contains("A foo struct."));
        assert!(page.contains("## Methods"));
        assert!(page.contains("### `fn new() -> Self`"));
        assert!(page.contains("Create a new Foo."));
    }

    #[test]
    fn test_function_page() {
        let f = FunctionDoc {
            qualified_name: "mycrate::run".to_string(),
            docs: Some("Run the app.".to_string()),
            signature: "pub fn run() -> String".to_string(),
        };
        let page = render_function_page(&f);
        assert!(page.contains("# Function `mycrate::run`"));
        assert!(page.contains("```rust\npub fn run() -> String\n```"));
        assert!(page.contains("Run the app."));
    }

    #[test]
    fn test_module_page() {
        let module = ModuleDoc {
            qualified_name: "mycrate::utils".to_string(),
            docs: Some("Utility functions.".to_string()),
            modules: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            traits: Vec::new(),
            functions: vec![FunctionDoc {
                qualified_name: "mycrate::utils::helper".to_string(),
                docs: Some("A helper function.".to_string()),
                signature: "pub fn helper()".to_string(),
            }],
            type_aliases: Vec::new(),
            constants: Vec::new(),
            statics: Vec::new(),
        };
        let page = render_module_page(&module, "mycrate");
        assert!(page.contains("# Module `mycrate::utils`"));
        assert!(page.contains("Utility functions."));
        assert!(page.contains("## Functions"));
        assert!(page.contains("[`helper`](function.mycrate.utils.helper.md)"));
    }

    #[test]
    fn test_method_heading_strips_pub() {
        assert_eq!(
            method_heading_sig("pub fn new() -> Self"),
            "fn new() -> Self"
        );
        assert_eq!(
            method_heading_sig("pub unsafe fn dangerous()"),
            "unsafe fn dangerous()"
        );
    }
}
