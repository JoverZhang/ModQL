# Internal Crate `simple`

[Surface view](index.md)

A simple test crate for ModQL.

This crate is used as a fixture to test the documentation generator.

## Modules

| Module | Summary | Surface |
|---|---|---|
| [`utils`](module.simple.utils.internal.md) | Utility functions for the simple crate. | [surface](module.simple.utils.md) |

## Structs

### `Container`

A generic container that holds a value.

```rust
pub struct Container<T: Clone> {
    pub value: T,
}
```

#### Fields

- `value`: The contained value.

### `Greeter`

A greeting struct that holds a name.

```rust
pub struct Greeter {
    pub name: String,
    secret: String,
}
```

#### Fields

- `name`: The name to greet.
- `secret`: Internal state used only while formatting output.

### `Marker`

A unit struct with no fields.

```rust
pub struct Marker;
```

### `Wrapper`

A tuple struct wrapping a value.

```rust
pub struct Wrapper(pub String);
```

---

### `Config`

An internal-only configuration holder.

```rust
pub(crate) struct Config {
    debug: bool,
}
```

#### Fields

- `debug`: Enable debug output.

## Enums

### `Format`

Supported output formats.

```rust
pub enum Format {
    Plain,
    Rich,
}
```

#### Variants

- `Plain`: Plain text output.
- `Rich`: Rich text output with formatting.

### `Shape`

Shapes with different variant kinds.

```rust
pub enum Shape {
    Circle(f64),
    Rectangle {
        width: f64,
        height: f64,
    },
    Point,
}
```

#### Variants

- `Circle`: A circle with a radius.
- `Rectangle`: A rectangle defined by width and height.
- `Point`: A point with no data.

---

### `LogLevel`

Log level for internal diagnostics.

```rust
pub(crate) enum LogLevel {
    Info,
    Warn,
    Error,
}
```

#### Variants

- `Info`: Informational messages.
- `Warn`: Warning messages.
- `Error`: Error messages.

## Traits

### `Render`

A trait for types that can render themselves.

```rust
pub trait Render {
    fn render(&self) -> String;
}
```

#### Methods

- `render`: Render the value to a string.

---

### `Validate`

A trait used only internally.

```rust
pub(crate) trait Validate {
    fn validate(&self) -> bool;
}
```

#### Methods

- `validate`: Validate the internal state.

## Impl Blocks

### `impl Greeter`

```rust
impl Greeter {
    /// Generate a greeting message.
    pub fn greet(&self) -> String;

    /// Create a new `Greeter` with the given name.
    pub fn new(name: &str) -> Self;

    // -- private --

    /// Resolve the display name used in greeting output.
    pub(crate) fn display_name(&self) -> &str;

    /// Return the internal secret for debugging.
    pub(crate) fn secret(&self) -> &str;

}
```

### `impl Render for Greeter`

```rust
impl Render for Greeter {
    /// Render the current greeting.
    fn render(&self) -> String;

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

// -- private --

/// Resolve an internal status string for diagnostics.
pub(crate) fn internal_status() -> &'static str;

```

## Type Aliases

```rust
/// A public type alias for results.
pub type Result<T> = Result<T, String>;

// -- private --

/// An internal type alias for optional strings.
pub(crate) type OptStr = Option<String>;

```

## Constants

```rust
/// The maximum number of retries.
pub const MAX_RETRIES: u32 = 3u32;

// -- private --

/// Internal buffer size.
pub(crate) const BUFFER_SIZE: usize = 1_024usize;

```

## Statics

```rust
/// The application version string.
pub static VERSION: &str;

// -- private --

/// Internal instance counter.
pub(crate) static mut INSTANCE_COUNT: u32;

```

