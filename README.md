# ModQL

A docs.rs-like Markdown documentation generator for Rust projects, powered by rustdoc JSON.

ModQL reads the JSON output from `rustdoc` and produces a tree of Markdown files that mirror the structure you see on docs.rs: one page per crate, module, struct, enum, trait, and function.

## Requirements

- **Nightly Rust** is required to generate rustdoc JSON output. Install it with:
  ```
  rustup toolchain install nightly
  ```
- Stable Rust to build `modql` itself.

## How it works

```
cargo +nightly rustdoc  ──>  rustdoc JSON  ──>  modql  ──>  Markdown files
       (step 1)                (step 2)        (step 3)       (output)
```

1. `modql` invokes `cargo +nightly rustdoc -Z unstable-options --output-format json` on your crate
2. Reads the generated rustdoc JSON file
3. Converts it into an internal documentation model
4. Renders docs.rs-like Markdown files

## Usage

```
modql generate --manifest-path path/to/Cargo.toml --out docs/
```

### Options

| Flag | Description |
|------|-------------|
| `--manifest-path <path>` | Path to the crate's `Cargo.toml` (required) |
| `--out <dir>` | Output directory for Markdown files (required) |
| `--package <name>` | Package name, for workspaces |
| `--document-private-items` | Include private items |
| `--nightly <toolchain>` | Nightly toolchain name (default: `nightly`) |

## Example output

```
docs/
  index.md                         # Crate root page
  module.mycrate.utils.md           # Module page
  struct.mycrate.Greeter.md         # Struct page (with methods)
  enum.mycrate.Format.md            # Enum page
  trait.mycrate.Render.md           # Trait page (with methods)
  function.mycrate.run.md           # Function page
```

Each page follows a consistent format:

- **Crate page**: `# Crate \`name\`` with grouped listings of modules, structs, enums, traits, functions
- **Module page**: `# Module \`crate::path\`` with its own item listings
- **Type pages**: signature in a fenced `rust` code block, documentation, and methods
- **Function page**: signature and documentation

## Building from source

```
cargo build --release
```

## Running tests

```
cargo test
```

Tests include unit tests for rendering and naming logic, plus an integration test that generates documentation for a fixture crate and verifies the output structure and content.
