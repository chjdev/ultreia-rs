use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
use crate::good::{Good, Inventory, InventoryAmount};
use crate::map::Map;
use crate::tile::consumes::Consumes;
use crate::tile::costs::Costs;
use crate::tile::pioneer::Pioneer;
use crate::tile::produces::Produces;
use crate::tile::state::State;
use crate::tile::warehouse::Warehouse;
use std::collections::HashMap;
use std::sync::{LockResult, RwLockReadGuard, RwLockWriteGuard};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub mod consumes;
pub mod costs;
mod helpers;
mod instance;
mod pioneer;
pub mod produces;
pub mod state;
mod warehouse;

#[derive(Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum TileName {
    Pioneer,
    Warehouse,
}

pub type SomeTileInstance = Box<dyn TileInstance + Send + Sync>;

pub trait Tile {
    fn tile(&self) -> &TileName;
    fn costs(&self) -> Option<&Costs> {
        None
    }
    fn consumes(&self) -> Option<&Consumes> {
        None
    }
    fn produces(&self) -> Option<&Produces> {
        None
    }
    fn influence_at(&self, at: &Coordinate) -> Range;
    fn influence(&self) -> Range {
        self.influence_at(&Default::default())
    }
    fn create(&self) -> SomeTileInstance;
    fn allowed(&self, _at: &Coordinate, _map: &Map) -> bool {
        false
    }
}

pub trait TileInstance:
    std::ops::AddAssign<Inventory> + std::ops::AddAssign<(Good, InventoryAmount)>
{
    fn tile(&self) -> &TileName;
    fn state(&self) -> Option<LockResult<RwLockReadGuard<'_, State>>> {
        None
    }
    fn state_mut(&self) -> Option<LockResult<RwLockWriteGuard<'_, State>>> {
        None
    }
    fn update(&self);
}

pub type SomeTile = Box<dyn Tile + Send + Sync>;

pub struct TileFactory {
    tiles: HashMap<TileName, SomeTile>,
}

impl TileFactory {
    fn new() -> Self {
        let mut tiles: HashMap<TileName, SomeTile> = HashMap::new();
        // so we don't forget one, match has to be exhaustive
        for tile_name in TileName::iter() {
            let tile: SomeTile = match tile_name {
                TileName::Pioneer => Box::new(Pioneer::new()),
                TileName::Warehouse => Box::new(Warehouse::new()),
            };
            tiles.insert(tile_name, tile);
        }
        TileFactory { tiles }
    }

    pub fn create(&self, tile: TileName) -> SomeTileInstance {
        self.tiles.get(&tile).unwrap().create()
    }

    pub fn tile(&self, tile: TileName) -> &dyn Tile {
        self.tiles.get(&tile).unwrap().as_ref()
    }
}
