use std::fmt;

use crate::card::{Card, Rank};

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn value(&self) -> u8 {
        let mut total = 0;
        let mut aces = 0;

        for card in &self.cards {
            if card.rank == Rank::Ace {
                aces += 1;
            }
            total += card.value();
        }

        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }

        total
    }

    pub fn is_busted(&self) -> bool {
        self.value() > 21
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, card) in self.cards.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", card)?;
        }
        write!(f, " ({})", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn test_new_hand_is_empty() {
        let hand = Hand::new();
        assert_eq!(hand.cards().len(), 0);
        assert_eq!(hand.value(), 0);
    }

    #[test]
    fn test_add_card() {
        let mut hand = Hand::new();
        let card = Card { suit: Suit::Hearts, rank: Rank::King };
        
        hand.add_card(card);
        assert_eq!(hand.cards().len(), 1);
        assert_eq!(hand.value(), 10);
    }

    #[test]
    fn test_hand_value_simple() {
        let mut hand = Hand::new();
        hand.add_card(Card { suit: Suit::Hearts, rank: Rank::Five });
        hand.add_card(Card { suit: Suit::Diamonds, rank: Rank::Seven });
        
        assert_eq!(hand.value(), 12);
    }

    #[test]
    fn test_ace_high_value() {
        let mut hand = Hand::new();
        hand.add_card(Card { suit: Suit::Hearts, rank: Rank::Ace });
        hand.add_card(Card { suit: Suit::Diamonds, rank: Rank::Nine });
        
        assert_eq!(hand.value(), 20); // Ace counts as 11
    }

    #[test]
    fn test_ace_low_value() {
        let mut hand = Hand::new();
        hand.add_card(Card { suit: Suit::Hearts, rank: Rank::Ace });
        hand.add_card(Card { suit: Suit::Diamonds, rank: Rank::King });
        hand.add_card(Card { suit: Suit::Clubs, rank: Rank::Five });
        
        assert_eq!(hand.value(), 16); // Ace counts as 1 to avoid bust
    }

    #[test]
    fn test_multiple_aces() {
        let mut hand = Hand::new();
        hand.add_card(Card { suit: Suit::Hearts, rank: Rank::Ace });
        hand.add_card(Card { suit: Suit::Diamonds, rank: Rank::Ace });
        
        assert_eq!(hand.value(), 12); // One Ace as 11, one as 1
        
        hand.add_card(Card { suit: Suit::Clubs, rank: Rank::Nine });
        assert_eq!(hand.value(), 21); // Both Aces as 1
    }

    #[test]
    fn test_is_busted() {
        let mut hand = Hand::new();
        hand.add_card(Card { suit: Suit::Hearts, rank: Rank::King });
        hand.add_card(Card { suit: Suit::Diamonds, rank: Rank::Queen });
        
        assert!(!hand.is_busted());
        
        hand.add_card(Card { suit: Suit::Clubs, rank: Rank::Five });
        assert!(hand.is_busted());
    }

    #[test]
    fn test_is_blackjack() {
        let mut hand = Hand::new();
        hand.add_card(Card { suit: Suit::Hearts, rank: Rank::Ace });
        hand.add_card(Card { suit: Suit::Diamonds, rank: Rank::King });
        
        assert!(hand.is_blackjack());
        
        // Not blackjack with 3 cards even if value is 21
        hand.add_card(Card { suit: Suit::Clubs, rank: Rank::Two });
        assert!(!hand.is_blackjack());
    }

    #[test]
    fn test_not_blackjack_with_21_in_3_cards() {
        let mut hand = Hand::new();
        hand.add_card(Card { suit: Suit::Hearts, rank: Rank::Seven });
        hand.add_card(Card { suit: Suit::Diamonds, rank: Rank::Seven });
        hand.add_card(Card { suit: Suit::Clubs, rank: Rank::Seven });
        
        assert_eq!(hand.value(), 21);
        assert!(!hand.is_blackjack());
    }

    #[test]
    fn test_clear_hand() {
        let mut hand = Hand::new();
        hand.add_card(Card { suit: Suit::Hearts, rank: Rank::King });
        hand.add_card(Card { suit: Suit::Diamonds, rank: Rank::Queen });
        
        assert_eq!(hand.cards().len(), 2);
        
        hand.clear();
        assert_eq!(hand.cards().len(), 0);
        assert_eq!(hand.value(), 0);
    }

    #[test]
    fn test_hand_display() {
        let mut hand = Hand::new();
        hand.add_card(Card { suit: Suit::Hearts, rank: Rank::Ace });
        hand.add_card(Card { suit: Suit::Diamonds, rank: Rank::King });
        
        let display = format!("{}", hand);
        assert_eq!(display, "A♥ K♦ (21)");
    }
}