# Module `service::handler::auth`

[Internal view](module.service.handler.auth.internal.md)

Authentication and authorization handlers.

## Types

```rust
pub struct Token;
```

## Functions

```rust
pub fn login(username: &str, _password: &str) -> Token;

pub fn verify(token: &Token) -> bool;

```

