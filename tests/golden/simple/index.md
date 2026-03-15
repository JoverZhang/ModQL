# Crate `simple`

A simple test crate for ModQL.

This crate is used as a fixture to test the documentation generator.

## Modules

| Module | Summary | Surface |
|---|---|---|
| [`utils`](module.simple.utils.internal.md) | Utility functions for the simple crate. | [surface](module.simple.utils.md) |

## Type Aliases

```rust
/// A public type alias for results.
pub type Result<T> = Result<T, String>;

```

## Constants

```rust
/// The maximum number of retries.
pub const MAX_RETRIES: u32 = 3u32;

```

## Structs

```rust
/// A generic container that holds a value.
pub struct Container<T: Clone> {
    pub value: T,
}
```

```rust
/// A greeting struct that holds a name.
pub struct Greeter {
    pub name: String,
    secret: String,
}
```

```rust
impl Render for Greeter {
    /// Render the current greeting.
    fn render(&self) -> String;

}
```

```rust
/// A unit struct with no fields.
pub struct Marker;
```

```rust
/// A tuple struct wrapping a value.
pub struct Wrapper(pub String);
```

## Enums

```rust
/// Supported output formats.
pub enum Format {
    Plain,
    Rich,
}
```

```rust
/// Shapes with different variant kinds.
pub enum Shape {
    Circle(f64),
    Rectangle {
        width: f64,
        height: f64,
    },
    Point,
}
```

## Traits

```rust
/// A trait for types that can render themselves.
pub trait Render {
    fn render(&self) -> String;
}
```

## Impl

```rust
impl Greeter {
    /// Generate a greeting message.
    pub fn greet(&self) -> String;

    /// Create a new `Greeter` with the given name.
    pub fn new(name: &str) -> Self;

}
```

## Functions

```rust
/// Compute a value at compile time.
pub const fn const_add(a: u32, b: u32) -> u32;

/// Run the application and return a status message.
pub fn run() -> String;

/// Perform an unsafe low-level operation.
pub unsafe fn unsafe_op(ptr: *const u8) -> u8;

```

## Statics

```rust
/// The application version string.
pub static VERSION: &str;

```

---

## Type Aliases (private)

```rust
/// An internal type alias for optional strings.
pub(crate) type OptStr = Option<String>;

```

## Constants (private)

```rust
/// Internal buffer size.
pub(crate) const BUFFER_SIZE: usize = 1_024usize;

```

## Structs (private)

```rust
/// An internal-only configuration holder.
pub(crate) struct Config {
    debug: bool,
}
```

## Enums (private)

```rust
/// Log level for internal diagnostics.
pub(crate) enum LogLevel {
    Info,
    Warn,
    Error,
}
```

## Traits (private)

```rust
/// A trait used only internally.
pub(crate) trait Validate {
    fn validate(&self) -> bool;
}
```

## Impl (private)

```rust
impl Greeter {
    /// Resolve the display name used in greeting output.
    pub(crate) fn display_name(&self) -> &str;

    /// Return the internal secret for debugging.
    pub(crate) fn secret(&self) -> &str;

}
```

## Functions (private)

```rust
/// Resolve an internal status string for diagnostics.
pub(crate) fn internal_status() -> &'static str;

```

## Statics (private)

```rust
/// Internal instance counter.
pub(crate) static mut INSTANCE_COUNT: u32;

```

