use crate::coordinate::Coordinate;
use crate::coordinate::range::{Range, RangeFactory};
use crate::tile::{Consumes, Tile, Tiles, SomeTileInstance};
use crate::tile::instance::DefaultInstance;
use crate::good::ProductionGood::Fish;
use crate::good::Good::ProductionGood;

pub struct Pioneer {
    tile: Tiles,
    consumes: Consumes,
}

impl Pioneer {
    pub fn new() -> Self {
        Pioneer {
            tile: Tiles::Pioneer,
            consumes: Consumes::from(&[(ProductionGood(Fish), 3)]),
        }
    }
}

impl Tile for Pioneer {
    fn tile(&self) -> &Tiles {
        &self.tile
    }

    fn influence_at(&self, at: &Coordinate) -> Range {
        Range::circle(at, 2)
    }

    fn create(&self) -> SomeTileInstance {
        DefaultInstance::from(self)
    }

}
