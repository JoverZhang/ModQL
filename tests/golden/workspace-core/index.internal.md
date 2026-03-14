# Internal Crate `core`

[Surface view](index.md)

Core types and traits for the application.

This crate provides the foundational abstractions used across the workspace.

## Structs

### `Id`

A unique identifier for entities.

```rust
pub struct Id(pub u64);
```

### `User`

A user entity with profile information.

```rust
pub struct User {
    pub id: Id,
    pub name: String,
    email: String,
}
```

#### Fields

- `id`: The user's unique identifier.
- `name`: Display name shown in the UI.
- `email`: Email address for notifications.

## Enums

### `Status`

Status of an entity in the system.

```rust
pub enum Status {
    Active,
    Suspended,
    Deleted,
}
```

#### Variants

- `Active`: Entity is active and can be used.
- `Suspended`: Entity is temporarily suspended.
- `Deleted`: Entity has been permanently removed.

## Traits

### `Describable`

A trait for types that can describe themselves in a human-readable way.

Provides a default implementation that returns `"(no description)"`.

```rust
pub trait Describable {
    fn describe(&self) -> String;
}
```

#### Methods

- `describe`: Return a human-readable description.

### `Repository`

A repository for loading and storing entities.

Implement this trait to provide persistence for a specific entity type.

```rust
pub trait Repository {
    fn get(&self, id: Id) -> Result<Option<<Self as >::Item>, <Self as >::Error>;
    fn save(&mut self, item: &<Self as >::Item) -> Result<Id, <Self as >::Error>;
}
```

#### Methods

- `get`: Retrieve an entity by its identifier.
- `save`: Persist an entity, returning its identifier.

## Impl Blocks

### `impl StructuralPartialEq for Id`

```rust
impl StructuralPartialEq for Id;
```

### `impl StructuralPartialEq for Status`

```rust
impl StructuralPartialEq for Status;
```

### `impl StructuralPartialEq for User`

```rust
impl StructuralPartialEq for User;
```

// Derived trait implementations

### `impl Clone for Id`

```rust
impl Clone for Id {
    fn clone(&self) -> Id;

}
```

### `impl Copy for Id`

```rust
impl Copy for Id;
```

### `impl Debug for Id`

```rust
impl Debug for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl Eq for Id`

```rust
impl Eq for Id;
```

### `impl Hash for Id`

```rust
impl Hash for Id {
    fn hash<__H: Hasher>(&self, state: &mut __H);

}
```

### `impl PartialEq for Id`

```rust
impl PartialEq for Id {
    fn eq(&self, other: &Id) -> bool;

}
```

### `impl Clone for Status`

```rust
impl Clone for Status {
    fn clone(&self) -> Status;

}
```

### `impl Debug for Status`

```rust
impl Debug for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl PartialEq for Status`

```rust
impl PartialEq for Status {
    fn eq(&self, other: &Status) -> bool;

}
```

### `impl Clone for User`

```rust
impl Clone for User {
    fn clone(&self) -> User;

}
```

### `impl Debug for User`

```rust
impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;

}
```

### `impl PartialEq for User`

```rust
impl PartialEq for User {
    fn eq(&self, other: &User) -> bool;

}
```

// Marker trait implementations

### `impl Send for Id`

```rust
impl Send for Id;
```

### `impl Sync for Id`

```rust
impl Sync for Id;
```

### `impl Unpin for Id`

```rust
impl Unpin for Id;
```

### `impl Send for Status`

```rust
impl Send for Status;
```

### `impl Sync for Status`

```rust
impl Sync for Status;
```

### `impl Unpin for Status`

```rust
impl Unpin for Status;
```

### `impl Send for User`

```rust
impl Send for User;
```

### `impl Sync for User`

```rust
impl Sync for User;
```

### `impl Unpin for User`

```rust
impl Unpin for User;
```

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

