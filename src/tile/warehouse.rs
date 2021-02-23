use crate::coordinate::range::{Range, RangeFrom};
use crate::coordinate::Coordinate;
use crate::good::{BuildingMaterial, Good, InventoryAmount, ProductionGood, Weapon};
use crate::map::minimap::GetByCoordinate;
use crate::map::terrain::TerrainType;
use crate::map::Map;
use crate::tile::{Consumes, Tile, TileName};
use std::iter::FromIterator;
use strum::IntoEnumIterator;

pub struct Warehouse {
    tile: TileName,
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
            tile: TileName::Warehouse,
            consumes: Consumes::from_iter(pairs),
        }
    }
}

impl Tile for Warehouse {
    fn tile(&self) -> &TileName {
        &self.tile
    }

    fn consumes(&self) -> Option<&Consumes> {
        Some(&self.consumes)
    }

    fn influence_at(&self, at: &Coordinate) -> Range {
        at.circle(6)
    }

    fn allowed(&self, at: &Coordinate, map: &Map) -> bool {
        let terrain_tile: TerrainType = map.terrain().get(at);
        terrain_tile == TerrainType::Grassland
    }
}
