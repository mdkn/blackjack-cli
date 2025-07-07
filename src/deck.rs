use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::{Card, Rank, Suit};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in [
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ] {
                cards.push(Card { suit, rank });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn cards_remaining(&self) -> usize {
        self.cards.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck_has_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.cards_remaining(), 52);
    }

    #[test]
    fn test_new_deck_contains_all_cards() {
        let deck = Deck::new();
        
        // Check we have 13 cards of each suit
        let hearts_count = deck.cards.iter().filter(|c| c.suit == Suit::Hearts).count();
        let diamonds_count = deck.cards.iter().filter(|c| c.suit == Suit::Diamonds).count();
        let clubs_count = deck.cards.iter().filter(|c| c.suit == Suit::Clubs).count();
        let spades_count = deck.cards.iter().filter(|c| c.suit == Suit::Spades).count();
        
        assert_eq!(hearts_count, 13);
        assert_eq!(diamonds_count, 13);
        assert_eq!(clubs_count, 13);
        assert_eq!(spades_count, 13);
        
        // Check we have 4 of each rank
        let ace_count = deck.cards.iter().filter(|c| c.rank == Rank::Ace).count();
        let king_count = deck.cards.iter().filter(|c| c.rank == Rank::King).count();
        
        assert_eq!(ace_count, 4);
        assert_eq!(king_count, 4);
    }

    #[test]
    fn test_deal_reduces_deck_size() {
        let mut deck = Deck::new();
        let initial_size = deck.cards_remaining();
        
        let card = deck.deal();
        assert!(card.is_some());
        assert_eq!(deck.cards_remaining(), initial_size - 1);
    }

    #[test]
    fn test_deal_from_empty_deck() {
        let mut deck = Deck::new();
        
        // Deal all cards
        for _ in 0..52 {
            assert!(deck.deal().is_some());
        }
        
        // Now deck should be empty
        assert_eq!(deck.cards_remaining(), 0);
        assert!(deck.deal().is_none());
    }

    #[test]
    fn test_shuffle_maintains_deck_size() {
        let mut deck = Deck::new();
        let size_before = deck.cards_remaining();
        
        deck.shuffle();
        
        assert_eq!(deck.cards_remaining(), size_before);
    }

    #[test]
    fn test_shuffle_changes_order() {
        let deck1 = Deck::new();
        let mut deck2 = Deck::new();
        
        // Get initial order of first 10 cards
        let initial_cards: Vec<_> = deck1.cards.iter().take(10).cloned().collect();
        
        // Shuffle deck2
        deck2.shuffle();
        
        // Get shuffled order of first 10 cards
        let shuffled_cards: Vec<_> = deck2.cards.iter().take(10).cloned().collect();
        
        // It's extremely unlikely (but not impossible) that shuffle doesn't change order
        // We check just the ranks to avoid issues with derive Clone/PartialEq
        let initial_ranks: Vec<_> = initial_cards.iter().map(|c| c.rank).collect();
        let shuffled_ranks: Vec<_> = shuffled_cards.iter().map(|c| c.rank).collect();
        
        // This test might fail very rarely due to randomness, but probability is negligible
        assert_ne!(initial_ranks, shuffled_ranks);
    }
}