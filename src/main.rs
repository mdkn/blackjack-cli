use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rank {
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
struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn value(&self) -> u8 {
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

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
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

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> u8 {
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

    fn is_busted(&self) -> bool {
        self.value() > 21
    }

    fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }

    fn clear(&mut self) {
        self.cards.clear();
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, card) in self.cards.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", card)?;
        }
        write!(f, " ({})", self.value())
    }
}

struct Game {
    deck: Deck,
    player_hand: Hand,
    dealer_hand: Hand,
    player_chips: u32,
    current_bet: u32,
}

impl Game {
    fn new() -> Self {
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
        print!("You have {} chips. Enter your bet (or 0 to quit): ", self.player_chips);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<u32>() {
            Ok(0) => false,
            Ok(bet) if bet <= self.player_chips => {
                self.current_bet = bet;
                true
            }
            Ok(_) => {
                println!("You don't have enough chips!");
                self.place_bet()
            }
            Err(_) => {
                println!("Invalid input!");
                self.place_bet()
            }
        }
    }

    fn deal_initial_cards(&mut self) {
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
        println!("\nYour hand: {}", self.player_hand);
        
        if hide_dealer_card {
            println!("Dealer's hand: {} [Hidden]", self.dealer_hand.cards[0]);
        } else {
            println!("Dealer's hand: {}", self.dealer_hand);
        }
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
                        println!("You drew: {}", card);
                        self.display_hands(true);
                    }
                }
                "s" | "stand" => return true,
                _ => println!("Invalid input! Please enter 'h' for hit or 's' for stand."),
            }
        }
    }

    fn dealer_turn(&mut self) {
        while self.dealer_hand.value() < 17 {
            if let Some(card) = self.deck.deal() {
                self.dealer_hand.add_card(card);
                println!("Dealer drew: {}", card);
            }
        }
    }

    fn determine_winner(&mut self) {
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

        if self.deck.cards.len() < 10 {
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

    fn run(&mut self) {
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

fn main() {
    let mut game = Game::new();
    game.run();
}