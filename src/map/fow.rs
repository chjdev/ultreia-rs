use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
use crate::map::minimap::{FillByCoordinate, GetByCoordinate, Minimap, SetByCoordinate, WithGrid};
use crate::observable::{Observable, Observers};
use derive_more::{Constructor, From, Into};

#[derive(Default)]
pub struct FOW {
    fow: Range,
    rows: usize,
    columns: usize,
    observers: Observers<Uncover>,
}

impl FOW {
    pub fn new(rows: usize, columns: usize) -> Self {
        FOW {
            fow: Default::default(),
            rows,
            columns,
            observers: Observers::new(),
        }
    }

    fn set_silent(&mut self, coordinate: Coordinate, value: bool) {
        if value {
            self.fow.insert(coordinate);
        } else {
            self.fow.remove(&coordinate);
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

impl SetByCoordinate<bool> for FOW {
    fn set(&mut self, coordinate: Coordinate, value: bool) {
        self.set_silent(coordinate, value);
        let uncover = Uncover::new(vec![coordinate]);
        self.notify_all(uncover)
    }
}

impl FillByCoordinate<bool> for FOW {
    fn fill(&mut self, range: Range, value: bool) {
        let uncover = Uncover::new(range.iter().copied().collect());
        range.into_iter().for_each(|c| {
            self.set_silent(c, value);
        });
        self.notify_all(uncover)
    }
}

impl Minimap<bool> for FOW {}

#[derive(Default, Clone, PartialEq, Eq, From, Into, Constructor)]
pub struct Uncover(Vec<Coordinate>);

impl Uncover {
    pub fn coordinates(&self) -> &Vec<Coordinate> {
        &self.0
    }
}

impl Observable<Uncover> for FOW {
    fn observers(&self) -> &Observers<Uncover> {
        &self.observers
    }
}
