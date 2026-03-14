//! Advanced Rust syntax test fixture.
//!
//! This crate exercises language features not covered by the simple fixture:
//! lifetimes, where clauses, async functions, const generics, trait objects,
//! and items deliberately left without documentation.

// ---------------------------------------------------------------------------
// Lifetimes
// ---------------------------------------------------------------------------

/// A zero-copy parser that borrows its input.
pub struct Parser<'a> {
    /// The input string being parsed.
    pub input: &'a str,
    /// Current byte offset.
    cursor: usize,
}

impl<'a> Parser<'a> {
    /// Create a parser for the given input.
    pub fn new(input: &'a str) -> Self {
        Parser { input, cursor: 0 }
    }

    /// Return the remaining unparsed input.
    pub fn remaining(&self) -> &'a str {
        &self.input[self.cursor..]
    }
}

/// A borrowed key-value pair with independent lifetimes.
pub struct Pair<'k, 'v> {
    /// The key.
    pub key: &'k str,
    /// The value.
    pub value: &'v str,
}

// ---------------------------------------------------------------------------
// Where clauses
// ---------------------------------------------------------------------------

/// Serialize a value to its display representation.
pub fn serialize<T>(value: &T) -> String
where
    T: std::fmt::Display + Clone,
{
    format!("{value}")
}

/// A wrapper that requires its inner type to be sendable.
pub struct Sendable<T>
where
    T: Send + Sync,
{
    /// The wrapped value.
    pub inner: T,
}

// ---------------------------------------------------------------------------
// Async functions
// ---------------------------------------------------------------------------

/// Fetch a resource by URL (simulated).
pub async fn fetch(url: &str) -> Result<String, String> {
    Ok(format!("fetched: {url}"))
}

/// A private async helper.
async fn resolve(name: &str) -> String {
    name.to_uppercase()
}

// ---------------------------------------------------------------------------
// Const generics
// ---------------------------------------------------------------------------

/// A fixed-size buffer backed by an array.
pub struct Buffer<const N: usize> {
    /// The underlying storage.
    pub data: [u8; N],
}

impl<const N: usize> Buffer<N> {
    /// Create a zeroed buffer.
    pub fn zeroed() -> Self {
        Buffer { data: [0u8; N] }
    }
}

// ---------------------------------------------------------------------------
// impl Trait / dyn Trait
// ---------------------------------------------------------------------------

/// Create a formatter that displays a greeting.
pub fn create_greeting() -> impl std::fmt::Display {
    "hello"
}

/// Process a debug-printable value and return its representation.
pub fn debug_format(value: &dyn std::fmt::Debug) -> String {
    format!("{value:?}")
}

/// A trait for processing borrowed data.
pub trait Processor<'a> {
    /// Process a borrowed input and return a borrowed result.
    fn process(&self, input: &'a str) -> &'a str;
}

// ---------------------------------------------------------------------------
// Function pointer types
// ---------------------------------------------------------------------------

/// A callback holder with function pointer fields.
pub struct Callback {
    /// The function to invoke.
    pub handler: fn(u32) -> bool,
    /// An optional finalizer.
    pub finalizer: Option<fn()>,
}

// ---------------------------------------------------------------------------
// Generic impl blocks with bounds
// ---------------------------------------------------------------------------

/// A generic container.
pub struct Container<T> {
    /// The stored value.
    pub value: T,
}

impl<T: std::fmt::Display + Clone> Container<T> {
    /// Format the contained value.
    pub fn display(&self) -> String {
        format!("{}", self.value)
    }
}

impl<T: Default> Container<T> {
    /// Create a container with the default value.
    pub fn default_value() -> Self {
        Container {
            value: T::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// Multiple impl blocks for one type
// ---------------------------------------------------------------------------

/// A type demonstrating multiple impl blocks.
pub struct Multi;

impl Multi {
    /// First inherent method.
    pub fn alpha(&self) -> &str {
        "alpha"
    }
}

impl std::fmt::Display for Multi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Multi")
    }
}

// ---------------------------------------------------------------------------
// Items WITHOUT doc comments (deliberately undocumented)
// ---------------------------------------------------------------------------

pub struct Undocumented {
    pub field_a: i32,
    field_b: String,
}

pub fn no_docs(x: i32) -> i32 {
    x * 2
}

pub enum Bare {
    X,
    Y(u8),
    Z { flag: bool },
}

pub trait Unmarked {
    fn action(&self);
}

pub const MAGIC: u32 = 42;

pub type Pair2 = (String, String);

struct HiddenUndocumented {
    flag: bool,
}

fn hidden_no_docs() -> bool {
    true
}
