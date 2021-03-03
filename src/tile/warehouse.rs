use crate::coordinate::range::{Range, RangeFrom};
use crate::coordinate::Coordinate;
use crate::good::{BuildingMaterial, Good, Inventory, InventoryAmount, ProductionGood, Weapon};
use crate::map::minimap::GetByCoordinate;
use crate::map::terrain::TerrainType;
use crate::map::Map;
use crate::tile::{Consumes, Tile, TileInstance, TileName};
use strum::IntoEnumIterator;

pub struct Warehouse {
    tile: TileName,
    consumes: Consumes,
}

impl Warehouse {
    pub fn new() -> Self {
        let mut pairs: Vec<<Inventory as InventoryAmount>::Entry> = vec![];
        pairs.extend(ProductionGood::iter().map(|g| (Good::ProductionGood(g), 100)));
        pairs.extend(Weapon::iter().map(|g| (Good::Weapon(g), 100)));
        pairs.extend(BuildingMaterial::iter().map(|g| (Good::BuildingMaterial(g), 100)));
        let inventory: Inventory = pairs.into_iter().collect();
        Warehouse {
            tile: TileName::Warehouse,
            consumes: inventory.into(),
        }
    }
}

impl Tile for Warehouse {
    fn name(&self) -> &TileName {
        &self.tile
    }

    fn consumes(&self) -> Option<&Consumes> {
        Some(&self.consumes)
    }

    fn allowed(&self, at: &Coordinate, map: &Map) -> bool {
        let terrain_tile: TerrainType = map.terrain.get(at);
        terrain_tile == TerrainType::Grassland
    }

    fn influence_at(&self, at: &Coordinate) -> Range {
        at.circle(6)
    }

    fn update(&self, _instance: &mut TileInstance) {
        unimplemented!()
    }
}
