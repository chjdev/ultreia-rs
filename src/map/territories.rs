use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
pub use crate::map::buildings::territories_state::{TerritoriesState, TerritoriesStateRw};
use crate::map::minimap::{FillByCoordinate, GetByCoordinate, Minimap, SetByCoordinate, WithGrid};
use crate::observable::{Observable, Observers};

use self::territories_storage::TerritoriesStorage;
pub use self::territories_storage::TerritoryID;

mod territories_storage;

#[derive(Default)]
pub struct Territories {
    territories: TerritoriesStorage,
    rows: usize,
    columns: usize,
    joiners: Observers<TerritoryJoined>,
    leavers: Observers<TerritoryLeft>,
}

impl Territories {
    pub fn new(rows: usize, columns: usize) -> Self {
        Territories {
            territories: Default::default(),
            rows,
            columns,
            joiners: Default::default(),
            leavers: Default::default(),
        }
    }

    pub fn extend(&mut self, territory_id: &TerritoryID, range: Range) {
        let filtered_range = range
            .into_iter()
            .filter(|coordinate| {
                let maybe_territory_id: Option<TerritoryID> = self.get(coordinate);
                maybe_territory_id.is_none()
            })
            .collect();
        self.fill(filtered_range, Some(*territory_id));
    }

    pub fn get_territory(&self, territory_id: &TerritoryID) -> Option<Range> {
        self.territories.get_range(territory_id).cloned()
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
            Some(territory_id) => {
                let maybe_old_territory_id = self.territories.insert(coordinate, territory_id);
                if let Some(old_territory_id) = maybe_old_territory_id {
                    self.notify_all(TerritoryLeft {
                        coordinate,
                        territory_id: old_territory_id,
                    });
                }
                self.notify_all(TerritoryJoined {
                    coordinate,
                    territory_id,
                });
            }
            None => {
                if let Some(old_territory_id) = self.territories.remove(&coordinate) {
                    self.notify_all(TerritoryLeft {
                        coordinate,
                        territory_id: old_territory_id,
                    });
                }
            }
        }
    }
}

impl FillByCoordinate<Option<TerritoryID>> for Territories {}

impl Minimap<Option<TerritoryID>> for Territories {}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TerritoryJoined {
    pub coordinate: Coordinate,
    pub territory_id: TerritoryID,
}

impl Observable<TerritoryJoined> for Territories {
    fn observers(&self) -> &Observers<TerritoryJoined> {
        &self.joiners
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TerritoryLeft {
    pub coordinate: Coordinate,
    pub territory_id: TerritoryID,
}

impl Observable<TerritoryLeft> for Territories {
    fn observers(&self) -> &Observers<TerritoryLeft> {
        &self.leavers
    }
}
