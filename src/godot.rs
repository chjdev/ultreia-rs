mod arcer;
mod buildings;
mod clock;
mod fow;
mod game;
mod game_controller;
mod good;
mod terrain;
mod territory;
mod variant;

use crate::godot::clock::Clock;
use crate::godot::fow::FOW;
use crate::godot::game::Game;
use crate::godot::good::Good;
use crate::godot::terrain::Terrain;
use crate::godot::territory::Territory;
use gdnative::prelude::*;

/// Register the godot adapters
fn init(handle: InitHandle) {
    handle.add_class::<Game>();
    handle.add_class::<Terrain>();
    handle.add_class::<Clock>();
    handle.add_class::<Good>();
    handle.add_class::<FOW>();
    handle.add_class::<Territory>();
}

// create entry points for library
godot_init!(init);
