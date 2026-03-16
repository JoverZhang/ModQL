/// Build a type dependency graph from rustdoc JSON and render it as Mermaid-in-Markdown.
///
/// This module operates directly on `rustdoc_types::Crate`, independent of the
/// `convert` + `render_md` pipeline. It extracts crate-local type nodes (structs,
/// enums, traits, type aliases, functions) and the dependency edges between them
/// (field types, param/return types, generic bounds, trait impls, alias targets).
use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::Result;
use rustdoc_types::{
    Crate, GenericArg, GenericArgs, GenericBound, Id, ItemEnum, Type, WherePredicate,
};

use crate::naming;

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// The kind of a node in the type graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeKind {
    Struct,
    Enum,
    Trait,
    TypeAlias,
    Function,
}

impl NodeKind {
    fn label(self) -> &'static str {
        match self {
            NodeKind::Struct => "struct",
            NodeKind::Enum => "enum",
            NodeKind::Trait => "trait",
            NodeKind::TypeAlias => "type",
            NodeKind::Function => "fn",
        }
    }
}

/// A node representing a crate-local type or function.
#[derive(Debug, Clone)]
struct Node {
    name: String,
    kind: NodeKind,
}

/// The kind of relationship between two nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EdgeKind {
    /// Struct/enum field references a type.
    Field,
    /// Function parameter references a type.
    Param,
    /// Function/method return type references a type.
    Return,
    /// Generic bound references a trait.
    Bound,
    /// `impl Trait for Type` relationship.
    ImplTrait,
    /// Type alias RHS references a type.
    Alias,
}

impl EdgeKind {
    fn label(self) -> &'static str {
        match self {
            EdgeKind::Field => "field",
            EdgeKind::Param => "param",
            EdgeKind::Return => "return",
            EdgeKind::Bound => "bound",
            EdgeKind::ImplTrait => "impl",
            EdgeKind::Alias => "alias",
        }
    }
}

/// A directed edge from one node to another.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Edge {
    from: Id,
    to: Id,
    kind: EdgeKind,
}

/// The complete type dependency graph for one crate.
struct TypeGraph {
    nodes: HashMap<Id, Node>,
    edges: Vec<Edge>,
}

// ---------------------------------------------------------------------------
// Type reference extraction
// ---------------------------------------------------------------------------

/// Recursively collect all `ResolvedPath` IDs from a `Type`.
fn collect_type_refs(ty: &Type, out: &mut Vec<Id>) {
    match ty {
        Type::ResolvedPath(path) => {
            out.push(path.id.clone());
            if let Some(args) = &path.args {
                collect_generic_args_refs(args, out);
            }
        }
        Type::BorrowedRef { type_, .. } | Type::RawPointer { type_, .. } => {
            collect_type_refs(type_, out);
        }
        Type::Slice(ty) => collect_type_refs(ty, out),
        Type::Array { type_, .. } => collect_type_refs(type_, out),
        Type::Pat { type_, .. } => collect_type_refs(type_, out),
        Type::Tuple(types) => {
            for ty in types {
                collect_type_refs(ty, out);
            }
        }
        Type::QualifiedPath {
            self_type, trait_, ..
        } => {
            collect_type_refs(self_type, out);
            if let Some(trait_path) = trait_ {
                out.push(trait_path.id.clone());
                if let Some(args) = &trait_path.args {
                    collect_generic_args_refs(args, out);
                }
            }
        }
        Type::DynTrait(dyn_trait) => {
            for poly_trait in &dyn_trait.traits {
                out.push(poly_trait.trait_.id.clone());
                if let Some(args) = &poly_trait.trait_.args {
                    collect_generic_args_refs(args, out);
                }
            }
        }
        Type::ImplTrait(bounds) => {
            collect_bound_refs(bounds, out);
        }
        Type::FunctionPointer(fp) => {
            for (_name, ty) in &fp.sig.inputs {
                collect_type_refs(ty, out);
            }
            if let Some(ty) = &fp.sig.output {
                collect_type_refs(ty, out);
            }
        }
        // Primitives, generics, infer — no type refs.
        Type::Generic(_) | Type::Primitive(_) | Type::Infer => {}
    }
}

