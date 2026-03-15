# Crate `core`

Core types and traits for the application.

This crate provides the foundational abstractions used across the workspace.

## Type Aliases

```rust
/// A timestamp represented as milliseconds since the Unix epoch.
pub type Timestamp = u64;

```

## Constants

```rust
/// Default page size for paginated queries.
pub const DEFAULT_PAGE_SIZE: usize = 25usize;

```

## Structs

```rust
/// A unique identifier for entities.
pub struct Id(pub u64);
```

```rust
impl StructuralPartialEq for Id;
```

```rust
/// A user entity with profile information.
pub struct User {
    pub id: Id,
    pub name: String,
    email: String,
}
```

```rust
impl StructuralPartialEq for User;
```

## Enums

```rust
/// Status of an entity in the system.
pub enum Status {
    Active,
    Suspended,
    Deleted,
}
```

```rust
impl StructuralPartialEq for Status;
```

## Traits

```rust
/// A trait for types that can describe themselves in a human-readable way.
///
/// Provides a default implementation that returns `"(no description)"`.
pub trait Describable {
    fn describe(&self) -> String;
}
```

```rust
/// A repository for loading and storing entities.
///
/// Implement this trait to provide persistence for a specific entity type.
pub trait Repository {
    fn get(&self, id: Id) -> Result<Option<<Self as >::Item>, <Self as >::Error>;
    fn save(&mut self, item: &<Self as >::Item) -> Result<Id, <Self as >::Error>;
}
```

