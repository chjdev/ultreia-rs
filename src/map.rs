use crate::map::terrain::Terrain;
use crate::map::road::RoadNetwork;
use crate::map::territory::Territories;
use std::cell::{RefCell, RefMut, Ref};
use crate::map::tiles::TileMap;

pub mod unit;
pub mod road;
pub mod terrain;
pub mod territory;
pub mod tiles;

pub struct Map {
    rows: usize,
    columns: usize,
    terrain: Terrain,
    territories: Territories,
    roads: RoadNetwork,
    tiles: RefCell<TileMap>,
}

impl Map {
    pub fn new(rows: usize, columns: usize) -> Self {
        Map {
            rows,
            columns,
            terrain: Terrain::new(rows, columns),
            territories: Default::default(),
            roads: Default::default(),
            tiles: RefCell::new(Default::default()),
        }
    }

    pub fn tiles(&self) -> Ref<'_, TileMap> {
        self.tiles.borrow()
    }

    pub fn tiles_mut(&self) -> RefMut<'_, TileMap> {
        self.tiles.borrow_mut()
    }

    pub fn terrain(&self) -> &Terrain {
        &self.terrain
    }
}
