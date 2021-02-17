use gdnative::prelude::*;

use crate::coordinate::Coordinate;
use crate::game::Configuration;
use crate::godot::events::clock_events::ClockEvents;
use crate::godot::game_instance::GameController;
use crate::map::terrain::{TerrainTile, TerrainType};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct Game {
    clock_events: Option<ClockEvents>,
}

impl Game {
    fn new(_onwer: &Node) -> Self {
        Game { clock_events: None }
    }
}

#[methods]
impl Game {
    fn register_signals(builder: &ClassBuilder<Self>) {
        // see https://github.com/godot-rust/godot-rust/blob/master/examples/signals/src/lib.rs
        builder.add_signal(Signal {
            name: "start_game",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "tick",
            args: &[SignalArgument {
                name: "epoch",
                default: Variant::from_i64(0),
                export_info: ExportInfo::new(VariantType::I64),
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "tock",
            args: &[SignalArgument {
                name: "epoch",
                default: Variant::from_i64(0),
                export_info: ExportInfo::new(VariantType::I64),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("I ARE GAME!!!!");
    }

    #[export]
    fn start_game(&mut self, owner: &Node, configuration: Configuration) {
        godot_print!("starting game now");
        let my_ref = owner
            .get_node("/root/Game")
            .expect("no main node at: /root/Game");
        GameController
            .start(configuration)
            .expect("should be possible to start the game");
        let game = GameController.game().expect("game should be here");
        let clock_events = ClockEvents::new(game.clock(), my_ref);
        self.clock_events = Some(clock_events);
        owner.emit_signal(GodotString::from_str("start_game"), &[]);
    }

    #[export]
    fn tick(&self, _owner: &Node) {
        GameController
            .game()
            .iter()
            .for_each(|game| game.clock().tick());
    }

    #[export]
    fn configuration(&self, _owner: &Node) -> Option<Configuration> {
        Some(*GameController.game()?.configuration())
    }
}
