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

/// A unit struct with no fields.
pub struct Marker;

/// A tuple struct wrapping a value.
pub struct Wrapper(pub String);

/// An internal-only configuration holder.
struct Config {
    /// Enable debug output.
    debug: bool,
}

/// A generic container that holds a value.
pub struct Container<T: Clone> {
    /// The contained value.
    pub value: T,
}

/// Supported output formats.
pub enum Format {
    /// Plain text output.
    Plain,
    /// Rich text output with formatting.
    Rich,
}

/// Log level for internal diagnostics.
enum LogLevel {
    /// Informational messages.
    Info,
    /// Warning messages.
    Warn,
    /// Error messages.
    Error,
}

/// Shapes with different variant kinds.
pub enum Shape {
    /// A circle with a radius.
    Circle(f64),
    /// A rectangle defined by width and height.
    Rectangle { width: f64, height: f64 },
    /// A point with no data.
    Point,
}

/// A trait for types that can render themselves.
pub trait Render {
    /// Render the value to a string.
    fn render(&self) -> String;
}

/// A trait used only internally.
trait Validate {
    /// Validate the internal state.
    fn validate(&self) -> bool;
}

impl Render for Greeter {
    /// Render the current greeting.
    fn render(&self) -> String {
        self.greet()
    }
}

/// The maximum number of retries.
pub const MAX_RETRIES: u32 = 3;

/// Internal buffer size.
const BUFFER_SIZE: usize = 1024;

/// The application version string.
pub static VERSION: &str = "0.1.0";

/// Internal instance counter.
static mut INSTANCE_COUNT: u32 = 0;

/// A public type alias for results.
pub type Result<T> = std::result::Result<T, String>;

/// An internal type alias for optional strings.
type OptStr = Option<String>;

/// Resolve an internal status string for diagnostics.
fn internal_status() -> &'static str {
    "internal"
}

/// Run the application and return a status message.
pub fn run() -> String {
    "running".to_string()
}

/// Perform an unsafe low-level operation.
pub unsafe fn unsafe_op(ptr: *const u8) -> u8 {
    *ptr
}

/// Compute a value at compile time.
pub const fn const_add(a: u32, b: u32) -> u32 {
    a + b
}
