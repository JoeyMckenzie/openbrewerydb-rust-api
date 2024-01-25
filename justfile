alias b := build

default: lint

# build for release
build:
    cargo build --release

# deploy to crates.io
deploy: build
    cargo publish

# lint files with clippy
lint:
    cargo clippy -v

# continuously lint, watching for file changes
lint-watch:
    cargo watch -x clippy

# run tests, watch for file changes
test:
    cargo test

# continuously run tests, watch for file changes
test-watch:
    cargo watch -x test

# check rust files format, emitting errors on formatting issues
check:
    cargo fmt -v --check

# format rust files
fmt:
    cargo fmt -v

ci: check lint build test
