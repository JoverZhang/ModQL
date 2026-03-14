//! Request handlers for the service layer.

pub mod auth;

/// Dispatch a request by name and return the response body.
pub fn dispatch(name: &str) -> String {
    format!("dispatching: {name}")
}

/// Log a handler event for auditing.
fn log_event(event: &str) {
    let _ = event;
}
