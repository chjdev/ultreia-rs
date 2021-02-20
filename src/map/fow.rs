use crate::coordinate::Coordinate;
use crate::map::minimap::{GetByCoordinate, Minimap, WithGrid};
use std::collections::HashSet;

type FOWStorage = HashSet<Coordinate>;

#[derive(Default)]
pub struct FOW {
    fow: FOWStorage,
    rows: usize,
    columns: usize,
}

impl FOW {
    pub fn new(rows: usize, columns: usize) -> Self {
        FOW {
            fow: Default::default(),
            rows,
            columns,
        }
    }
}

impl WithGrid for FOW {
    fn rows(&self) -> usize {
        self.rows
    }

    fn columns(&self) -> usize {
        self.columns
    }
}

impl GetByCoordinate<bool> for FOW {
    fn get(&self, coordinate: &Coordinate) -> bool {
        self.fow.contains(coordinate)
    }
}

impl Minimap<bool> for FOW {}
