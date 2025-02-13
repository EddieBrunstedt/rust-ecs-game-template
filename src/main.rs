#![doc = include_str!("../README.md")]

mod config;
mod game;
mod world;

use game::Game;

fn main() {
    Game::new().run();
}
