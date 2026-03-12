set shell := ["bash", "-cu"]

default:
    just --list

build:
    cargo build

test:
    cargo test

docs:
    mkdir -p docs/modql
    cargo run -- generate --manifest-path Cargo.toml --out docs/modql

all:
    just build
    just test
    just docs
