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