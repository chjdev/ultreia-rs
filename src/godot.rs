mod events;
mod game;
mod game_instance;
mod terrain;
mod variant;

use crate::godot::game::Game;
use crate::godot::terrain::Terrain;
use gdnative::prelude::*;

/// Register the godot adapters
fn init(handle: InitHandle) {
    handle.add_class::<Game>();
    handle.add_class::<Terrain>();
}

// create entry points for library
godot_init!(init);
