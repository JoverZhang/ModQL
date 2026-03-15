# Internal Module `simple::utils`

[Surface view](module.simple.utils.md)

Utility functions for the simple crate.

## Constants

```rust
/// The utility version number.
pub const UTIL_VERSION: u32 = 1u32;

```

## Functions

```rust
/// A helper function that formats a value.
pub fn helper(value: &str) -> String;

```

---

## Constants (private)

```rust
/// Internal utility limit.
pub(in ::utils) const UTIL_LIMIT: usize = 100usize;

```

## Functions (private)

```rust
/// Format a value using the crate's private utility path.
pub(in ::utils) fn internal_helper(value: &str) -> String;

```

