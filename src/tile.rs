use std::collections::HashMap;
use crate::good::Inventory;
use crate::tile::pioneer::Pioneer;
use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;

mod pioneer;
mod instance;

pub struct Consumes(Inventory);

pub struct Produces(Inventory);

pub struct Costs(Inventory);

pub struct State(Inventory);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tiles {
    Pioneer
}

pub trait Tile {
    fn tile(&self) -> &Tiles;
    fn consumes(&self) -> &Consumes;
    fn influence_at(&self, at: &Coordinate) -> Range;
    fn influence(&self) -> Range {
        self.influence_at(&Default::default())
    }
    fn create(&self) -> Box<dyn TileInstance>;
}


pub trait TileInstance {
    fn tile(&self) -> &Tiles;
    fn state(&self) -> &State;
    fn update(&mut self);
}

pub struct TileFactory {
    tiles: HashMap<Tiles, Box<dyn Tile>>,
}

impl TileFactory {
    pub fn new() -> Self {
        let mut tiles: HashMap<Tiles, Box<dyn Tile>> = HashMap::new();
        tiles.insert(Tiles::Pioneer, Box::new(Pioneer::new()));
        TileFactory {
            tiles,
        }
    }

    pub fn create(&self, tile: Tiles) -> Box<dyn TileInstance> {
        self.tiles.get(&tile).unwrap().create()
    }

    pub fn tile(&self, tile: Tiles) -> &dyn Tile {
        self.tiles.get(&tile).unwrap().as_ref()
    }
}
