//! A simple test crate for ModQL.
//!
//! This crate is used as a fixture to test the documentation generator.

pub mod utils;

/// A greeting struct that holds a name.
pub struct Greeter {
    /// The name to greet.
    pub name: String,
    /// Internal state used only while formatting output.
    secret: String,
}

impl Greeter {
    /// Create a new `Greeter` with the given name.
    pub fn new(name: &str) -> Self {
        Greeter {
            name: name.to_string(),
            secret: "classified".to_string(),
        }
    }

    /// Generate a greeting message.
    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.display_name())
    }

    /// Resolve the display name used in greeting output.
    fn display_name(&self) -> &str {
        &self.name
    }

    /// Return the internal secret for debugging.
    pub(crate) fn secret(&self) -> &str {
        &self.secret
    }
}

/// Supported output formats.
pub enum Format {
    /// Plain text output.
    Plain,
    /// Rich text output with formatting.
    Rich,
}

/// A trait for types that can render themselves.
pub trait Render {
    /// Render the value to a string.
    fn render(&self) -> String;
}

impl Render for Greeter {
    /// Render the current greeting.
    fn render(&self) -> String {
        self.greet()
    }
}

/// Run the application and return a status message.
pub fn run() -> String {
    "running".to_string()
}
