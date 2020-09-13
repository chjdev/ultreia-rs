use crate::tile::{TileInstance, Tiles, State, Tile};
use std::cell::{RefCell, Ref, RefMut};

pub struct DefaultInstance {
    tile: Tiles,
    state: RefCell<State>,
}

impl DefaultInstance {
    pub fn new(tile: Tiles, state: State) -> Self {
        DefaultInstance {
            tile,
            state: RefCell::new(state),
        }
    }

    pub fn from(tile: &impl Tile) -> Box<dyn TileInstance> {
        Box::new(DefaultInstance::new(
            *tile.tile(),
            State::from(tile.consumes(), tile.produces()),
        ))
    }
}

impl TileInstance for DefaultInstance {
    fn tile(&self) -> &Tiles {
        &self.tile
    }

    fn state(&self) -> Option<Ref<State>> {
        self.state.try_borrow().ok()
    }

    fn state_mut(&self) -> Option<RefMut<State>> {
        self.state.try_borrow_mut().ok()
    }

    fn update(&self) {
        unimplemented!()
    }
}



