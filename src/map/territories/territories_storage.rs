use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
use derive_more::{Constructor, From, Into};
use std::collections::HashMap;

#[derive(Default, Hash, Clone, Copy, PartialEq, Eq, Constructor, From, Into)]
pub struct TerritoryID(usize);

#[derive(Default)]
pub struct TerritoriesStorage {
    by_coordinate: HashMap<Coordinate, TerritoryID>,
    by_territory_id: HashMap<TerritoryID, Range>,
}

impl TerritoriesStorage {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(
        &mut self,
        coordinate: Coordinate,
        territory_id: TerritoryID,
    ) -> Option<TerritoryID> {
        if !self.by_territory_id.contains_key(&territory_id) {
            self.by_territory_id.insert(territory_id, Range::new());
        }
        self.by_territory_id
            .get_mut(&territory_id)
            .unwrap()
            .insert(coordinate);
        let maybe_old_territory_id = self.by_coordinate.insert(coordinate, territory_id);
        if let Some(old_territory_id) = maybe_old_territory_id {
            if let Some(old_range) = self.by_territory_id.get_mut(&old_territory_id) {
                old_range.remove(&coordinate);
            }
        }
        maybe_old_territory_id
    }

    pub fn remove(&mut self, coordinate: &Coordinate) -> Option<TerritoryID> {
        let maybe_territory_id = self.by_coordinate.remove(coordinate);
        if let Some(territory_id) = maybe_territory_id {
            if self.by_territory_id.get(&territory_id).unwrap().len() <= 1 {
                self.by_territory_id.remove(&territory_id);
            } else {
                self.by_territory_id
                    .get_mut(&territory_id)
                    .unwrap()
                    .remove(coordinate);
            }
        }
        maybe_territory_id
    }

    pub fn remove_territory(&mut self, territory_id: &TerritoryID) -> Option<Range> {
        let maybe_range = self.by_territory_id.remove(territory_id);
        if let Some(range) = &maybe_range {
            range.iter().for_each(|coordinate| {
                self.by_coordinate.remove(coordinate);
            });
        }
        maybe_range
    }

    pub fn get_range(&self, territory_id: &TerritoryID) -> Option<&Range> {
        self.by_territory_id.get(territory_id)
    }

    pub fn get_territory_id(&self, coordinate: &Coordinate) -> Option<&TerritoryID> {
        self.by_coordinate.get(coordinate)
    }
}
