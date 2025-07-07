use crate::card::{Card, Rank};

pub fn render_card(card: &Card) -> Vec<String> {
    let rank_str = match card.rank {
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

    let suit_str = match card.suit {
        crate::card::Suit::Hearts => "♥",
        crate::card::Suit::Diamonds => "♦",
        crate::card::Suit::Clubs => "♣",
        crate::card::Suit::Spades => "♠",
    };

    let content = format!("{}{}", rank_str, suit_str);

    vec![
        "╔═══╗".to_string(),
        format!("║{:<3}║", content),
        "╚═══╝".to_string(),
    ]
}

pub fn render_hidden_card() -> Vec<String> {
    vec![
        "╔═══╗".to_string(),
        "║░░░║".to_string(),
        "╚═══╝".to_string(),
    ]
}

pub fn render_cards_horizontal(cards: &[Vec<String>]) -> String {
    if cards.is_empty() {
        return String::new();
    }

    let height = cards[0].len();
    let mut result = Vec::new();

    for row in 0..height {
        let mut line = String::new();
        for (i, card) in cards.iter().enumerate() {
            if i > 0 {
                line.push(' ');
            }
            line.push_str(&card[row]);
        }
        result.push(line);
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn test_render_card() {
        let card = Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let rendered = render_card(&card);
        assert_eq!(rendered[0], "╔═══╗");
        assert_eq!(rendered[1], "║A♥ ║");
        assert_eq!(rendered[2], "╚═══╝");
    }

    #[test]
    fn test_render_card_ten() {
        let card = Card {
            suit: Suit::Spades,
            rank: Rank::Ten,
        };
        let rendered = render_card(&card);
        assert_eq!(rendered[1], "║10♠║");
    }

    #[test]
    fn test_render_hidden_card() {
        let rendered = render_hidden_card();
        assert_eq!(rendered[0], "╔═══╗");
        assert_eq!(rendered[1], "║░░░║");
        assert_eq!(rendered[2], "╚═══╝");
    }

    #[test]
    fn test_render_cards_horizontal() {
        let card1 = render_card(&Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        });
        let card2 = render_card(&Card {
            suit: Suit::Spades,
            rank: Rank::King,
        });

        let result = render_cards_horizontal(&[card1, card2]);
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(lines[0], "╔═══╗ ╔═══╗");
        assert_eq!(lines[1], "║A♥ ║ ║K♠ ║");
        assert_eq!(lines[2], "╚═══╝ ╚═══╝");
    }
}
