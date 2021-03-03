use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString, EnumVariantNames};

use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
use crate::good::costs::Costs;
use crate::map::Map;
use crate::tile::consumes::Consumes;
use crate::tile::pioneer::Pioneer;
use crate::tile::produces::Produces;
use crate::tile::state::State;
use crate::tile::warehouse::Warehouse;

pub mod consumes;
mod pioneer;
pub mod produces;
pub mod state;
mod warehouse;

#[derive(Copy, Clone, PartialEq, Eq, Hash, EnumIter, AsRefStr, EnumString, EnumVariantNames)]
pub enum TileName {
    Pioneer,
    Warehouse,
}

impl Default for TileName {
    fn default() -> Self {
        TileName::Warehouse
    }
}

pub type SomeTile = Box<dyn Tile>;

pub trait Tile: Send + Sync {
    fn name(&self) -> &TileName;
    fn costs(&self) -> Option<&Costs> {
        None
    }
    fn consumes(&self) -> Option<&Consumes> {
        None
    }
    fn produces(&self) -> Option<&Produces> {
        None
    }
    fn allowed(&self, _at: &Coordinate, _map: &Map) -> bool {
        false
    }
    fn influence_at(&self, at: &Coordinate) -> Range;
    fn influence(&self) -> Range {
        self.influence_at(&Default::default())
    }
    fn update(&self, instance: &mut TileInstance);
}

pub struct TileInstance {
    tile: &'static dyn Tile,
    state: Option<State>,
}

impl TileInstance {
    fn new(tile: &'static dyn Tile, state: Option<State>) -> Self {
        TileInstance { tile, state }
    }

    pub fn from(tile: &'static dyn Tile) -> Self {
        TileInstance::new(tile, State::from(tile.consumes(), tile.produces()))
    }

    pub fn tile(&self) -> &'static dyn Tile {
        self.tile
    }

    pub fn state(&self) -> Option<&State> {
        self.state.as_ref()
    }
    pub fn state_mut(&mut self) -> Option<&mut State> {
        self.state.as_mut()
    }
}

pub struct TileFactory {
    tiles: HashMap<TileName, &'static dyn Tile>,
}

lazy_static! {
    static ref _pioneer: Pioneer = Pioneer::new();
    static ref _warehouse: Warehouse = Warehouse::new();
}

impl TileFactory {
    pub fn new() -> Self {
        let mut tiles: HashMap<TileName, &'static dyn Tile> = HashMap::new();
        // so we don't forget one, match has to be exhaustive
        for tile_name in TileName::iter() {
            let tile: &'static dyn Tile = match tile_name {
                TileName::Pioneer => &*_pioneer,
                TileName::Warehouse => &*_warehouse,
            };
            tiles.insert(tile_name, tile);
        }
        TileFactory { tiles }
    }

    pub fn create(&self, tile_name: &TileName) -> TileInstance {
        TileInstance::from(self.tile(tile_name))
    }

    pub fn tile(&self, tile_name: &TileName) -> &'static dyn Tile {
        *self.tiles.get(tile_name).unwrap()
    }
}
