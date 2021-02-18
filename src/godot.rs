mod clock;
mod game;
mod game_controller;
mod terrain;
mod variant;

use crate::godot::clock::Clock;
use crate::godot::game::Game;
use crate::godot::terrain::Terrain;
use gdnative::prelude::*;

/// Register the godot adapters
fn init(handle: InitHandle) {
    handle.add_class::<Game>();
    handle.add_class::<Terrain>();
    handle.add_class::<Clock>();
}

// create entry points for library
godot_init!(init);
