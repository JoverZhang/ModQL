# Crate `service`

Service layer for business logic.

This crate implements the application's core use cases on top of the
`core` crate abstractions.

## Modules

| Module | Summary | Surface |
|---|---|---|
| [`handler`](module.service.handler.internal.md) | Request handlers for the service layer. | [surface](module.service.handler.md) |

## Structs

```rust
/// An in-memory user store.
pub struct UserStore {
    users: Vec<User>,
}
```

```rust
impl Describable for UserStore {
    /// Describes the user store with its current size.
    fn describe(&self) -> String;

}
```

```rust
impl Repository for UserStore {
    /// Look up a user by id.
    fn get(&self, id: Id) -> Result<Option<User>, String>;

    /// Store a user, returning the assigned id.
    fn save(&mut self, item: &User) -> Result<Id, String>;

}
```

## Functions

```rust
/// Initialize the service layer with an empty user store.
pub fn init() -> UserStore;

```

---

## Structs (private)

```rust
/// Service-level configuration.
pub(crate) struct ServiceConfig {
    max_concurrent: usize,
}
```

