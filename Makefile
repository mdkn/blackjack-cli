.PHONY: build run clean test check fmt clippy install help

# Default target
all: build

# Build the project
build:
	cargo build

# Build for release
build-release:
	cargo build --release

# Run the game
run:
	cargo run

# Run tests
test:
	cargo test

# Quick compile check
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Run clippy for linting
clippy:
	cargo clippy

# Clean build artifacts
clean:
	cargo clean

# Install the binary
install:
	cargo install --path .

# Show help
help:
	@echo "Available targets:"
	@echo "  build         - Build the project"
	@echo "  build-release - Build optimized release version"
	@echo "  run           - Run the blackjack game"
	@echo "  test          - Run tests"
	@echo "  check         - Quick compile check"
	@echo "  fmt           - Format code"
	@echo "  clippy        - Run clippy linter"
	@echo "  clean         - Clean build artifacts"
	@echo "  install       - Install binary to system"
	@echo "  help          - Show this help message"