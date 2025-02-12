#![doc = include_str!("../README.md")]

mod config;
mod game;
mod world;

use game::GameBuilder;

fn main() {
    let mut game = GameBuilder::new().enter();
    game.run();
}
