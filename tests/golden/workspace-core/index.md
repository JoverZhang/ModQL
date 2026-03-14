# Crate `core`

[Internal view](index.internal.md)

Core types and traits for the application.

This crate provides the foundational abstractions used across the workspace.

## Types

```rust
pub trait Describable;
pub trait Repository;
pub struct Id;
pub struct User;
pub enum Status;
```

## Impl Blocks

```rust
impl Clone for Id;
impl Copy for Id;
impl Debug for Id;
impl Eq for Id;
impl Hash for Id;
impl PartialEq for Id;
impl StructuralPartialEq for Id;
impl Clone for Status;
impl Debug for Status;
impl PartialEq for Status;
impl StructuralPartialEq for Status;
impl Clone for User;
impl Debug for User;
impl PartialEq for User;
impl StructuralPartialEq for User;
```

## Type Aliases

```rust
pub type Timestamp = u64;

```

## Constants

```rust
pub const DEFAULT_PAGE_SIZE: usize = 25usize;

```

