FROM rust:1.80.1-bookworm

# it makes faster cargo build
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

# target runtime
RUN rustup target add x86_64-unknown-linux-gnu

# linter and formatter
RUN rustup component add clippy
RUN rustup component add rustfmt

# for VS Code Remote Container
RUN rustup component add rust-analysis
RUN rustup component add rust-src
RUN rustup component add rls

# utility for Cargo.toml
RUN cargo install cargo-edit
