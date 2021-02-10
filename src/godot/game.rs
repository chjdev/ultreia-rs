use gdnative::prelude::*;

use crate::coordinate::Coordinate;
use crate::game;
use crate::game::Configuration;
use crate::godot::events::clock_events::ClockEvents;
use crate::map::terrain::{TerrainTile, TerrainType};
use std::sync::{Arc, Mutex};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
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
        let game = game::Game::new(configuration);
        let clock_events = ClockEvents::new(game.clock(), &Arc::new(Mutex::new()));
        self.game = Some(game);
        owner.emit_signal(GodotString::from_str("start_game"), &[]);
    }

    #[export]
    fn configuration(&self, _owner: &Node) -> Option<Configuration> {
        self.game.as_ref().map(|game| *game.configuration())
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

    #[export]
    fn minimap(&self, _owner: &Node, width: u16, height: u16) -> Option<Vec<TerrainType>> {
        self.game
            .as_ref()
            .map(|game| game.map().terrain().minimap(width, height))
    }
}
