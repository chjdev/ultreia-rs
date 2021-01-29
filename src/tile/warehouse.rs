use crate::coordinate::range::Range;
use crate::coordinate::range::RangeFactory;
use crate::coordinate::Coordinate;
use crate::good::{BuildingMaterial, Good, InventoryAmount, ProductionGood, Weapon};
use crate::map::terrain::{Terrain, TerrainType};
use crate::map::territory::Territory;
use crate::tile::instance::DefaultInstance;
use crate::tile::{Consumes, SomeTileInstance, Tile, Tiles};
use std::iter::FromIterator;
use strum::IntoEnumIterator;

pub struct Warehouse {
    tile: Tiles,
    consumes: Consumes,
}

impl Warehouse {
    pub fn new() -> Self {
        let mut pairs = vec![];
        let production_goods: Vec<(Good, InventoryAmount)> = ProductionGood::iter()
            .map(|g| (Good::ProductionGood(g), 100))
            .collect();
        pairs.extend(production_goods);
        let weapons: Vec<(Good, InventoryAmount)> =
            Weapon::iter().map(|g| (Good::Weapon(g), 100)).collect();
        pairs.extend(weapons);
        let building_materials: Vec<(Good, InventoryAmount)> = BuildingMaterial::iter()
            .map(|g| (Good::BuildingMaterial(g), 100))
            .collect();
        pairs.extend(building_materials);
        Warehouse {
            tile: Tiles::Warehouse,
            consumes: Consumes::from_iter(pairs),
        }
    }
}

impl Tile for Warehouse {
    fn tile(&self) -> &Tiles {
        &self.tile
    }

    fn consumes(&self) -> Option<&Consumes> {
        Some(&self.consumes)
    }

    fn influence_at(&self, at: &Coordinate) -> Range {
        Range::circle(at, 6)
    }

    fn create(&self) -> SomeTileInstance {
        DefaultInstance::from(self)
    }

    fn allowed(&self, at: &Coordinate, terrain: &Terrain, _territory: Option<&Territory>) -> bool {
        let terrain_tile = terrain.get(at);
        terrain_tile.terrain_type() == TerrainType::Grassland
    }
}
