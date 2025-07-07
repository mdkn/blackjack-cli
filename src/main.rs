mod card;
mod deck;
mod display;
mod game;
mod hand;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
