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
pub struct Greeter;
pub enum Format;
```

## Functions

```rust
pub(crate) fn internal_status() -> &'static str;

pub fn run() -> String;

```

## Impl Blocks

```rust
impl Greeter;
impl Render for Greeter;
```

