# https://github.com/casey/just

# Build test and lint
default: build test lint

# Build project
build:
    cargo build

# Run tests
test:
    cargo test

# Lint code
lint:
    cargo fmt --all -- --check
    cargo clippy --workspace -- -D warnings

# Generate and commit a release
release +args:
    @ command -v panrelease &> /dev/null || (echo "panrelease not found in \$PATH. Please install it using 'cargo install panrelease'" && exit 1)
    panrelease release {{args}}
