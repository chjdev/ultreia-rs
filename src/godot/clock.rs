mod clock_signal;

use gdnative::prelude::*;

use crate::godot::clock::clock_signal::ClockEvents;
pub use crate::godot::clock::clock_signal::ClockSignal;
use crate::godot::game::GameSignal;
use crate::godot::game_instance::GameController;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct Clock {
    clock_events: Option<ClockEvents>,
}

impl Clock {
    fn new(_onwer: &Node) -> Self {
        Clock { clock_events: None }
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
        let clock_events = ClockEvents::new(game.clock(), owner.claim());
        self.clock_events = Some(clock_events);
    }

    #[export]
    fn tick(&self, _owner: &Node) {
        if let Some(game) = GameController::game() {
            game.clock().tick();
        }
    }
}
