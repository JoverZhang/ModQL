# Module `modql::naming`

[Internal view](module.modql.naming.internal.md)

## Functions

```rust
pub fn crate_index_file() -> &'static str;

pub fn internal_crate_index_file() -> &'static str;

pub fn internal_module_file_name(qualified_name: &str) -> String;

pub fn module_file_name(qualified_name: &str) -> String;

pub fn short_name(qualified_name: &str) -> &str;

pub fn synopsis(docs: &Option<String>) -> Option<String>;

```

