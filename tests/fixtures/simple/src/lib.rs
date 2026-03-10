//! A simple test crate for ModQL.
//!
//! This crate is used as a fixture to test the documentation generator.

pub mod utils;

/// A greeting struct that holds a name.
pub struct Greeter {
    /// The name to greet.
    pub name: String,
}

impl Greeter {
    /// Create a new `Greeter` with the given name.
    pub fn new(name: &str) -> Self {
        Greeter {
            name: name.to_string(),
        }
    }

    /// Generate a greeting message.
    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
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

/// Run the application and return a status message.
pub fn run() -> String {
    "running".to_string()
}
