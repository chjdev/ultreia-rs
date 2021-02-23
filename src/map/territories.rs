mod territories_storage;

use self::territories_storage::TerritoriesStorage;
pub use self::territories_storage::TerritoryID;
use crate::coordinate::Coordinate;
use crate::map::minimap::{FillByCoordinate, GetByCoordinate, Minimap, SetByCoordinate, WithGrid};
use std::sync::RwLock;

#[derive(Default)]
pub struct Territories {
    territories: RwLock<TerritoriesStorage>,
    rows: usize,
    columns: usize,
}

impl Territories {
    pub fn new(rows: usize, columns: usize) -> Self {
        Territories {
            territories: Default::default(),
            rows,
            columns,
        }
    }
}

impl WithGrid for Territories {
    fn rows(&self) -> usize {
        self.rows
    }

    fn columns(&self) -> usize {
        self.columns
    }
}

impl GetByCoordinate<Option<TerritoryID>> for Territories {
    fn get(&self, coordinate: &Coordinate) -> Option<TerritoryID> {
        self.territories
            .read()
            .unwrap()
            .get_territory_id(coordinate)
            .copied()
    }
}

impl SetByCoordinate<Option<TerritoryID>> for Territories {
    fn set(&self, coordinate: Coordinate, maybe_territory_id: Option<TerritoryID>) {
        let mut territories = self.territories.write().unwrap();
        match maybe_territory_id {
            Some(territory_id) => territories.insert(coordinate, territory_id),
            None => territories.remove(&coordinate),
        }
    }
}

impl FillByCoordinate<Option<TerritoryID>> for Territories {}

impl Minimap<Option<TerritoryID>> for Territories {}
