# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Building and Running
- `cargo build` - Build the project
- `cargo run` - Run the blackjack game
- `cargo check` - Quick compile check without building executable

### Testing
- `cargo test` - Run all tests (currently no tests implemented)

## Architecture

This is a single-file CLI blackjack game written in Rust. The architecture is organized around these core components:

### Card System
- `Card` struct with `Suit` and `Rank` enums
- `Deck` struct manages a standard 52-card deck with shuffling and dealing
- Card values follow blackjack rules (Ace = 11/1, face cards = 10)

### Game Logic
- `Hand` struct manages collections of cards with automatic Ace value adjustment
- `Game` struct orchestrates the entire game flow including betting, dealing, and win/loss determination
- Game implements standard blackjack rules: dealer hits on soft 17, blackjack pays 3:2

### Game Flow
1. Player places bet from chip balance (starts with 1000 chips)
2. Initial deal: 2 cards each to player and dealer
3. Player turn: hit or stand until bust or stand
4. Dealer turn: automatic play following house rules
5. Win/loss determination and chip adjustment
6. Repeat until player quits or runs out of chips

All game logic is contained in `src/main.rs` as a single-file implementation.