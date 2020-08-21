use crate::coordinate::Coordinate;
use crate::coordinate::range::Range;
use crate::tile::{Consumes, State, Tile, TileInstance, Tiles};
use crate::tile::instance::DefaultInstance;

pub struct Pioneer {
    tile: Tiles,
    consumes: Consumes,
}

impl Pioneer {
    pub fn new() -> Self {
        Pioneer {
            tile: Tiles::Pioneer,
            consumes: Consumes(Default::default()),
        }
    }
}

impl Tile for Pioneer {
    fn tile(&self) -> &Tiles {
        &self.tile
    }

    fn consumes(&self) -> &Consumes {
        &self.consumes
    }

    fn influence_at(&self, at: &Coordinate) -> Range {
        Range::circle(at, 2)
    }

    fn create(&self) -> Box<dyn TileInstance> {
        Box::new(DefaultInstance::new(
            self.tile,
            State(Default::default()),
        ))
    }
}
