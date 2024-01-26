#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

set dotenv-load := true

# Show available commands
default:
    @just --list --justfile {{justfile()}}

# Show unused dependencies
udeps:
    cargo +nightly udeps

# Run cargo doc
doc $RUSTDOCFLAGS="-D warnings":
    cargo doc --all --no-deps

# Run cargo doc and open the docs in your browser
doc-open $RUSTDOCFLAGS="-A missing_docs":
    cargo doc --all --no-deps --open

# Run cargo clippy on all crates
clippy *clippy-args:
    cargo clippy --all --tests --examples --benches -- {{ clippy-args }}
