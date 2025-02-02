#![doc = include_str!("../README.md")]

mod config;
mod game;
mod world;
use config::DEFAULT_CONFIG;
use game::GameBuilder;

fn main() {
    let mut game = GameBuilder::new(&DEFAULT_CONFIG).build();
    game.run();
}
