# Internal Module `service::handler::auth`

[Surface view](module.service.handler.auth.md)

Authentication and authorization handlers.

## Structs

### `Token`

An authentication token.

```rust
pub struct Token {
    pub value: String,
    pub expires_in: u64,
}
```

#### Fields

- `value`: The raw token string.
- `expires_in`: Seconds until expiration.

## Impl Blocks

// Marker trait implementations

### `impl Send for Token`

```rust
impl Send for Token;
```

### `impl Sync for Token`

```rust
impl Sync for Token;
```

### `impl Unpin for Token`

```rust
impl Unpin for Token;
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

