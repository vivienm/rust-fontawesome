DEFAULT: ci
cargo := "cargo"

ci: check test fmt clippy doc deny

build:
    {{cargo}} build

check:
    {{cargo}} check

test:
    {{cargo}} +nightly test

fmt:
    {{cargo}} fmt --all -- --check

clippy:
    {{cargo}} clippy -- -D warnings

doc:
    {{cargo}} +nightly rustdoc -- -D warnings

deny:
    {{cargo}} deny check
