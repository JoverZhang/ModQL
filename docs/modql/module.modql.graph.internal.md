# Internal Module `modql::graph`

[Surface view](module.modql.graph.md)

## Functions

```rust
/// Build a type dependency graph from the rustdoc JSON crate and write it
/// as a Mermaid-in-Markdown file to `out_dir/type-graph.md`.
pub fn generate(krate: &Crate, out_dir: &Path) -> Result<()>;

```

---

## Structs (private)

```rust
/// A directed edge from one node to another.
pub(in ::graph) struct Edge {
    from: Id,
    to: Id,
    kind: EdgeKind,
}
```

```rust
impl StructuralPartialEq for Edge;
```

```rust
/// A node representing a crate-local type or function.
pub(in ::graph) struct Node {
    name: String,
    kind: NodeKind,
}
```

```rust
/// The complete type dependency graph for one crate.
pub(in ::graph) struct TypeGraph {
    nodes: HashMap<Id, Node>,
    edges: Vec<Edge>,
}
```

## Enums (private)

```rust
/// The kind of relationship between two nodes.
pub(in ::graph) enum EdgeKind {
    Field,
    Param,
    Return,
    Bound,
    ImplTrait,
    Alias,
}
```

```rust
impl StructuralPartialEq for EdgeKind;
```

```rust
/// The kind of a node in the type graph.
pub(in ::graph) enum NodeKind {
    Struct,
    Enum,
    Trait,
    TypeAlias,
    Function,
}
```

```rust
impl StructuralPartialEq for NodeKind;
```

## Impl (private)

```rust
impl EdgeKind {
    pub(in ::graph) fn label(self) -> &'static str;

}
```

```rust
impl NodeKind {
    pub(in ::graph) fn label(self) -> &'static str;

}
```

## Functions (private)

```rust
/// Add edges from `source` to every ref ID that is in `local_ids` (and not self).
pub(in ::graph) fn add_edges_from_refs(source: &Id, refs: &[Id], local_ids: &HashSet<Id>, kind: EdgeKind, edge_set: &mut HashSet<Edge>);

/// Build a type dependency graph from a rustdoc JSON crate.
///
/// Only crate-local items appear as nodes. Edges point from an item to the
/// crate-local types it references through fields, parameters, return types,
/// generic bounds, trait impls, and type alias targets.
pub(in ::graph) fn build_type_graph(krate: &Crate) -> TypeGraph;

/// Collect type refs from generic bounds.
pub(in ::graph) fn collect_bound_refs(bounds: &[GenericBound], out: &mut Vec<Id>);

/// Collect param/return edges for a function.
pub(in ::graph) fn collect_function_edges(source: &Id, f: &Function, local_ids: &HashSet<Id>, edge_set: &mut HashSet<Edge>);

/// Collect type refs from generic arguments.
pub(in ::graph) fn collect_generic_args_refs(args: &GenericArgs, out: &mut Vec<Id>);

/// Collect type refs from generics (params + where clauses).
pub(in ::graph) fn collect_generics_refs(generics: &Generics, out: &mut Vec<Id>);

/// Collect impl-related edges (trait impls) for a type's associated impl blocks.
pub(in ::graph) fn collect_impl_edges(krate: &Crate, type_id: &Id, impl_ids: &[Id], local_ids: &HashSet<Id>, edge_set: &mut HashSet<Edge>);

/// Recursively collect all `ResolvedPath` IDs from a `Type`.
pub(in ::graph) fn collect_type_refs(ty: &Type, out: &mut Vec<Id>);

/// Convert a rustdoc `Id` to a valid Mermaid node identifier.
pub(in ::graph) fn mermaid_id(id: &Id) -> String;

/// Render the type graph as a Mermaid-in-Markdown string.
pub(in ::graph) fn render_type_graph(graph: &TypeGraph) -> String;

```

