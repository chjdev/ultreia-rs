use gdnative::prelude::*;

use crate::game::Configuration;
use crate::godot::game_instance::GameController;

use strum_macros::AsRefStr;

#[derive(Copy, Clone, PartialEq, Eq, AsRefStr)]
pub enum GameSignal {
    GameStart,
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct Game;

impl Game {
    fn new(_onwer: &Node) -> Self {
        Game {}
    }
}

#[methods]
impl Game {
    fn register_signals(builder: &ClassBuilder<Self>) {
        // see https://github.com/godot-rust/godot-rust/blob/master/examples/signals/src/lib.rs
        builder.add_signal(Signal {
            name: GameSignal::GameStart.as_ref(),
            args: &[],
        });
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("I ARE GAME!!!!");
    }

    #[export]
    fn start_game(&mut self, owner: &Node, configuration: Configuration) {
        godot_print!("starting game now");
        GameController
            .start(configuration)
            .expect("should be possible to start the game");
        owner.emit_signal(GameSignal::GameStart, &[]);
    }

    #[export]
    fn configuration(&self, _owner: &Node) -> Option<Configuration> {
        Some(*GameController.game()?.configuration())
    }
}
