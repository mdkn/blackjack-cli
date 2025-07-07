mod card;
mod deck;
mod hand;
mod game;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}