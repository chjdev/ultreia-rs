use gdnative::prelude::*;

use crate::coordinate::Coordinate;
use crate::game;
use crate::game::Configuration;
use crate::map::terrain::{TerrainTile, TerrainType};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Game {
    game: Option<game::Game>,
}

impl Game {
    fn new(_owner: &Node) -> Self {
        Game { game: None }
    }
}

#[methods]
impl Game {
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("I ARE GAME!!!!");
    }

    #[export]
    fn start_game(&mut self, _owner: &Node, size: usize) {
        godot_print!("starting game now");
        self.game = Some(game::Game::new(Configuration::new(size, size)));
    }

    #[export]
    fn gameroo(&self, _owner: &Node) {
        if let Some(game) = &self.game {
            godot_print!("game size {}", game.configuration().rows());
        }
    }

    #[export]
    fn terrain_type_at(&self, _owner: &Node, coordinate: Coordinate) -> Option<TerrainType> {
        self.game
            .as_ref()
            .map(|game| game.map().terrain().get(&coordinate).terrain_type())
    }

    #[export]
    fn terrain_at(&self, _owner: &Node, coordinate: Coordinate) -> Option<TerrainTile> {
        godot_print!("blupp {} {}", coordinate.x(), coordinate.y());
        self.game
            .as_ref()
            .map(|game| game.map().terrain().get(&coordinate))
    }
}
