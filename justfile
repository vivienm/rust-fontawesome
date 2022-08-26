DEFAULT: fmt check test clippy doc deny typos

fmt:
    cargo fmt --all -- --check

build:
    cargo build --all-features

check:
    cargo check --all-features

test:
    cargo test --all-features

clippy:
    cargo clippy -- -D warnings

doc:
    cargo rustdoc --all-features -- -D warnings

deny:
    cargo deny --all-features check

typos:
    typos
