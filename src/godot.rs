mod game;
mod variant;

use crate::godot::game::Game;
use gdnative::prelude::*;

/// Register the godot adapters
fn init(handle: InitHandle) {
    handle.add_class::<Game>();
}

// create entry points for library
godot_init!(init);
