use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
use crate::good::{Good, Inventory, InventoryAmount};
use crate::map::terrain::Terrain;
use crate::map::territory::Territory;
use crate::tile::consumes::Consumes;
use crate::tile::costs::Costs;
use crate::tile::pioneer::Pioneer;
use crate::tile::produces::Produces;
use crate::tile::state::State;
use crate::tile::warehouse::Warehouse;
use std::collections::HashMap;
use std::sync::{LockResult, RwLockReadGuard, RwLockWriteGuard};

pub mod consumes;
pub mod costs;
mod helpers;
mod instance;
mod pioneer;
pub mod produces;
pub mod state;
mod warehouse;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Tiles {
    Pioneer,
    Warehouse,
}

pub type SomeTileInstance = Box<dyn TileInstance + Send + Sync>;

pub trait Tile {
    fn tile(&self) -> &Tiles;
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
    fn allowed(
        &self,
        _at: &Coordinate,
        _terrain: &Terrain,
        _territory: Option<&Territory>,
    ) -> bool {
        false
    }
}

pub trait TileInstance:
    std::ops::AddAssign<Inventory> + std::ops::AddAssign<(Good, InventoryAmount)>
{
    fn tile(&self) -> &Tiles;
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
    tiles: HashMap<Tiles, SomeTile>,
}

lazy_static! {
    static ref TILE_FACTORY_INSTANCE: TileFactory = TileFactory::new();
}

impl TileFactory {
    fn new() -> Self {
        let mut tiles: HashMap<Tiles, SomeTile> = HashMap::new();
        tiles.insert(Tiles::Pioneer, Box::new(Pioneer::new()));
        tiles.insert(Tiles::Warehouse, Box::new(Warehouse::new()));
        TileFactory { tiles }
    }

    pub fn instance() -> &'static Self {
        &TILE_FACTORY_INSTANCE
    }

    pub fn create(&self, tile: Tiles) -> SomeTileInstance {
        self.tiles.get(&tile).unwrap().create()
    }

    pub fn tile(&self, tile: Tiles) -> &dyn Tile {
        self.tiles.get(&tile).unwrap().as_ref()
    }
}
