extern crate base;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;

pub mod render;
mod config;
mod game;

pub use config::Config;


pub fn start_game(config: Config) -> Result<(), ()> {
    game::run(&config)
}
