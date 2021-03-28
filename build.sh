#!/bin/sh

cargo build --release --manifest-path=rust-book/Cargo.toml
cargo build --release --manifest-path=rust-std/Cargo.toml
cargo build --release --manifest-path=rust-bio/Cargo.toml
cargo build --release --manifest-path=plotters/Cargo.toml
cargo build --release --manifest-path=gtk/Cargo.toml
