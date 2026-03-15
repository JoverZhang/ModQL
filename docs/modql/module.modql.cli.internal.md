# Internal Module `modql::cli`

[Surface view](module.modql.cli.md)

## Structs

```rust
pub struct Cli {
    pub command: Command,
}
```

```rust
impl Args for Cli {
    fn augment_args<'b>(__clap_app: Command) -> Command;

    fn augment_args_for_update<'b>(__clap_app: Command) -> Command;

    fn group_id() -> Option<Id>;

}
```

```rust
impl CommandFactory for Cli {
    fn command<'b>() -> Command;

    fn command_for_update<'b>() -> Command;

}
```

```rust
impl FromArgMatches for Cli {
    fn from_arg_matches(__clap_arg_matches: &ArgMatches) -> Result<Self, Error>;

    fn from_arg_matches_mut(__clap_arg_matches: &mut ArgMatches) -> Result<Self, Error>;

    fn update_from_arg_matches(&mut self, __clap_arg_matches: &ArgMatches) -> Result<(), Error>;

    fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut ArgMatches) -> Result<(), Error>;

}
```

```rust
impl Parser for Cli;
```

## Enums

```rust
pub enum Command {
    Generate {
        manifest_path: PathBuf,
        out: PathBuf,
        nightly: String,
    },
}
```

```rust
impl FromArgMatches for Command {
    fn from_arg_matches(__clap_arg_matches: &ArgMatches) -> Result<Self, Error>;

    fn from_arg_matches_mut(__clap_arg_matches: &mut ArgMatches) -> Result<Self, Error>;

    fn update_from_arg_matches(&mut self, __clap_arg_matches: &ArgMatches) -> Result<(), Error>;

    fn update_from_arg_matches_mut<'b>(&mut self, __clap_arg_matches: &mut ArgMatches) -> Result<(), Error>;

}
```

```rust
impl Subcommand for Command {
    fn augment_subcommands<'b>(__clap_app: Command) -> Command;

    fn augment_subcommands_for_update<'b>(__clap_app: Command) -> Command;

    fn has_subcommand(__clap_name: &str) -> bool;

}
```

