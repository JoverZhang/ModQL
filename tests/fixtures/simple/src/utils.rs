//! Utility functions for the simple crate.

/// A helper function that formats a value.
pub fn helper(value: &str) -> String {
    format!("helper: {value}")
}

/// Format a value using the crate's private utility path.
fn internal_helper(value: &str) -> String {
    format!("internal: {value}")
}
