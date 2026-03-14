# Internal Crate `service`

[Surface view](index.md)

Service layer for business logic.

This crate implements the application's core use cases on top of the
`core` crate abstractions.

## Modules

| Module | Summary | Surface |
|---|---|---|
| [`handler`](module.service.handler.internal.md) | Request handlers for the service layer. | [surface](module.service.handler.md) |

## Structs

### `UserStore`

An in-memory user store.

```rust
pub struct UserStore {
    users: Vec<User>,
}
```

#### Fields

- `users`: Users indexed by their identifier.

## Impl Blocks

### `impl Describable for UserStore`

```rust
impl Describable for UserStore {
    /// Describes the user store with its current size.
    fn describe(&self) -> String;

}
```

### `impl Repository for UserStore`

```rust
impl Repository for UserStore {
    /// Look up a user by id.
    fn get(&self, id: Id) -> Result<Option<User>, String>;

    /// Store a user, returning the assigned id.
    fn save(&mut self, item: &User) -> Result<Id, String>;

}
```

// Marker trait implementations

### `impl Send for UserStore`

```rust
impl Send for UserStore;
```

### `impl Sync for UserStore`

```rust
impl Sync for UserStore;
```

### `impl Unpin for UserStore`

```rust
impl Unpin for UserStore;
```

## Functions

```rust
/// Initialize the service layer with an empty user store.
pub fn init() -> UserStore;

```

---

## Structs (private)

### `ServiceConfig`

Service-level configuration.

```rust
pub(crate) struct ServiceConfig {
    max_concurrent: usize,
}
```

#### Fields

- `max_concurrent`: Maximum number of concurrent requests.

## Impl Blocks (private)

// Marker trait implementations

### `impl Send for ServiceConfig`

```rust
impl Send for ServiceConfig;
```

### `impl Sync for ServiceConfig`

```rust
impl Sync for ServiceConfig;
```

### `impl Unpin for ServiceConfig`

```rust
impl Unpin for ServiceConfig;
```

