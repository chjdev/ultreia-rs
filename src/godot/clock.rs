mod clock_signal;

use gdnative::prelude::*;

use crate::godot::clock::clock_signal::ClockObserver;
pub use crate::godot::clock::clock_signal::ClockSignal;
use crate::godot::game::GameSignal;
use crate::godot::game_controller::GameController;
use std::sync::Arc;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct Clock {
    clock_observer: Option<Arc<ClockObserver>>,
}

impl Clock {
    fn new(_onwer: &Node) -> Self {
        Clock {
            clock_observer: None,
        }
    }
}

#[methods]
impl Clock {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: ClockSignal::Tick.as_ref(),
            args: &[SignalArgument {
                name: "epoch",
                default: Variant::from_i64(0),
                export_info: ExportInfo::new(VariantType::I64),
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: ClockSignal::Tock.as_ref(),
            args: &[SignalArgument {
                name: "epoch",
                default: Variant::from_i64(0),
                export_info: ExportInfo::new(VariantType::I64),
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
        let clock_observer = ClockObserver::new(game.clock(), owner.claim());
        self.clock_observer.replace(clock_observer);
    }

    #[export]
    fn tick(&self, _owner: &Node) -> Option<()> {
        Some(GameController::game()?.clock().tick())
    }
}
