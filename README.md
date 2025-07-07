# Blackjack CLI

A command-line blackjack game written in Rust.

## Installation

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

Clone or download this repository, then build the project:

```bash
cargo build --release
```

## Usage

Run the game with:

```bash
cargo run
```

### How to Play

1. **Start the game** - You begin with 1000 chips
2. **Place your bet** - Enter the amount you want to wager (or 0 to quit)
3. **Play your hand** - Choose to hit (h) or stand (s)
4. **Win or lose** - The game determines the winner and adjusts your chips

### Game Rules

- Standard blackjack rules apply
- Dealer hits on soft 17
- Blackjack pays 3:2
- Aces count as 11 or 1 (automatically adjusted)
- Face cards (J, Q, K) count as 10
- Game continues until you quit or run out of chips

### Controls

- `h` or `hit` - Take another card
- `s` or `stand` - Keep your current hand
- `0` - Quit the game when placing a bet

## Features

- Full 52-card deck with shuffling
- Automatic Ace value adjustment
- Betting system with chip tracking
- Unicode card symbols (♠ ♥ ♦ ♣)
- Clear display of hands and values
- Proper blackjack win/loss conditions

## Requirements

- Rust 1.70 or later
- No additional system dependencies

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Linting

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy

# Run clippy with strict warnings
cargo clippy -- -D warnings
```

### Using Make

This project includes a Makefile for convenience:

```bash
# Show available targets
make help

# Build project
make build

# Run tests
make test

# Format and lint
make fmt
make clippy
```

## GitHub Actions

This project includes comprehensive CI/CD workflows:

### CI Pipeline (`.github/workflows/ci.yml`)
- **Triggers**: Push to main/develop, PRs to main/develop
- **Tests**: Runs on stable, beta, and nightly Rust
- **Platforms**: Linux, Windows, macOS
- **Checks**: Tests, formatting, clippy, code coverage
- **Artifacts**: Build artifacts for all platforms

### Release Pipeline (`.github/workflows/release.yml`)
- **Triggers**: Git tags starting with `v` (e.g., `v1.0.0`)
- **Builds**: Cross-platform binaries (Linux, Windows, macOS, ARM64)
- **Releases**: Automatic GitHub releases with assets
- **Publishing**: Automatic crates.io publishing for stable releases

### Security Audit (`.github/workflows/security.yml`)
- **Triggers**: Push to main, PRs, weekly schedule
- **Checks**: Dependency vulnerabilities, license compliance, unsafe code detection
- **Tools**: cargo-audit, cargo-deny, cargo-geiger

### Performance Benchmarks (`.github/workflows/benchmarks.yml`)
- **Triggers**: Push to main, PRs
- **Benchmarks**: Performance regression testing
- **Artifacts**: Benchmark results and reports

### Creating a Release

1. Update version in `Cargo.toml`
2. Create and push a git tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
3. The release workflow will automatically:
   - Build binaries for all platforms
   - Create a GitHub release
   - Upload assets
   - Publish to crates.io (for stable releases)

## Project Structure

```
src/
├── main.rs          # Entry point
├── lib.rs           # Library exports
├── card.rs          # Card, Suit, and Rank definitions
├── deck.rs          # Deck management
├── hand.rs          # Hand evaluation and display
├── game.rs          # Game logic and flow
└── display.rs       # ASCII art card rendering

.github/
├── workflows/       # GitHub Actions workflows
├── dependabot.yml   # Automated dependency updates
└── ...

deny.toml           # Security policy configuration
Makefile            # Build automation
```

## License

This project is licensed under the MIT License.