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

## Impl Blocks

### `impl Greeter`

```rust
impl Greeter {
    /// Generate a greeting message.
    pub fn greet(&self) -> String;

    /// Create a new `Greeter` with the given name.
    pub fn new(name: &str) -> Self;

}
```

// Trait implementations

### `impl Render for Greeter`

```rust
impl Render for Greeter {
    /// Render the current greeting.
    fn render(&self) -> String;

}
```

// Marker trait implementations

### `impl<T> Send for Container<T>`

```rust
impl<T> Send for Container<T>
where
    T: Send,;
```

### `impl<T> Sync for Container<T>`

```rust
impl<T> Sync for Container<T>
where
    T: Sync,;
```

### `impl<T> Unpin for Container<T>`

```rust
impl<T> Unpin for Container<T>
where
    T: Unpin,;
```

### `impl Send for Format`

```rust
impl Send for Format;
```

### `impl Sync for Format`

```rust
impl Sync for Format;
```

### `impl Unpin for Format`

```rust
impl Unpin for Format;
```

### `impl Send for Greeter`

```rust
impl Send for Greeter;
```

### `impl Sync for Greeter`

```rust
impl Sync for Greeter;
```

### `impl Unpin for Greeter`

```rust
impl Unpin for Greeter;
```

### `impl Send for Marker`

```rust
impl Send for Marker;
```

### `impl Sync for Marker`

```rust
impl Sync for Marker;
```

### `impl Unpin for Marker`

```rust
impl Unpin for Marker;
```

### `impl Send for Shape`

```rust
impl Send for Shape;
```

### `impl Sync for Shape`

```rust
impl Sync for Shape;
```

### `impl Unpin for Shape`

```rust
impl Unpin for Shape;
```

### `impl Send for Wrapper`

```rust
impl Send for Wrapper;
```

### `impl Sync for Wrapper`

```rust
impl Sync for Wrapper;
```

### `impl Unpin for Wrapper`

```rust
impl Unpin for Wrapper;
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

## Statics

```rust
/// The application version string.
pub static VERSION: &str;

```

---

## Structs (private)

### `Config`

An internal-only configuration holder.

```rust
pub(crate) struct Config {
    debug: bool,
}
```

#### Fields

- `debug`: Enable debug output.

## Enums (private)

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

## Traits (private)

### `Validate`

A trait used only internally.

```rust
pub(crate) trait Validate {
    fn validate(&self) -> bool;
}
```

#### Methods

- `validate`: Validate the internal state.

## Impl Blocks (private)

### `impl Greeter`

```rust
impl Greeter {
    /// Resolve the display name used in greeting output.
    pub(crate) fn display_name(&self) -> &str;

    /// Return the internal secret for debugging.
    pub(crate) fn secret(&self) -> &str;

}
```

// Marker trait implementations

### `impl Send for Config`

```rust
impl Send for Config;
```

### `impl Sync for Config`

```rust
impl Sync for Config;
```

### `impl Unpin for Config`

```rust
impl Unpin for Config;
```

### `impl Send for LogLevel`

```rust
impl Send for LogLevel;
```

### `impl Sync for LogLevel`

```rust
impl Sync for LogLevel;
```

### `impl Unpin for LogLevel`

```rust
impl Unpin for LogLevel;
```

## Functions (private)

```rust
/// Resolve an internal status string for diagnostics.
pub(crate) fn internal_status() -> &'static str;

```

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

## Statics (private)

```rust
/// Internal instance counter.
pub(crate) static mut INSTANCE_COUNT: u32;

```

