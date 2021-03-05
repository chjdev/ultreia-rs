use crate::coordinate::range::{Range, RangeFrom};
use crate::coordinate::Coordinate;
use crate::good::Good::ProductionGood;
use crate::good::ProductionGood::Fish;
use crate::tile::{Consumes, Tile, TileName};

pub struct Pioneer {
    name: TileName,
    consumes: Consumes,
}

impl Pioneer {
    pub fn new() -> Self {
        Pioneer {
            name: TileName::Pioneer,
            consumes: Consumes::from(&[(ProductionGood(Fish), 3)]),
        }
    }
}

impl Tile for Pioneer {
    fn name(&self) -> &TileName {
        &self.name
    }

    fn influence_at(&self, at: &Coordinate) -> Range {
        at.circle(2)
    }
}
