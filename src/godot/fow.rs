mod fow_signal;

use gdnative::prelude::*;

use crate::coordinate::range::RangeFrom;
use crate::coordinate::Coordinate;
use crate::godot::fow::fow_signal::{FOWEvents, FOWSignal};
use crate::godot::game::GameSignal;
use crate::godot::game_controller::GameController;
use crate::map::minimap::{FillByCoordinate, SetByCoordinate};
use crate::map::minimap::{GetByCoordinate, Minimap};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct FOW {
    fow_events: Option<FOWEvents>,
}

impl FOW {
    fn new(_owner: &Node) -> Self {
        FOW { fow_events: None }
    }
}

#[methods]
impl FOW {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: FOWSignal::Uncover.as_ref(),
            args: &[SignalArgument {
                name: "coordinate",
                default: vec![Coordinate::default()].to_variant(),
                export_info: ExportInfo::new(VariantType::Vector3Array),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) {
        godot_print!("setting up fow");
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
        let fow_events = FOWEvents::new(game.map().fow(), owner.claim());
        self.fow_events = Some(fow_events);
    }

    #[export]
    fn at(&self, _owner: &Node, coordinate: Coordinate) -> Option<bool> {
        Some(GameController::game()?.map().fow().get(&coordinate))
    }

    #[export]
    fn uncover(
        &self,
        _owner: &Node,
        coordinate: Coordinate,
        maybe_radius: Option<u16>,
    ) -> Option<()> {
        let radius = maybe_radius.unwrap_or(1);
        match radius {
            0 => None,
            1 => Some(GameController::game()?.map().fow().set(coordinate, true)),
            some_radius => Some(
                GameController::game()?
                    .map()
                    .fow()
                    .fill(&coordinate.circle(some_radius), true),
            ),
        }
    }

    #[export]
    fn minimap(&self, _owner: &Node, width: u16, height: u16) -> Option<Vec<bool>> {
        Some(GameController::game()?.map().fow().minimap(width, height))
    }
}
