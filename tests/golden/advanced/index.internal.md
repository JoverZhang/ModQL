# Internal Crate `advanced`

[Surface view](index.md)

Advanced Rust syntax test fixture.

This crate exercises language features not covered by the simple fixture:
lifetimes, where clauses, async functions, const generics, trait objects,
and items deliberately left without documentation.

## Structs

### `Buffer`

A fixed-size buffer backed by an array.

```rust
pub struct Buffer<const N: usize> {
    pub data: [u8; N],
}
```

#### Fields

- `data`: The underlying storage.

### `Callback`

A callback holder with function pointer fields.

```rust
pub struct Callback {
    pub handler: fn(_: u32) -> bool,
    pub finalizer: Option<fn()>,
}
```

#### Fields

- `handler`: The function to invoke.
- `finalizer`: An optional finalizer.

### `Container`

A generic container.

```rust
pub struct Container<T> {
    pub value: T,
}
```

#### Fields

- `value`: The stored value.

### `Multi`

A type demonstrating multiple impl blocks.

```rust
pub struct Multi;
```

### `Pair`

A borrowed key-value pair with independent lifetimes.

```rust
pub struct Pair<'k, 'v> {
    pub key: &'k str,
    pub value: &'v str,
}
```

#### Fields

- `key`: The key.
- `value`: The value.

### `Parser`

A zero-copy parser that borrows its input.

```rust
pub struct Parser<'a> {
    pub input: &'a str,
    cursor: usize,
}
```

#### Fields

- `input`: The input string being parsed.
- `cursor`: Current byte offset.

### `Sendable`

A wrapper that requires its inner type to be sendable.

```rust
pub struct Sendable<T>
where
    T: Send + Sync, {
    pub inner: T,
}
```

#### Fields

- `inner`: The wrapped value.

### `Undocumented`

```rust
pub struct Undocumented {
    pub field_a: i32,
    field_b: String,
}
```

## Enums

### `Bare`

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

### `Processor`

A trait for processing borrowed data.

```rust
pub trait Processor<'a> {
    fn process(&self, input: &'a str) -> &'a str;
}
```

#### Methods

- `process`: Process a borrowed input and return a borrowed result.

### `Unmarked`

```rust
pub trait Unmarked {
    fn action(&self);
}
```

## Impl Blocks

### `impl<const N: usize> Buffer<_>`

```rust
impl<const N: usize> Buffer<_> {
    /// Create a zeroed buffer.
    pub fn zeroed() -> Self;

}
```

### `impl<T: Default> Container<T>`

```rust
impl<T: Default> Container<T> {
    /// Create a container with the default value.
    pub fn default_value() -> Self;

}
```

### `impl<T: Display + Clone> Container<T>`

```rust
impl<T: Display + Clone> Container<T> {
    /// Format the contained value.
    pub fn display(&self) -> String;

}
```

### `impl Multi`

```rust
impl Multi {
    /// First inherent method.
    pub fn alpha(&self) -> &str;

}
```

### `impl<'a> Parser<'a>`

```rust
impl<'a> Parser<'a> {
    /// Create a parser for the given input.
    pub fn new(input: &'a str) -> Self;

    /// Return the remaining unparsed input.
    pub fn remaining(&self) -> &'a str;

}
```

// Trait implementations

### `impl Display for Multi`

```rust
impl Display for Multi {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

// Marker trait implementations

### `impl Send for Bare`

```rust
impl Send for Bare;
```

### `impl Sync for Bare`

```rust
impl Sync for Bare;
```

### `impl Unpin for Bare`

```rust
impl Unpin for Bare;
```

### `impl<const N: usize> Send for Buffer<_>`

```rust
impl<const N: usize> Send for Buffer<_>;
```

### `impl<const N: usize> Sync for Buffer<_>`

```rust
impl<const N: usize> Sync for Buffer<_>;
```

### `impl<const N: usize> Unpin for Buffer<_>`

```rust
impl<const N: usize> Unpin for Buffer<_>;
```

### `impl Send for Callback`

```rust
impl Send for Callback;
```

### `impl Sync for Callback`

```rust
impl Sync for Callback;
```

### `impl Unpin for Callback`

```rust
impl Unpin for Callback;
```

### `impl<T> Send for Container<T>`

```rust
impl<T> Send for Container<T>
where
    T: Send,;
```

### `impl<T> Sync for Container<T>`

```rust
impl<T> Sync for Container<T>
where
    T: Sync,;
```

### `impl<T> Unpin for Container<T>`

```rust
impl<T> Unpin for Container<T>
where
    T: Unpin,;
```

### `impl Send for Multi`

```rust
impl Send for Multi;
```

### `impl Sync for Multi`

```rust
impl Sync for Multi;
```

### `impl Unpin for Multi`

```rust
impl Unpin for Multi;
```

### `impl<'k, 'v> Send for Pair<'k, 'v>`

```rust
impl<'k, 'v> Send for Pair<'k, 'v>;
```

### `impl<'k, 'v> Sync for Pair<'k, 'v>`

```rust
impl<'k, 'v> Sync for Pair<'k, 'v>;
```

### `impl<'k, 'v> Unpin for Pair<'k, 'v>`

```rust
impl<'k, 'v> Unpin for Pair<'k, 'v>;
```

### `impl<'a> Send for Parser<'a>`

```rust
impl<'a> Send for Parser<'a>;
```

### `impl<'a> Sync for Parser<'a>`

```rust
impl<'a> Sync for Parser<'a>;
```

### `impl<'a> Unpin for Parser<'a>`

```rust
impl<'a> Unpin for Parser<'a>;
```

### `impl<T> Send for Sendable<T>`

```rust
impl<T> Send for Sendable<T>;
```

### `impl<T> Sync for Sendable<T>`

```rust
impl<T> Sync for Sendable<T>;
```

### `impl<T> Unpin for Sendable<T>`

```rust
impl<T> Unpin for Sendable<T>
where
    T: Unpin,;
```

### `impl Send for Undocumented`

```rust
impl Send for Undocumented;
```

### `impl Sync for Undocumented`

```rust
impl Sync for Undocumented;
```

### `impl Unpin for Undocumented`

```rust
impl Unpin for Undocumented;
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

## Type Aliases

```rust
pub type Pair2 = (String, String);

```

## Constants

```rust
pub const MAGIC: u32 = 42u32;

```

---

## Structs (private)

### `HiddenUndocumented`

```rust
pub(crate) struct HiddenUndocumented {
    flag: bool,
}
```

## Impl Blocks (private)

// Marker trait implementations

### `impl Send for HiddenUndocumented`

```rust
impl Send for HiddenUndocumented;
```

### `impl Sync for HiddenUndocumented`

```rust
impl Sync for HiddenUndocumented;
```

### `impl Unpin for HiddenUndocumented`

```rust
impl Unpin for HiddenUndocumented;
```

## Functions (private)

```rust
pub(crate) fn hidden_no_docs() -> bool;

/// A private async helper.
pub(crate) async fn resolve(name: &str) -> String;

```

