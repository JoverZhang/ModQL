# Crate `service`

[Internal view](index.internal.md)

Service layer for business logic.

This crate implements the application's core use cases on top of the
`core` crate abstractions.

## Modules

| Module | Summary | Internal |
|---|---|---|
| [`handler`](module.service.handler.md) | Request handlers for the service layer. | [internal](module.service.handler.internal.md) |

## Types

```rust
pub struct UserStore;
pub(crate) struct ServiceConfig;
```

## Functions

```rust
pub fn init() -> UserStore;

```

## Impl Blocks

```rust
impl Describable for UserStore;
impl Repository for UserStore;
```

