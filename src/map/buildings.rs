use crate::coordinate::indexed::CoordinateIndexed;
use crate::coordinate::Coordinate;
use crate::map::minimap::{FillClonedByCoordinate, GetByCoordinate, SetByCoordinate, WithGrid};
use crate::tile::SomeTileInstance;

#[derive(Default)]
pub struct Buildings {
    buildings: CoordinateIndexed<SomeTileInstance>,
    rows: usize,
    columns: usize,
}

impl Buildings {
    pub fn new(rows: usize, columns: usize) -> Self {
        Buildings {
            buildings: Default::default(),
            rows,
            columns,
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
            Some(instance) => self.buildings.insert(coordinate, instance),
            None => self.buildings.remove(&coordinate),
        };
    }
}

impl FillClonedByCoordinate<Option<SomeTileInstance>> for Buildings {}
