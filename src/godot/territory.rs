use gdnative::prelude::*;

use crate::coordinate::Coordinate;
use crate::godot::game_controller::GameController;
use crate::map::minimap::{GetByCoordinate, Minimap};
use crate::map::territories::TerritoryID;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Territory;

impl Territory {
    fn new(_owner: &Node) -> Self {
        Territory {}
    }
}

#[methods]
impl Territory {
    #[export]
    fn at(&self, _owner: &Node, coordinate: Coordinate) -> Option<TerritoryID> {
        GameController::game()?.map().territories().get(&coordinate)
    }

    #[export]
    fn minimap(&self, _owner: &Node, width: u16, height: u16) -> Option<Vec<Option<TerritoryID>>> {
        Some(
            GameController::game()?
                .map()
                .territories()
                .minimap(width, height),
        )
    }
}
