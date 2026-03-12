# Internal Module `modql::cli`

[Surface view](module.modql.cli.md)

## Structs

### `Cli`

```rust
pub struct Cli {
    pub command: Command,
}
```

## Enums

### `Command`

```rust
pub enum Command {
    Generate {
        manifest_path: PathBuf,
        out: PathBuf,
        package: Option<String>,
        nightly: String,
    },
}
```

#### Variants

- `Generate`: Generate Markdown documentation from a Rust crate

## Impl Blocks

### `impl Args for Cli`

```rust
impl Args for Cli {
    fn augment_args<'b>(__clap_app: Command) -> Command;

    fn augment_args_for_update<'b>(__clap_app: Command) -> Command;

    fn group_id() -> Option<Id>;

}
```

### `impl CommandFactory for Cli`

```rust
impl CommandFactory for Cli {
    fn command<'b>() -> Command;

    fn command_for_update<'b>() -> Command;

}
```

### `impl FromArgMatches for Cli`

```rust
impl FromArgMatches for Cli {
    fn from_arg_matches(__clap_arg_matches: &ArgMatches) -> Result<Self, Error>;

    fn from_arg_matches_mut(__clap_arg_matches: &mut ArgMatches) -> Result<Self, Error>;

    fn update_from_arg_matches(&mut self, __clap_arg_matches: &ArgMatches) -> Result<(), Error>;

    fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut ArgMatches) -> Result<(), Error>;

}
```

### `impl Parser for Cli`

```rust
impl Parser for Cli;
```

### `impl FromArgMatches for Command`

```rust
impl FromArgMatches for Command {
    fn from_arg_matches(__clap_arg_matches: &ArgMatches) -> Result<Self, Error>;

    fn from_arg_matches_mut(__clap_arg_matches: &mut ArgMatches) -> Result<Self, Error>;

    fn update_from_arg_matches(&mut self, __clap_arg_matches: &ArgMatches) -> Result<(), Error>;

    fn update_from_arg_matches_mut<'b>(&mut self, __clap_arg_matches: &mut ArgMatches) -> Result<(), Error>;

}
```

### `impl Subcommand for Command`

```rust
impl Subcommand for Command {
    fn augment_subcommands<'b>(__clap_app: Command) -> Command;

    fn augment_subcommands_for_update<'b>(__clap_app: Command) -> Command;

    fn has_subcommand(__clap_name: &str) -> bool;

}
```

