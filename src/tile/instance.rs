use crate::tile::{TileInstance, Tiles, State, Tile, SomeTileInstance};
use std::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard};

pub struct DefaultInstance {
    tile: Tiles,
    state: RwLock<Option<State>>,
}

impl DefaultInstance {
    pub fn new(tile: Tiles, state: Option<State>) -> Self {
        DefaultInstance {
            tile,
            state: RwLock::new(state),
        }
    }

    pub fn from(tile: &impl Tile) -> SomeTileInstance {
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

    fn state(&self) -> RwLockReadGuard<Option<State>> {
        self.state.read().expect("could not acquire read lock on tile instance")
    }

    fn state_mut(&self) -> RwLockWriteGuard<Option<State>> {
        self.state.write().expect("could not acquire read lock on tile instance")
    }

    fn update(&self) {
        unimplemented!()
    }
}



