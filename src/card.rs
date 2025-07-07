use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn value(&self) -> u8 {
        match self.rank {
            Rank::Ace => 11,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_str = match self.rank {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };

        let suit_str = match self.suit {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };

        write!(f, "{}{}", rank_str, suit_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_value() {
        assert_eq!(Card { suit: Suit::Hearts, rank: Rank::Ace }.value(), 11);
        assert_eq!(Card { suit: Suit::Diamonds, rank: Rank::Two }.value(), 2);
        assert_eq!(Card { suit: Suit::Clubs, rank: Rank::Three }.value(), 3);
        assert_eq!(Card { suit: Suit::Spades, rank: Rank::Four }.value(), 4);
        assert_eq!(Card { suit: Suit::Hearts, rank: Rank::Five }.value(), 5);
        assert_eq!(Card { suit: Suit::Diamonds, rank: Rank::Six }.value(), 6);
        assert_eq!(Card { suit: Suit::Clubs, rank: Rank::Seven }.value(), 7);
        assert_eq!(Card { suit: Suit::Spades, rank: Rank::Eight }.value(), 8);
        assert_eq!(Card { suit: Suit::Hearts, rank: Rank::Nine }.value(), 9);
        assert_eq!(Card { suit: Suit::Diamonds, rank: Rank::Ten }.value(), 10);
        assert_eq!(Card { suit: Suit::Clubs, rank: Rank::Jack }.value(), 10);
        assert_eq!(Card { suit: Suit::Spades, rank: Rank::Queen }.value(), 10);
        assert_eq!(Card { suit: Suit::Hearts, rank: Rank::King }.value(), 10);
    }

    #[test]
    fn test_card_display() {
        assert_eq!(format!("{}", Card { suit: Suit::Hearts, rank: Rank::Ace }), "A♥");
        assert_eq!(format!("{}", Card { suit: Suit::Diamonds, rank: Rank::King }), "K♦");
        assert_eq!(format!("{}", Card { suit: Suit::Clubs, rank: Rank::Ten }), "10♣");
        assert_eq!(format!("{}", Card { suit: Suit::Spades, rank: Rank::Two }), "2♠");
    }

    #[test]
    fn test_suit_equality() {
        assert_eq!(Suit::Hearts, Suit::Hearts);
        assert_ne!(Suit::Hearts, Suit::Diamonds);
    }

    #[test]
    fn test_rank_equality() {
        assert_eq!(Rank::Ace, Rank::Ace);
        assert_ne!(Rank::Ace, Rank::King);
    }
}