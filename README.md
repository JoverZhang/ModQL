# ModQL

A Markdown documentation generator for Rust projects, powered by rustdoc JSON.

ModQL reads the JSON output from `rustdoc` and produces a paired set of Markdown files:

- a public surface view that reads like a Rust header file
- an internal view that also includes private symbols and their documentation comments

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
4. Renders surface and internal Markdown views

## Usage

### Single crate

```
modql generate --manifest-path path/to/Cargo.toml
```

By default, ModQL writes generated files to `docs/modql`.

To override the output directory:

```
modql generate --manifest-path path/to/Cargo.toml --out docs/custom
```

### Workspace (multi-crate)

When `--manifest-path` points to a workspace `Cargo.toml` (one with `[workspace]`),
ModQL automatically discovers all member packages and generates documentation for
each one into a per-package subdirectory:

```
modql generate --manifest-path path/to/workspace/Cargo.toml --out docs/modql
```

This produces:

```
docs/modql/
  core/
    index.md
    index.internal.md
    ...
  service/
    index.md
    index.internal.md
    ...
```

### Options

| Flag | Description |
|------|-------------|
| `--manifest-path <path>` | Path to the crate or workspace `Cargo.toml` (default: `Cargo.toml`) |
| `--out <dir>` | Output directory for Markdown files (default: `docs/modql`) |
| `--nightly <toolchain>` | Nightly toolchain name (default: `nightly`) |

## Example output

```
docs/modql/
  index.md                           # Public surface view
  index.internal.md                  # Internal view with private symbols
  module.mycrate.utils.md            # Public module page
  module.mycrate.utils.internal.md   # Internal module page
```

Each page follows a consistent format:

- **Surface pages**: public module/type/function declarations grouped like a header file
- **Internal pages**: the same structure, plus private symbols and their comments
- **Impl blocks**: rendered as first-class sections so inherent impls and `impl Trait for Type` are visible

## Building from source

```
cargo build --release
```

## Running tests

```
cargo test
```

Tests include integration tests that generate documentation for fixture crates and
verify the output matches golden files.
