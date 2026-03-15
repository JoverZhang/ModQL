# Internal Module `service::handler::auth`

[Surface view](module.service.handler.auth.md)

Authentication and authorization handlers.

## Structs

```rust
/// An authentication token.
pub struct Token {
    pub value: String,
    pub expires_in: u64,
}
```

## Functions

```rust
/// Validate credentials and return an auth token.
pub fn login(username: &str, _password: &str) -> Token;

/// Verify that a token is still valid.
pub fn verify(token: &Token) -> bool;

```

---

## Functions (private)

```rust
/// Revoke a token, making it unusable.
pub(in ::handler::auth) fn revoke(_token: &Token);

```