/// Collect type refs from generic arguments.
fn collect_generic_args_refs(args: &GenericArgs, out: &mut Vec<Id>) {
    match args {
        GenericArgs::AngleBracketed {
            args, constraints, ..
        } => {
            for arg in args {
                match arg {
                    GenericArg::Type(ty) => collect_type_refs(ty, out),
                    GenericArg::Lifetime(_) | GenericArg::Const(_) | GenericArg::Infer => {}
                }
            }
            for constraint in constraints {
                match &constraint.binding {
                    rustdoc_types::AssocItemConstraintKind::Equality(term) => {
                        if let rustdoc_types::Term::Type(ty) = term {
                            collect_type_refs(ty, out);
                        }
                    }
                    rustdoc_types::AssocItemConstraintKind::Constraint(bounds) => {
                        collect_bound_refs(bounds, out);
                    }
                }
            }
        }
        GenericArgs::Parenthesized { inputs, output } => {
            for ty in inputs {
                collect_type_refs(ty, out);
            }
            if let Some(ty) = output {
                collect_type_refs(ty, out);
            }
        }
        GenericArgs::ReturnTypeNotation => {}
    }
}

/// Collect type refs from generic bounds.
fn collect_bound_refs(bounds: &[GenericBound], out: &mut Vec<Id>) {
    for bound in bounds {
        if let GenericBound::TraitBound {
            trait_, modifier, ..
        } = bound
        {
            // Skip negative bounds (? bounds) — they don't represent a dependency.
            if !matches!(modifier, rustdoc_types::TraitBoundModifier::Maybe) {
                out.push(trait_.id.clone());
                if let Some(args) = &trait_.args {
                    collect_generic_args_refs(args, out);
                }
            }
        }
    }
}

