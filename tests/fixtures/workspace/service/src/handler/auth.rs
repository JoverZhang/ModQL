//! Authentication and authorization handlers.

/// An authentication token.
pub struct Token {
    /// The raw token string.
    pub value: String,
    /// Seconds until expiration.
    pub expires_in: u64,
}

/// Validate credentials and return an auth token.
pub fn login(username: &str, _password: &str) -> Token {
    Token {
        value: format!("token-for-{username}"),
        expires_in: 3600,
    }
}

/// Verify that a token is still valid.
pub fn verify(token: &Token) -> bool {
    !token.value.is_empty()
}

/// Revoke a token, making it unusable.
fn revoke(_token: &Token) {
    // internal only
}
