use serde_repr::{Serialize_repr, Deserialize_repr};
use std::collections::HashMap;
use crate::good::{Inventory, Good};
use crate::tile::pioneer::Pioneer;
use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
use std::iter::FromIterator;
use crate::tile::warehouse::Warehouse;
use std::cmp::Ordering;
use crate::map::terrain::Terrain;
use crate::map::territory::Territory;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

mod pioneer;
mod instance;
mod warehouse;

#[derive(Default)]
pub struct Consumes(Inventory);

impl Consumes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(pairs: &[(Good, i32)]) -> Self {
        Self::from_iter(pairs.iter())
    }
}

impl FromIterator<(Good, i32)> for Consumes {
    fn from_iter<T: IntoIterator<Item=(Good, i32)>>(iter: T) -> Self {
        Consumes(iter.into_iter().collect::<Inventory>())
    }
}

impl<'a> FromIterator<&'a (Good, i32)> for Consumes {
    fn from_iter<T: IntoIterator<Item=&'a (Good, i32)>>(iter: T) -> Self {
        Consumes(iter.into_iter().cloned().collect::<Inventory>())
    }
}

#[derive(Default)]
pub struct Produces(Inventory);

impl Produces {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(pairs: &[(Good, i32)]) -> Self {
        Self::from_iter(pairs.iter())
    }
}

impl FromIterator<(Good, i32)> for Produces {
    fn from_iter<T: IntoIterator<Item=(Good, i32)>>(iter: T) -> Self {
        Produces(iter.into_iter().collect::<Inventory>())
    }
}

impl<'a> FromIterator<&'a (Good, i32)> for Produces {
    fn from_iter<T: IntoIterator<Item=&'a (Good, i32)>>(iter: T) -> Self {
        Produces(iter.into_iter().cloned().collect::<Inventory>())
    }
}

#[derive(Default)]
pub struct Costs(Inventory);

impl Costs {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(pairs: &[(Good, i32)]) -> Self {
        Self::from_iter(pairs.iter())
    }
}

impl FromIterator<(Good, i32)> for Costs {
    fn from_iter<T: IntoIterator<Item=(Good, i32)>>(iter: T) -> Self {
        Costs(iter.into_iter().collect::<Inventory>())
    }
}

impl<'a> FromIterator<&'a (Good, i32)> for Costs {
    fn from_iter<T: IntoIterator<Item=&'a (Good, i32)>>(iter: T) -> Self {
        Costs(iter.into_iter().cloned().collect::<Inventory>())
    }
}

impl<'a> IntoIterator for &'a Costs {
    type Item = (&'a Good, &'a i32);
    type IntoIter = std::collections::hash_map::Iter<'a, Good, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Costs {
    type Item = (&'a Good, &'a mut i32);
    type IntoIter = std::collections::hash_map::IterMut<'a, Good, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}


#[derive(Default)]
pub struct State(Inventory);

impl State {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(maybe_consumes: Option<&Consumes>, maybe_produces: Option<&Produces>) -> Option<Self> {
        if maybe_consumes.is_none() && maybe_produces.is_none() {
            return None;
        }
        let mut state = State(Default::default());
        if let Some(consumes) = maybe_consumes {
            for good in consumes.0.keys() {
                state.0.insert(*good, 0);
            }
        }
        if let Some(produces) = maybe_produces {
            for good in produces.0.keys() {
                state.0.insert(*good, 0);
            }
        }
        Some(state)
    }

