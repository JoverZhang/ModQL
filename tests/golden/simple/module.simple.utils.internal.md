# Internal Module `simple::utils`

[Surface view](module.simple.utils.md)

Utility functions for the simple crate.

## Functions

```rust
/// A helper function that formats a value.
pub fn helper(value: &str) -> String;

/// Format a value using the crate's private utility path.
pub(in ::utils) fn internal_helper(value: &str) -> String;

```

