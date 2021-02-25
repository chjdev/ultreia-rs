mod buildings_signal;

use gdnative::prelude::*;

use crate::coordinate::Coordinate;
use crate::godot::buildings::buildings_signal::{BuildingsObserver, BuildingsSignal};
use crate::godot::game::GameSignal;
use crate::godot::game_controller::GameController;
use std::sync::Arc;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct Buildings {
    buildings_observer: Option<Arc<BuildingsObserver>>,
}

impl Buildings {
    fn new(_onwer: &Node) -> Self {
        Buildings {
            buildings_observer: None,
        }
    }
}

#[methods]
impl Buildings {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: BuildingsSignal::Created.as_ref(),
            args: &[
                SignalArgument {
                    name: "coordinate",
                    default: Coordinate::default().to_variant(),
                    export_info: ExportInfo::new(VariantType::Vector3),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "tile_name",
                    default: GodotString::from_str("").to_variant(),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });
        builder.add_signal(Signal {
            name: BuildingsSignal::Destroyed.as_ref(),
            args: &[SignalArgument {
                name: "coordinate",
                default: Coordinate::default().to_variant(),
                export_info: ExportInfo::new(VariantType::Vector3),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) {
        godot_print!("setting up clock");
        let emitter = &mut owner.get_node("/root/Game").unwrap();
        let emitter = unsafe { emitter.assume_safe() };
        emitter
            .connect(
                GameSignal::GameStart.as_ref(),
                owner,
                "_attach_game",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
    }

    #[export]
    fn _attach_game(&mut self, owner: TRef<Node>) {
        godot_print!("attaching clock to game now");
        let game = GameController::game().expect("game should be here");
        let buildings_observer = BuildingsObserver::new(&game.map().buildings, owner.claim());
        self.buildings_observer.replace(buildings_observer);
    }

    #[export]
    fn tick(&self, _owner: &Node) {
        if let Some(game) = GameController::game() {
            game.clock().tick();
        }
    }
}