    pub fn get(&self, good: &Good) -> &i32 {
        self.0.get(good).unwrap_or(&0)
    }
}

impl IntoIterator for State {
    type Item = (Good, i32);
    type IntoIter = std::collections::hash_map::IntoIter<Good, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a State {
    type Item = (&'a Good, &'a i32);
    type IntoIter = std::collections::hash_map::Iter<'a, Good, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut State {
    type Item = (&'a Good, &'a mut i32);
    type IntoIter = std::collections::hash_map::IterMut<'a, Good, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::AddAssign<(&Good, &i32)> for &mut State {
    fn add_assign(&mut self, rhs: (&Good, &i32)) {
        let (good, value) = rhs;
        let old = *self.0.get(good).unwrap_or(&0);
        self.0.insert(*good, *value + old);
    }
}

impl std::ops::AddAssign<&State> for &mut State {
    fn add_assign(&mut self, rhs: &State) {
        for tuple in rhs {
            *self += tuple;
        }
    }
}

impl std::ops::AddAssign<&State> for State {
    fn add_assign(&mut self, rhs: &State) {
        // wont let me borrow self as mut for some reason
        for (good, value) in rhs {
            let old = *self.0.get(good).unwrap_or(&0);
            self.0.insert(*good, *value + old);
        }
    }
}

impl std::ops::SubAssign<(&Good, &i32)> for State {
    fn sub_assign(&mut self, rhs: (&Good, &i32)) {
        let (good, value) = rhs;
        let old = *self.0.get(good).unwrap_or(&0);
        self.0.insert(*good, *value - old);
    }
}

impl std::ops::SubAssign<(&Good, &i32)> for &mut State {
    fn sub_assign(&mut self, rhs: (&Good, &i32)) {
        let (good, value) = rhs;
        let old = *self.0.get(good).unwrap_or(&0);
        self.0.insert(*good, *value - old);
    }
}


impl PartialEq<Costs> for State {
    fn eq(&self, other: &Costs) -> bool {
        for key in other.0.keys() {
            if !self.0.contains_key(key) {
                return false;
            }
            if other.0.get(key).unwrap_or(&0) != self.0.get(key).unwrap_or(&0) {
                return false;
            }
        }
        true
    }
}

impl PartialEq<&Costs> for State {
    fn eq(&self, other: &&Costs) -> bool {
        self.eq(*other)
    }
}

impl PartialOrd<Costs> for State {
    fn partial_cmp(&self, other: &Costs) -> Option<Ordering> {
        let mut is_less = false;
        for key in other.0.keys() {
            if !self.0.contains_key(key) {
                return Some(Ordering::Greater);
            }
            let others = other.0.get(key).unwrap_or(&0);
            let mine = self.0.get(key).unwrap_or(&0);
            if others > mine {
                return Some(Ordering::Greater);
            } else if others < mine {
                is_less = true;
            }
        }
        Some(if is_less { Ordering::Less } else { Ordering::Equal })
    }
}

impl PartialOrd<&Costs> for State {
    fn partial_cmp(&self, other: &&Costs) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Tiles {
    Pioneer,
    Warehouse,
}

pub type SomeTileInstance = Box<dyn TileInstance + Send + Sync>;

pub trait Tile {
    fn tile(&self) -> &Tiles;
    fn costs(&self) -> Option<&Costs> {
        None
    }
    fn consumes(&self) -> Option<&Consumes> {
        None
    }
    fn produces(&self) -> Option<&Produces> {
        None
    }
    fn influence_at(&self, at: &Coordinate) -> Range;
    fn influence(&self) -> Range {
        self.influence_at(&Default::default())
    }
    fn create(&self) -> SomeTileInstance;
    fn allowed(&self, _at: &Coordinate, _terrain: &Terrain, _territory: Option<&Territory>) -> bool {
        false
    }
}

pub trait TileInstance {
    fn tile(&self) -> &Tiles;
    fn state(&self) -> RwLockReadGuard<Option<State>>;
    fn state_mut(&self) -> RwLockWriteGuard<Option<State>>;
    fn update(&self);
}

pub type SomeTile = Box<dyn Tile + Send + Sync>;

pub struct TileFactory {
    tiles: HashMap<Tiles, SomeTile>,
}

impl TileFactory {
    pub fn new() -> Self {
        let mut tiles: HashMap<Tiles, SomeTile> = HashMap::new();
        tiles.insert(Tiles::Pioneer, Box::new(Pioneer::new()));
        tiles.insert(Tiles::Warehouse, Box::new(Warehouse::new()));
        TileFactory {
            tiles,
        }
    }

    pub fn create(&self, tile: Tiles) -> SomeTileInstance {
        self.tiles.get(&tile).unwrap().create()
    }

    pub fn tile(&self, tile: Tiles) -> &dyn Tile {
        self.tiles.get(&tile).unwrap().as_ref()
    }
}
