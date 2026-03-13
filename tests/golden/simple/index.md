# Crate `simple`

[Internal view](index.internal.md)

A simple test crate for ModQL.

This crate is used as a fixture to test the documentation generator.

## Modules

| Module | Summary | Internal |
|---|---|---|
| [`utils`](module.simple.utils.md) | Utility functions for the simple crate. | [internal](module.simple.utils.internal.md) |

## Types

```rust
pub trait Render;
pub(crate) trait Validate;
pub struct Container<T: Clone>;
pub struct Greeter;
pub struct Marker;
pub struct Wrapper;
pub(crate) struct Config;
pub enum Format;
pub enum Shape;
pub(crate) enum LogLevel;
```

## Functions

```rust
pub const fn const_add(a: u32, b: u32) -> u32;

pub fn run() -> String;

pub unsafe fn unsafe_op(ptr: *const u8) -> u8;

pub(crate) fn internal_status() -> &'static str;

```

## Impl Blocks

```rust
impl Greeter;
impl Render for Greeter;
```

## Type Aliases

```rust
pub type Result<T> = Result<T, String>;

pub(crate) type OptStr = Option<String>;

```

## Constants

```rust
pub const MAX_RETRIES: u32 = 3u32;

pub(crate) const BUFFER_SIZE: usize = 1_024usize;

```

## Statics

```rust
pub static VERSION: &str;

pub(crate) static mut INSTANCE_COUNT: u32;

```