/// Collect type refs from generics (params + where clauses).
fn collect_generics_refs(generics: &rustdoc_types::Generics, out: &mut Vec<Id>) {
    for param in &generics.params {
        if let rustdoc_types::GenericParamDefKind::Type {
            bounds, default, ..
        } = &param.kind
        {
            collect_bound_refs(bounds, out);
            if let Some(ty) = default {
                collect_type_refs(ty, out);
            }
        }
    }
    for pred in &generics.where_predicates {
        match pred {
            WherePredicate::BoundPredicate { type_, bounds, .. } => {
                collect_type_refs(type_, out);
                collect_bound_refs(bounds, out);
            }
            WherePredicate::EqPredicate { lhs, rhs } => {
                collect_type_refs(lhs, out);
                if let rustdoc_types::Term::Type(ty) = rhs {
                    collect_type_refs(ty, out);
                }
            }
            WherePredicate::LifetimePredicate { .. } => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Graph construction
// ---------------------------------------------------------------------------

/// Build a type dependency graph from a rustdoc JSON crate.
///
/// Only crate-local items appear as nodes. Edges point from an item to the
/// crate-local types it references through fields, parameters, return types,
/// generic bounds, trait impls, and type alias targets.
fn build_type_graph(krate: &Crate) -> TypeGraph {
    let mut nodes: HashMap<Id, Node> = HashMap::new();

    // --- Pre-pass: collect all IDs that are impl/trait methods (not standalone functions) ---
    let mut method_ids: HashSet<Id> = HashSet::new();
    for item in krate.index.values() {
        match &item.inner {
            ItemEnum::Impl(impl_) => {
                for method_id in &impl_.items {
                    method_ids.insert(method_id.clone());
                }
            }
            ItemEnum::Trait(t) => {
                for method_id in &t.items {
                    method_ids.insert(method_id.clone());
                }
            }
            _ => {}
        }
    }

    // --- Pass 1: collect crate-local type/function nodes ---
    for (id, item) in &krate.index {
        if item.crate_id != 0 {
            continue;
        }
        let Some(name) = &item.name else { continue };
        let kind = match &item.inner {
            ItemEnum::Struct(_) => NodeKind::Struct,
            ItemEnum::Enum(_) => NodeKind::Enum,
            ItemEnum::Trait(_) => NodeKind::Trait,
            ItemEnum::TypeAlias(_) => NodeKind::TypeAlias,
            ItemEnum::Function(_) => {
                // Skip methods that belong to impl/trait blocks — only include standalone functions.
                if method_ids.contains(id) {
                    continue;
                }
                NodeKind::Function
            }
            _ => continue,
        };
        nodes.insert(
            id.clone(),
            Node {
                name: name.clone(),
                kind,
            },
        );
    }

    let local_ids: HashSet<Id> = nodes.keys().cloned().collect();

    // --- Pass 2: collect edges ---
    let mut edge_set: HashSet<Edge> = HashSet::new();

    for (id, item) in &krate.index {
        if !local_ids.contains(id) {
            continue;
        }

        match &item.inner {
            ItemEnum::Struct(s) => {
                // Field type refs
                let field_ids = match &s.kind {
                    rustdoc_types::StructKind::Plain { fields, .. } => fields.clone(),
                    rustdoc_types::StructKind::Tuple(opts) => {
                        opts.iter().filter_map(|o| o.clone()).collect()
                    }
                    rustdoc_types::StructKind::Unit => Vec::new(),
                };
                for field_id in &field_ids {
                    if let Some(field_item) = krate.index.get(field_id) {
                        if let ItemEnum::StructField(ty) = &field_item.inner {
                            let mut refs = Vec::new();
                            collect_type_refs(ty, &mut refs);
                            for ref_id in refs {
                                if local_ids.contains(&ref_id) && ref_id != *id {
                                    edge_set.insert(Edge {
                                        from: id.clone(),
                                        to: ref_id,
                                        kind: EdgeKind::Field,
                                    });
                                }
                            }
                        }
                    }
                }
                // Generic bound refs
                let mut refs = Vec::new();
                collect_generics_refs(&s.generics, &mut refs);
                add_edges_from_refs(id, &refs, &local_ids, EdgeKind::Bound, &mut edge_set);

                // Impl block edges
                collect_impl_edges(krate, id, &s.impls, &local_ids, &mut edge_set);
            }
            ItemEnum::Enum(e) => {
                // Variant field type refs
                for variant_id in &e.variants {
                    if let Some(variant_item) = krate.index.get(variant_id) {
                        if let ItemEnum::Variant(v) = &variant_item.inner {
                            let field_ids = match &v.kind {
                                rustdoc_types::VariantKind::Plain => Vec::new(),
                                rustdoc_types::VariantKind::Tuple(opts) => {
                                    opts.iter().filter_map(|o| o.clone()).collect()
                                }
                                rustdoc_types::VariantKind::Struct {
                                    fields,
                                    has_stripped_fields: _,
                                } => fields.clone(),
                            };
                            for field_id in &field_ids {
                                if let Some(field_item) = krate.index.get(field_id) {
                                    if let ItemEnum::StructField(ty) = &field_item.inner {
                                        let mut refs = Vec::new();
                                        collect_type_refs(ty, &mut refs);
                                        add_edges_from_refs(
                                            id,
                                            &refs,
                                            &local_ids,
                                            EdgeKind::Field,
                                            &mut edge_set,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                // Generic bound refs
                let mut refs = Vec::new();
                collect_generics_refs(&e.generics, &mut refs);
                add_edges_from_refs(id, &refs, &local_ids, EdgeKind::Bound, &mut edge_set);

                // Impl block edges
                collect_impl_edges(krate, id, &e.impls, &local_ids, &mut edge_set);
            }
            ItemEnum::Trait(t) => {
                // Supertrait bounds
                let mut refs = Vec::new();
                collect_bound_refs(&t.bounds, &mut refs);
                collect_generics_refs(&t.generics, &mut refs);
                add_edges_from_refs(id, &refs, &local_ids, EdgeKind::Bound, &mut edge_set);

                // Trait method param/return refs
                for method_id in &t.items {
                    if let Some(method_item) = krate.index.get(method_id) {
                        if let ItemEnum::Function(f) = &method_item.inner {
                            collect_function_edges(id, f, &local_ids, &mut edge_set);
                        }
                    }
                }
            }
            ItemEnum::Function(f) => {
                collect_function_edges(id, f, &local_ids, &mut edge_set);
            }
            ItemEnum::TypeAlias(ta) => {
                let mut refs = Vec::new();
                collect_type_refs(&ta.type_, &mut refs);
                collect_generics_refs(&ta.generics, &mut refs);
                add_edges_from_refs(id, &refs, &local_ids, EdgeKind::Alias, &mut edge_set);
            }
            _ => {}
        }
    }

    let mut edges: Vec<Edge> = edge_set.into_iter().collect();
    // Sort for deterministic output: by from node name, then to node name, then edge kind.
    edges.sort_by(|a, b| {
        let a_from = nodes.get(&a.from).map(|n| n.name.as_str()).unwrap_or("");
        let b_from = nodes.get(&b.from).map(|n| n.name.as_str()).unwrap_or("");
        let a_to = nodes.get(&a.to).map(|n| n.name.as_str()).unwrap_or("");
        let b_to = nodes.get(&b.to).map(|n| n.name.as_str()).unwrap_or("");
        a_from
            .cmp(b_from)
            .then_with(|| a_to.cmp(b_to))
            .then_with(|| a.kind.label().cmp(b.kind.label()))
    });

    TypeGraph { nodes, edges }
}

/// Add edges from `source` to every ref ID that is in `local_ids` (and not self).
fn add_edges_from_refs(
    source: &Id,
    refs: &[Id],
    local_ids: &HashSet<Id>,
    kind: EdgeKind,
    edge_set: &mut HashSet<Edge>,
) {
    for ref_id in refs {
        if local_ids.contains(ref_id) && ref_id != source {
            edge_set.insert(Edge {
                from: source.clone(),
                to: ref_id.clone(),
                kind,
            });
        }
    }
}

/// Collect param/return edges for a function.
fn collect_function_edges(
    source: &Id,
    f: &rustdoc_types::Function,
    local_ids: &HashSet<Id>,
    edge_set: &mut HashSet<Edge>,
) {
    // Parameters
    let mut param_refs = Vec::new();
    for (_name, ty) in &f.sig.inputs {
        collect_type_refs(ty, &mut param_refs);
    }
    add_edges_from_refs(source, &param_refs, local_ids, EdgeKind::Param, edge_set);

    // Return type
    if let Some(ty) = &f.sig.output {
        let mut return_refs = Vec::new();
        collect_type_refs(ty, &mut return_refs);
        add_edges_from_refs(source, &return_refs, local_ids, EdgeKind::Return, edge_set);
    }

    // Generic bounds
    let mut bound_refs = Vec::new();
    collect_generics_refs(&f.generics, &mut bound_refs);
    add_edges_from_refs(source, &bound_refs, local_ids, EdgeKind::Bound, edge_set);
}

/// Collect impl-related edges (trait impls) for a type's associated impl blocks.
fn collect_impl_edges(
    krate: &Crate,
    type_id: &Id,
    impl_ids: &[Id],
    local_ids: &HashSet<Id>,
    edge_set: &mut HashSet<Edge>,
) {
    for impl_id in impl_ids {
        let Some(impl_item) = krate.index.get(impl_id) else {
            continue;
        };
        let ItemEnum::Impl(impl_) = &impl_item.inner else {
            continue;
        };
        // Skip synthetic/derived impls.
        if impl_.is_synthetic {
            continue;
        }

        // Trait impl edge: Type --> Trait
        if let Some(trait_path) = &impl_.trait_ {
            if local_ids.contains(&trait_path.id) && trait_path.id != *type_id {
                edge_set.insert(Edge {
                    from: type_id.clone(),
                    to: trait_path.id.clone(),
                    kind: EdgeKind::ImplTrait,
                });
            }
        }

        // Method param/return refs within impl blocks
        for method_id in &impl_.items {
            if let Some(method_item) = krate.index.get(method_id) {
                if let ItemEnum::Function(f) = &method_item.inner {
                    collect_function_edges(type_id, f, local_ids, edge_set);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Render the type graph as a Mermaid-in-Markdown string.
fn render_type_graph(graph: &TypeGraph) -> String {
    let mut out = String::new();
    out.push_str("# Type Dependency Graph\n\n");

    // Filter to only nodes that participate in at least one edge.
    let participating: HashSet<&Id> = graph.edges.iter().flat_map(|e| [&e.from, &e.to]).collect();

    let mut sorted_nodes: Vec<(&Id, &Node)> = graph
        .nodes
        .iter()
        .filter(|(id, _)| participating.contains(id))
        .collect();
    sorted_nodes.sort_by_key(|(_, node)| &node.name);

    if sorted_nodes.is_empty() {
        out.push_str("No type dependencies found within this crate.\n");
        return out;
    }

    out.push_str("```mermaid\ngraph LR\n");

    // Node declarations
    for (id, node) in &sorted_nodes {
        let node_id = mermaid_id(id);
        let label = format!("{} ({})", node.name, node.kind.label());
        let shape = match node.kind {
            NodeKind::Trait => format!("    {node_id}((\"{label}\"))"),
            NodeKind::Enum => format!("    {node_id}{{\"{label}\"}}"),
            _ => format!("    {node_id}[\"{label}\"]"),
        };
        out.push_str(&shape);
        out.push('\n');
    }

    out.push('\n');

    // Edge declarations
    for edge in &graph.edges {
        let from = mermaid_id(&edge.from);
        let to = mermaid_id(&edge.to);
        let label = edge.kind.label();
        out.push_str(&format!("    {from} -->|{label}| {to}\n"));
    }

    out.push_str("```\n");
    out
}

/// Convert a rustdoc `Id` to a valid Mermaid node identifier.
fn mermaid_id(id: &Id) -> String {
    format!("n{}", id.0)
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Build a type dependency graph from the rustdoc JSON crate and write it
/// as a Mermaid-in-Markdown file to `out_dir/type-graph.md`.
pub fn generate(krate: &Crate, out_dir: &Path) -> Result<()> {
    let graph = build_type_graph(krate);
    let content = render_type_graph(&graph);
    let path = out_dir.join(naming::graph_file());
    std::fs::write(&path, &content)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rustdoc_types::Path as RPath;

    #[test]
    fn test_collect_type_refs_primitive() {
        let ty = Type::Primitive("u32".to_string());
        let mut refs = Vec::new();
        collect_type_refs(&ty, &mut refs);
        assert!(refs.is_empty());
    }

    #[test]
    fn test_collect_type_refs_resolved_path() {
        let ty = Type::ResolvedPath(RPath {
            path: "MyStruct".to_string(),
            id: Id(42),
            args: None,
        });
        let mut refs = Vec::new();
        collect_type_refs(&ty, &mut refs);
        assert_eq!(refs, vec![Id(42)]);
    }

    #[test]
    fn test_collect_type_refs_borrowed_ref() {
        let ty = Type::BorrowedRef {
            lifetime: None,
            is_mutable: false,
            type_: Box::new(Type::ResolvedPath(RPath {
                path: "Foo".to_string(),
                id: Id(10),
                args: None,
            })),
        };
        let mut refs = Vec::new();
        collect_type_refs(&ty, &mut refs);
        assert_eq!(refs, vec![Id(10)]);
    }

    #[test]
    fn test_collect_type_refs_tuple() {
        let ty = Type::Tuple(vec![
            Type::ResolvedPath(RPath {
                path: "A".to_string(),
                id: Id(1),
                args: None,
            }),
            Type::Primitive("u8".to_string()),
            Type::ResolvedPath(RPath {
                path: "B".to_string(),
                id: Id(2),
                args: None,
            }),
        ]);
        let mut refs = Vec::new();
        collect_type_refs(&ty, &mut refs);
        assert_eq!(refs, vec![Id(1), Id(2)]);
    }

    #[test]
    fn test_collect_type_refs_nested_generics() {
        let ty = Type::ResolvedPath(RPath {
            path: "Vec".to_string(),
            id: Id(100),
            args: Some(Box::new(GenericArgs::AngleBracketed {
                args: vec![GenericArg::Type(Type::ResolvedPath(RPath {
                    path: "MyType".to_string(),
                    id: Id(200),
                    args: None,
                }))],
                constraints: vec![],
            })),
        });
        let mut refs = Vec::new();
        collect_type_refs(&ty, &mut refs);
        assert_eq!(refs, vec![Id(100), Id(200)]);
    }

    #[test]
    fn test_collect_type_refs_slice() {
        let ty = Type::Slice(Box::new(Type::ResolvedPath(RPath {
            path: "Foo".to_string(),
            id: Id(5),
            args: None,
        })));
        let mut refs = Vec::new();
        collect_type_refs(&ty, &mut refs);
        assert_eq!(refs, vec![Id(5)]);
    }

    #[test]
    fn test_mermaid_id() {
        assert_eq!(mermaid_id(&Id(0)), "n0");
        assert_eq!(mermaid_id(&Id(42)), "n42");
    }

    #[test]
    fn test_render_empty_graph() {
        let graph = TypeGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        };
        let output = render_type_graph(&graph);
        assert!(output.contains("No type dependencies found"));
        assert!(!output.contains("```mermaid"));
    }

    #[test]
    fn test_render_graph_with_edges() {
        let mut nodes = HashMap::new();
        nodes.insert(
            Id(1),
            Node {
                name: "Greeter".to_string(),
                kind: NodeKind::Struct,
            },
        );
        nodes.insert(
            Id(2),
            Node {
                name: "Render".to_string(),
                kind: NodeKind::Trait,
            },
        );
        let edges = vec![Edge {
            from: Id(1),
            to: Id(2),
            kind: EdgeKind::ImplTrait,
        }];
        let graph = TypeGraph { nodes, edges };
        let output = render_type_graph(&graph);
        assert!(output.contains("```mermaid"));
        assert!(output.contains("graph LR"));
        assert!(output.contains("n1[\"Greeter (struct)\"]"));
        assert!(output.contains("n2((\"Render (trait)\"))"));
        assert!(output.contains("n1 -->|impl| n2"));
    }

    #[test]
    fn test_render_graph_node_shapes() {
        let mut nodes = HashMap::new();
        nodes.insert(
            Id(1),
            Node {
                name: "MyStruct".to_string(),
                kind: NodeKind::Struct,
            },
        );
        nodes.insert(
            Id(2),
            Node {
                name: "MyEnum".to_string(),
                kind: NodeKind::Enum,
            },
        );
        nodes.insert(
            Id(3),
            Node {
                name: "MyTrait".to_string(),
                kind: NodeKind::Trait,
            },
        );
        let edges = vec![
            Edge {
                from: Id(1),
                to: Id(2),
                kind: EdgeKind::Field,
            },
            Edge {
                from: Id(1),
                to: Id(3),
                kind: EdgeKind::ImplTrait,
            },
        ];
        let graph = TypeGraph { nodes, edges };
        let output = render_type_graph(&graph);
        // Struct uses rectangle []
        assert!(output.contains("[\"MyStruct (struct)\"]"));
        // Enum uses diamond {}
        assert!(output.contains("{\"MyEnum (enum)\"}"));
        // Trait uses circle (())
        assert!(output.contains("((\"MyTrait (trait)\"))"));
    }

    #[test]
    fn test_render_graph_excludes_isolated_nodes() {
        let mut nodes = HashMap::new();
        nodes.insert(
            Id(1),
            Node {
                name: "Connected".to_string(),
                kind: NodeKind::Struct,
            },
        );
        nodes.insert(
            Id(2),
            Node {
                name: "Target".to_string(),
                kind: NodeKind::Struct,
            },
        );
        nodes.insert(
            Id(3),
            Node {
                name: "Isolated".to_string(),
                kind: NodeKind::Struct,
            },
        );
        let edges = vec![Edge {
            from: Id(1),
            to: Id(2),
            kind: EdgeKind::Field,
        }];
        let graph = TypeGraph { nodes, edges };
        let output = render_type_graph(&graph);
        assert!(output.contains("Connected"));
        assert!(output.contains("Target"));
        assert!(!output.contains("Isolated"));
    }
}
