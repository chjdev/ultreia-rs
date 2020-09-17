use crate::map::terrain::Terrain;
use crate::map::road::RoadNetwork;
use crate::map::territory::Territory;
use crate::coordinate::Coordinate;
use crate::tile::{Tiles, TileFactory};
use std::sync::{Arc, Weak};
use crate::coordinate::range::Range;

pub mod unit;
pub mod road;
pub mod terrain;
pub mod territory;

pub struct Map {
    rows: usize,
    columns: usize,
    terrain: Arc<Terrain>,
    territories: Vec<Territory>,
    roads: RoadNetwork,
    tile_factory: Weak<TileFactory>,
}

impl Map {
    pub fn new(rows: usize, columns: usize, tile_factory: Weak<TileFactory>) -> Self {
        Map {
            rows,
            columns,
            terrain: Arc::new(Terrain::new(rows, columns)),
            territories: Default::default(),
            roads: Default::default(),
            tile_factory,
        }
    }

    pub fn terrain(&self) -> &Terrain {
        &self.terrain
    }

    pub fn territories(&self) -> &Vec<Territory> {
        &self.territories
    }

    pub fn territory_for_coordinate(&self, coordinate: &Coordinate) -> Option<&Territory> {
        // todo??
        for territory in &self.territories {
            if territory.contains(coordinate) {
                return Some(territory);
            }
        }
        None
    }

    pub fn can_construct(&self, at: &Coordinate, tile: Tiles) -> bool {
        return if let Some(territory) = self.territory_for_coordinate(&at) {
            territory.can_construct(&at, tile)
        } else {
            if tile == Tiles::Warehouse && self.territories.is_empty() {
                return if let Some(tile_factory) = self.tile_factory.upgrade() {
                    tile_factory.tile(Tiles::Warehouse).allowed(at, self.terrain(), None)
                } else {
                    false
                };
            }
            false
        };
    }

    pub fn can_construct_range(&self, range: &Range, tile: Tiles) -> Vec<bool> {
        range.iter().map(|coordinate| self.can_construct(coordinate, tile)).collect()
    }
}
