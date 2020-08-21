use crate::tile::{State, Tile, TileInstance, Tiles};

pub struct DefaultInstance {
    tile: Tiles,
    state: State,
}

impl DefaultInstance {
    pub fn new(tile: Tiles, state: State) -> Self {
        DefaultInstance {
            tile,
            state,
        }
    }
}

impl TileInstance for DefaultInstance {
    fn tile(&self) -> &Tiles {
        &self.tile
    }

    fn state(&self) -> &State {
        &self.state
    }

    fn update(&mut self) {
        unimplemented!()
    }
}


