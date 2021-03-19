use crate::coordinate::range::{Range, RangeFrom};
use crate::coordinate::Coordinate;
use crate::good::costs::Costs;
use crate::good::{BuildingMaterial, Good, Inventory, InventoryAmount, ProductionGood, Weapon};
use crate::map::minimap::GetByCoordinate;
use crate::map::terrain::TerrainType;
use crate::map::MapStorage;
use crate::tile::{Consumes, Tile, TileName};
use std::iter::FromIterator;
use strum::IntoEnumIterator;

pub struct Warehouse {
    name: TileName,
    consumes: Consumes,
    costs: Costs,
}

impl Warehouse {
    pub fn new() -> Self {
        let mut pairs: Vec<<Inventory as InventoryAmount>::Entry> = vec![];
        pairs.extend(ProductionGood::iter().map(|g| (g.into(), 100)));
        pairs.extend(Weapon::iter().map(|g| (g.into(), 100)));
        pairs.extend(BuildingMaterial::iter().map(|g| (g.into(), 100)));
        pairs.push((Good::Money(), u32::max_value()));
        let inventory: Inventory = pairs.into_iter().collect();
        Warehouse {
            name: TileName::Warehouse,
            consumes: inventory.into(),
            costs: Costs::from_iter(vec![(Good::Money(), 10)]),
        }
    }
}

impl Tile for Warehouse {
    fn name(&self) -> &TileName {
        &self.name
    }

    fn costs(&self) -> Option<&Costs> {
        Some(&self.costs)
    }

    fn consumes(&self) -> Option<&Consumes> {
        Some(&self.consumes)
    }

    fn allowed(&self, at: &Coordinate, map: &MapStorage) -> bool {
        let terrain_tile: TerrainType = map.terrain.get(at);
        terrain_tile == TerrainType::Grassland
    }

    fn influence_at(&self, at: &Coordinate) -> Range {
        at.circle(6)
    }
}
