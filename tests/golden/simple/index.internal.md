# Internal Crate `simple`

[Surface view](index.md)

A simple test crate for ModQL.

This crate is used as a fixture to test the documentation generator.

## Modules

| Module | Summary | Surface |
|---|---|---|
| [`utils`](module.simple.utils.internal.md) | Utility functions for the simple crate. | [surface](module.simple.utils.md) |

## Structs

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
    /// Resolve the display name used in greeting output.
    pub(crate) fn display_name(&self) -> &str;

    /// Generate a greeting message.
    pub fn greet(&self) -> String;

    /// Create a new `Greeter` with the given name.
    pub fn new(name: &str) -> Self;

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
/// Run the application and return a status message.
pub fn run() -> String;

```
