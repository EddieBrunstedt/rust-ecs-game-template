//! A game engine implementation using Entity Component System (ECS) architecture.
//! Built with Raylib for rendering and input handling.

mod config;
mod game;
mod world;
use config::DEFAULT_CONFIG;
use game::GameBuilder;

fn main() {
    let mut game = GameBuilder::new(&DEFAULT_CONFIG).build();
    game.run();
}
