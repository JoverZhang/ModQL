# Crate `advanced`

[Internal view](index.internal.md)

Advanced Rust syntax test fixture.

This crate exercises language features not covered by the simple fixture:
lifetimes, where clauses, async functions, const generics, trait objects,
and items deliberately left without documentation.

## Types

```rust
pub trait Processor<'a>;
pub trait Unmarked;
pub struct Buffer<const N: usize>;
pub struct Callback;
pub struct Container<T>;
pub struct Multi;
pub struct Pair<'k, 'v>;
pub struct Parser<'a>;
pub struct Sendable<T>
where
    T: Send + Sync,;
pub struct Undocumented;
pub(crate) struct HiddenUndocumented;
pub enum Bare;
```

## Functions

```rust
pub fn create_greeting() -> impl Display;

pub fn debug_format(value: &dyn Debug) -> String;

pub async fn fetch(url: &str) -> Result<String, String>;

pub fn no_docs(x: i32) -> i32;

pub fn serialize<T>(value: &T) -> String
where
    T: Display + Clone,;

pub(crate) fn hidden_no_docs() -> bool;

pub(crate) async fn resolve(name: &str) -> String;

```

## Impl Blocks

```rust
impl<const N: usize> Buffer<_>;
impl<T: Default> Container<T>;
impl<T: Display + Clone> Container<T>;
impl Display for Multi;
impl Multi;
impl<'a> Parser<'a>;
```

## Type Aliases

```rust
pub type Pair2 = (String, String);

```

## Constants

```rust
pub const MAGIC: u32 = 42u32;

```

