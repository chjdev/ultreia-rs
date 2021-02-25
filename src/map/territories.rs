mod territories_storage;

use self::territories_storage::TerritoriesStorage;
pub use self::territories_storage::TerritoryID;
use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
use crate::map::minimap::{FillByCoordinate, GetByCoordinate, Minimap, SetByCoordinate, WithGrid};

#[derive(Default)]
pub struct Territories {
    territories: TerritoriesStorage,
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
        self.territories.get_territory_id(coordinate).copied()
    }
}

impl GetByCoordinate<Option<Range>> for Territories {
    fn get(&self, coordinate: &Coordinate) -> Option<Range> {
        self.territories
            .get_territory_id(coordinate)
            .and_then(|territory_id| self.territories.get_range(territory_id).cloned())
    }
}

impl SetByCoordinate<Option<TerritoryID>> for Territories {
    fn set(&mut self, coordinate: Coordinate, maybe_territory_id: Option<TerritoryID>) {
        match maybe_territory_id {
            Some(territory_id) => self.territories.insert(coordinate, territory_id),
            None => self.territories.remove(&coordinate),
        }
    }
}

impl FillByCoordinate<Option<TerritoryID>> for Territories {}

impl Minimap<Option<TerritoryID>> for Territories {}
