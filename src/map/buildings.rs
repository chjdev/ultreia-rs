use crate::coordinate::indexed::CoordinateIndexed;
use crate::coordinate::Coordinate;
use crate::map::minimap::{FillClonedByCoordinate, GetByCoordinate, SetByCoordinate, WithGrid};
use crate::tile::SomeTileInstance;
use std::sync::RwLock;

#[derive(Default)]
pub struct Buildings {
    buildings: RwLock<CoordinateIndexed<SomeTileInstance>>,
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

// impl GetByCoordinate<Option<&SomeTileInstance>> for Buildings {
//     fn get(&self, coordinate: &Coordinate) -> Option<&SomeTileInstance> {
//         self.buildings.read().unwrap().get(coordinate)
//     }
// }
//
// impl SetByCoordinate<Option<SomeTileInstance>> for Buildings {
//     fn set(&self, coordinate: Coordinate, value: Option<SomeTileInstance>) {
//         unimplemented!()
//     }
// }

// impl FillClonedByCoordinate<Option<SomeTileInstance>> for Buildings {}
