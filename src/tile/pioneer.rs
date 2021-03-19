use crate::coordinate::range::{Range, RangeFrom};
use crate::coordinate::Coordinate;
use crate::good::Good;
use crate::map::MapStorage;
use crate::tile::{Consumes, Tile, TileName};
use std::iter::FromIterator;

pub struct Pioneer {
    name: TileName,
    consumes: Consumes,
}

impl Pioneer {
    pub fn new() -> Self {
        Pioneer {
            name: TileName::Pioneer,
            consumes: Consumes::from_iter(vec![(Good::Fish(), 3)].into_iter()),
        }
    }
}

impl Tile for Pioneer {
    fn name(&self) -> &TileName {
        &self.name
    }

    fn allowed(&self, _at: &Coordinate, _map: &MapStorage) -> bool {
        false
    }

    fn influence_at(&self, at: &Coordinate) -> Range {
        at.circle(2)
    }
}
