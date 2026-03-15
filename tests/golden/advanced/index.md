# Crate `advanced`

Advanced Rust syntax test fixture.

This crate exercises language features not covered by the simple fixture:
lifetimes, where clauses, async functions, const generics, trait objects,
and items deliberately left without documentation.

## Type Aliases

```rust
pub type Pair2 = (String, String);

```

## Constants

```rust
pub const MAGIC: u32 = 42u32;

```

## Structs

```rust
/// A fixed-size buffer backed by an array.
pub struct Buffer<const N: usize> {
    pub data: [u8; N],
}
```

```rust
/// A callback holder with function pointer fields.
pub struct Callback {
    pub handler: fn(_: u32) -> bool,
    pub finalizer: Option<fn()>,
}
```

```rust
/// A generic container.
pub struct Container<T> {
    pub value: T,
}
```

```rust
/// A type demonstrating multiple impl blocks.
pub struct Multi;
```

```rust
impl Display for Multi {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

```rust
/// A borrowed key-value pair with independent lifetimes.
pub struct Pair<'k, 'v> {
    pub key: &'k str,
    pub value: &'v str,
}
```

```rust
/// A zero-copy parser that borrows its input.
pub struct Parser<'a> {
    pub input: &'a str,
    cursor: usize,
}
```

```rust
/// A wrapper that requires its inner type to be sendable.
pub struct Sendable<T>
where
    T: Send + Sync, {
    pub inner: T,
}
```

```rust
pub struct Undocumented {
    pub field_a: i32,
    field_b: String,
}
```

## Enums

```rust
pub enum Bare {
    X,
    Y(u8),
    Z {
        flag: bool,
    },
}
```

## Traits

```rust
/// A trait for processing borrowed data.
pub trait Processor<'a> {
    fn process(&self, input: &'a str) -> &'a str;
}
```

```rust
pub trait Unmarked {
    fn action(&self);
}
```

## Impl

```rust
impl<const N: usize> Buffer<_> {
    /// Create a zeroed buffer.
    pub fn zeroed() -> Self;

}
```

```rust
impl<T: Default> Container<T> {
    /// Create a container with the default value.
    pub fn default_value() -> Self;

}
```

```rust
impl<T: Display + Clone> Container<T> {
    /// Format the contained value.
    pub fn display(&self) -> String;

}
```

```rust
impl Multi {
    /// First inherent method.
    pub fn alpha(&self) -> &str;

}
```

```rust
impl<'a> Parser<'a> {
    /// Create a parser for the given input.
    pub fn new(input: &'a str) -> Self;

    /// Return the remaining unparsed input.
    pub fn remaining(&self) -> &'a str;

}
```

## Functions

```rust
/// Create a formatter that displays a greeting.
pub fn create_greeting() -> impl Display;

/// Process a debug-printable value and return its representation.
pub fn debug_format(value: &dyn Debug) -> String;

/// Fetch a resource by URL (simulated).
pub async fn fetch(url: &str) -> Result<String, String>;

pub fn no_docs(x: i32) -> i32;

/// Serialize a value to its display representation.
pub fn serialize<T>(value: &T) -> String
where
    T: Display + Clone,;

```

---

## Structs (private)

```rust
pub(crate) struct HiddenUndocumented {
    flag: bool,
}
```

## Functions (private)

```rust
pub(crate) fn hidden_no_docs() -> bool;

/// A private async helper.
pub(crate) async fn resolve(name: &str) -> String;

```

