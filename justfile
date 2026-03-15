set shell := ["bash", "-cu"]

default:
    just --list

build:
    cargo build

test:
    cargo test

modql:
    rm -rf docs/modql
    mkdir -p docs/modql
    cargo run -- generate --manifest-path Cargo.toml --out docs/modql

all:
    just build
    just test
    just modql
