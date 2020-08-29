use crate::map::ground::Ground;
use crate::map::road::RoadNetwork;
use crate::map::territory::Territories;
use crate::map::tiles::TileMap;

pub mod unit;
pub mod road;
pub mod ground;
pub mod territory;
pub mod tiles;

pub struct Map {
    ground: Ground,
    territories: Territories,
    roads: RoadNetwork,
    tiles: TileMap,
}

impl Map {
    pub fn new() -> Self {
        Map {
            ground: Ground::new(),
            territories: Default::default(),
            roads: Default::default(),
            tiles: Default::default(),
        }
    }

    pub fn tiles(&self) -> &TileMap {
        &self.tiles
    }

    pub fn tiles_mut(&mut self) -> &mut TileMap {
        &mut self.tiles
    }

    pub fn ground(&self) -> &Ground {
        &self.ground
    }
}
