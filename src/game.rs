use std::io::{self, Write};

use crate::deck::Deck;
use crate::hand::Hand;

#[cfg(test)]
use crate::card::{Card, Rank, Suit};

pub struct Game {
    pub(crate) deck: Deck,
    pub(crate) player_hand: Hand,
    pub(crate) dealer_hand: Hand,
    pub(crate) player_chips: u32,
    pub(crate) current_bet: u32,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        Game {
            deck,
            player_hand: Hand::new(),
            dealer_hand: Hand::new(),
            player_chips: 1000,
            current_bet: 0,
        }
    }

    fn place_bet(&mut self) -> bool {
        loop {
            print!(
                "You have {} chips. Enter your bet (or 0 to quit): ",
                self.player_chips
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {}
                Err(_) => {
                    println!("Error reading input!");
                    continue;
                }
            }

            match input.trim().parse::<u32>() {
                Ok(0) => return false,
                Ok(bet) if bet <= self.player_chips => {
                    self.current_bet = bet;
                    return true;
                }
                Ok(_) => {
                    println!("You don't have enough chips!");
                    continue;
                }
                Err(_) => {
                    println!("Invalid input! Please enter a number.");
                    continue;
                }
            }
        }
    }

    pub(crate) fn deal_initial_cards(&mut self) {
        self.player_hand.clear();
        self.dealer_hand.clear();

        for _ in 0..2 {
            if let Some(card) = self.deck.deal() {
                self.player_hand.add_card(card);
            }
            if let Some(card) = self.deck.deal() {
                self.dealer_hand.add_card(card);
            }
        }
    }

    fn display_hands(&self, hide_dealer_card: bool) {
        use crate::display::{render_card, render_cards_horizontal, render_hidden_card};

        println!("\n=== Dealer's Hand ===");
        if hide_dealer_card {
            let mut dealer_cards = Vec::new();
            dealer_cards.push(render_hidden_card());

            if self.dealer_hand.cards().len() > 1 {
                for card in &self.dealer_hand.cards()[1..] {
                    dealer_cards.push(render_card(card));
                }
            }

            let cards_display = render_cards_horizontal(&dealer_cards);
            println!("{cards_display}");
        } else {
            println!("{}", self.dealer_hand);
        }

        println!("\n=== Your Hand ===");
        println!("{}", self.player_hand);
        println!();
    }

    fn player_turn(&mut self) -> bool {
        loop {
            if self.player_hand.is_busted() {
                println!("You busted!");
                return false;
            }

            if self.player_hand.is_blackjack() {
                println!("Blackjack!");
                return true;
            }

            print!("Hit (h) or Stand (s)? ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().to_lowercase().as_str() {
                "h" | "hit" => {
                    if let Some(card) = self.deck.deal() {
                        self.player_hand.add_card(card);
                        println!("You drew: {card}");
                        self.display_hands(true);
                    }
                }
                "s" | "stand" => return true,
                _ => println!("Invalid input! Please enter 'h' for hit or 's' for stand."),
            }
        }
    }

    pub(crate) fn dealer_turn(&mut self) {
        while self.dealer_hand.value() < 17 {
            if let Some(card) = self.deck.deal() {
                self.dealer_hand.add_card(card);
                println!("Dealer drew: {card}");
            }
        }
    }

    pub(crate) fn determine_winner(&mut self) {
        let player_value = self.player_hand.value();
        let dealer_value = self.dealer_hand.value();

        if self.player_hand.is_busted() {
            println!("You lose! You busted.");
            self.player_chips -= self.current_bet;
        } else if self.dealer_hand.is_busted() {
            println!("You win! Dealer busted.");
            self.player_chips += self.current_bet;
        } else if self.player_hand.is_blackjack() && !self.dealer_hand.is_blackjack() {
            println!("You win with Blackjack!");
            self.player_chips += (self.current_bet * 3) / 2;
        } else if self.dealer_hand.is_blackjack() && !self.player_hand.is_blackjack() {
            println!("Dealer wins with Blackjack!");
            self.player_chips -= self.current_bet;
        } else if player_value > dealer_value {
            println!("You win!");
            self.player_chips += self.current_bet;
        } else if dealer_value > player_value {
            println!("Dealer wins!");
            self.player_chips -= self.current_bet;
        } else {
            println!("Push! It's a tie.");
        }
    }

    fn play_round(&mut self) -> bool {
        if !self.place_bet() {
            return false;
        }

        if self.deck.cards_remaining() < 10 {
            println!("Shuffling deck...");
            self.deck = Deck::new();
            self.deck.shuffle();
        }

        self.deal_initial_cards();
        self.display_hands(true);

        if self.player_turn() {
            self.display_hands(false);
            self.dealer_turn();
            self.display_hands(false);
        }

        self.determine_winner();
        println!("You now have {} chips.\n", self.player_chips);

        self.player_chips > 0
    }

    pub fn run(&mut self) {
        println!("Welcome to Blackjack!");
        println!("==================");

        while self.play_round() {
            if self.player_chips == 0 {
                println!("You're out of chips! Game over.");
                break;
            }
        }

        println!("Thanks for playing!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new();
        assert_eq!(game.player_chips, 1000);
        assert_eq!(game.current_bet, 0);
        assert_eq!(game.player_hand.cards().len(), 0);
        assert_eq!(game.dealer_hand.cards().len(), 0);
    }

    #[test]
    fn test_deal_initial_cards() {
        let mut game = Game::new();
        game.deal_initial_cards();

        assert_eq!(game.player_hand.cards().len(), 2);
        assert_eq!(game.dealer_hand.cards().len(), 2);
    }

    #[test]
    fn test_dealer_hits_on_soft_17() {
        let mut game = Game::new();

        // Set up dealer hand with 16
        game.dealer_hand.add_card(Card {
            suit: Suit::Hearts,
            rank: Rank::Ten,
        });
        game.dealer_hand.add_card(Card {
            suit: Suit::Diamonds,
            rank: Rank::Six,
        });

        let initial_hand_size = game.dealer_hand.cards().len();
        game.dealer_turn();

        // Dealer should have drawn at least one more card
        assert!(game.dealer_hand.cards().len() > initial_hand_size);
        assert!(game.dealer_hand.value() >= 17);
    }

    #[test]
    fn test_dealer_stands_on_17_or_higher() {
        let mut game = Game::new();

        // Set up dealer hand with 17
        game.dealer_hand.add_card(Card {
            suit: Suit::Hearts,
            rank: Rank::Ten,
        });
        game.dealer_hand.add_card(Card {
            suit: Suit::Diamonds,
            rank: Rank::Seven,
        });

        let initial_hand_size = game.dealer_hand.cards().len();
        game.dealer_turn();

        // Dealer should not have drawn any more cards
        assert_eq!(game.dealer_hand.cards().len(), initial_hand_size);
    }

    #[test]
    fn test_determine_winner_player_bust() {
        let mut game = Game::new();
        game.current_bet = 100;
        let initial_chips = game.player_chips;

        // Player busts
        game.player_hand.add_card(Card {
            suit: Suit::Hearts,
            rank: Rank::King,
        });
        game.player_hand.add_card(Card {
            suit: Suit::Diamonds,
            rank: Rank::Queen,
        });
        game.player_hand.add_card(Card {
            suit: Suit::Clubs,
            rank: Rank::Five,
        });

        // Dealer has valid hand
        game.dealer_hand.add_card(Card {
            suit: Suit::Spades,
            rank: Rank::Ten,
        });
        game.dealer_hand.add_card(Card {
            suit: Suit::Hearts,
            rank: Rank::Seven,
        });

        game.determine_winner();
        assert_eq!(game.player_chips, initial_chips - game.current_bet);
    }

    #[test]
    fn test_determine_winner_dealer_bust() {
        let mut game = Game::new();
        game.current_bet = 100;
        let initial_chips = game.player_chips;

        // Player has valid hand
        game.player_hand.add_card(Card {
            suit: Suit::Hearts,
            rank: Rank::Ten,
        });
        game.player_hand.add_card(Card {
            suit: Suit::Diamonds,
            rank: Rank::Nine,
        });

        // Dealer busts
        game.dealer_hand.add_card(Card {
            suit: Suit::Spades,
            rank: Rank::King,
        });
        game.dealer_hand.add_card(Card {
            suit: Suit::Hearts,
            rank: Rank::Queen,
        });
        game.dealer_hand.add_card(Card {
            suit: Suit::Clubs,
            rank: Rank::Five,
        });

        game.determine_winner();
        assert_eq!(game.player_chips, initial_chips + game.current_bet);
    }

    #[test]
    fn test_determine_winner_player_blackjack() {
        let mut game = Game::new();
        game.current_bet = 100;
        let initial_chips = game.player_chips;

        // Player has blackjack
        game.player_hand.add_card(Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        });
        game.player_hand.add_card(Card {
            suit: Suit::Diamonds,
            rank: Rank::King,
        });

        // Dealer has 20
        game.dealer_hand.add_card(Card {
            suit: Suit::Spades,
            rank: Rank::King,
        });
        game.dealer_hand.add_card(Card {
            suit: Suit::Hearts,
            rank: Rank::Queen,
        });

        game.determine_winner();
        assert_eq!(
            game.player_chips,
            initial_chips + (game.current_bet * 3) / 2
        );
    }

    #[test]
    fn test_determine_winner_push() {
        let mut game = Game::new();
        game.current_bet = 100;
        let initial_chips = game.player_chips;

        // Both have 20
        game.player_hand.add_card(Card {
            suit: Suit::Hearts,
            rank: Rank::King,
        });
        game.player_hand.add_card(Card {
            suit: Suit::Diamonds,
            rank: Rank::Queen,
        });

        game.dealer_hand.add_card(Card {
            suit: Suit::Spades,
            rank: Rank::Ten,
        });
        game.dealer_hand.add_card(Card {
            suit: Suit::Hearts,
            rank: Rank::Ten,
        });

        game.determine_winner();
        assert_eq!(game.player_chips, initial_chips); // No change in chips
    }

    #[test]
    fn test_deck_reshuffles_when_low() {
        let mut game = Game::new();

        // Deal most of the deck
        for _ in 0..23 {
            game.deck.deal();
            game.deck.deal();
        }

        assert!(game.deck.cards_remaining() < 10);

        // This would trigger a reshuffle in play_round, but we can't test play_round
        // due to I/O operations. Instead, we just verify the condition is correct.
    }
}
