alias b := build

default: lint

# build for release
build:
    cargo build --release

# deploy to crates.io
deploy: build
    cargo publish

# continuously lint, watching for file changes
lint:
    cargo watch -x clippy

# continuously run tests, watch for file changes
test:
    cargo watch -x test

# check rust files format, emitting errors on formatting issues
check:
    cargo fmt -v --check

# format rust files
fmt:
    cargo fmt -v
