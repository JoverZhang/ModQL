# Internal Module `modql::naming`

[Surface view](module.modql.naming.md)

## Functions

```rust
/// The crate root page file name.
pub fn crate_index_file() -> &'static str;

/// The internal crate index page file name.
pub fn internal_crate_index_file() -> &'static str;

/// Generate the file name for an internal module page.
pub fn internal_module_file_name(qualified_name: &str) -> String;

/// File naming and link generation for Markdown output.
/// Generate the file name for a module page.
///
/// Examples:
/// - `module_file_name("mycrate::utils")` -> `"module.mycrate.utils.md"`
/// - `module_file_name("mycrate::foo::bar")` -> `"module.mycrate.foo.bar.md"`
pub fn module_file_name(qualified_name: &str) -> String;

/// Extract the short name (last segment) from a qualified name.
/// e.g. "mycrate::foo::Bar" -> "Bar"
pub fn short_name(qualified_name: &str) -> &str;

/// Extract the first sentence from documentation text, for use as a synopsis
/// in listing pages.
pub fn synopsis(docs: &Option<String>) -> Option<String>;

```

