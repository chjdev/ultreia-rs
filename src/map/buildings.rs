use crate::coordinate::indexed::CoordinateIndexed;
use crate::coordinate::Coordinate;
use crate::map::minimap::{FillClonedByCoordinate, GetByCoordinate, SetByCoordinate, WithGrid};
use crate::observable::{Observable, Observers};
use crate::tile::{SomeTileInstance, TileName};

#[derive(Default)]
pub struct Buildings {
    buildings: CoordinateIndexed<SomeTileInstance>,
    rows: usize,
    columns: usize,
    creators: Observers<BuildingCreated>,
    destroyers: Observers<BuildingDestroyed>,
}

impl Buildings {
    pub fn new(rows: usize, columns: usize) -> Self {
        Buildings {
            buildings: Default::default(),
            rows,
            columns,
            creators: Default::default(),
            destroyers: Default::default(),
        }
    }
}

impl WithGrid for Buildings {
    fn rows(&self) -> usize {
        self.rows
    }

    fn columns(&self) -> usize {
        self.columns
    }
}

impl GetByCoordinate<Option<SomeTileInstance>> for Buildings {
    fn get(&self, coordinate: &Coordinate) -> Option<SomeTileInstance> {
        self.buildings.get(coordinate).cloned()
    }
}

impl SetByCoordinate<Option<SomeTileInstance>> for Buildings {
    fn set(&mut self, coordinate: Coordinate, maybe_instance: Option<SomeTileInstance>) {
        match maybe_instance {
            Some(instance) => {
                let tile_name = *instance.tile();
                self.buildings.insert(coordinate, instance);
                self.notify_all(&BuildingCreated {
                    coordinate,
                    tile_name,
                });
            }
            None => {
                self.buildings.remove(&coordinate);
                self.notify_all(&BuildingDestroyed { coordinate });
            }
        };
    }
}

impl FillClonedByCoordinate<Option<SomeTileInstance>> for Buildings {}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BuildingCreated {
    pub coordinate: Coordinate,
    pub tile_name: TileName,
}

impl Observable<BuildingCreated> for Buildings {
    fn observers(&self) -> &Observers<BuildingCreated> {
        &self.creators
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BuildingDestroyed {
    pub coordinate: Coordinate,
}

impl Observable<BuildingDestroyed> for Buildings {
    fn observers(&self) -> &Observers<BuildingDestroyed> {
        &self.destroyers
    }
}
