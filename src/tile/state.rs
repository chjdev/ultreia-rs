use crate::good::{Inventory, Good};
use std::iter::FromIterator;
use crate::tile::consumes::Consumes;
use crate::tile::produces::Produces;
use crate::tile::costs::Costs;
use std::cmp::Ordering;
use std::collections::hash_map::Keys;

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
            for good in consumes.keys() {
                state.0.insert(*good, 0);
            }
        }
        if let Some(produces) = maybe_produces {
            for good in produces.keys() {
                state.0.insert(*good, 0);
            }
        }
        Some(state)
    }

    pub fn keys(&self) -> Keys<'_, Good, i32> {
        self.0.keys()
    }

    pub fn get(&self, key: &Good) -> Option<&i32> {
        self.0.get(key)
    }
}

impl FromIterator<(Good, i32)> for State {
    fn from_iter<T: IntoIterator<Item=(Good, i32)>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<Inventory>())
    }
}

impl<'a> FromIterator<&'a (Good, i32)> for State {
    fn from_iter<T: IntoIterator<Item=&'a (Good, i32)>>(iter: T) -> Self {
        Self(iter.into_iter().cloned().collect::<Inventory>())
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
        for key in other.keys() {
            if !self.0.contains_key(key) {
                return false;
            }
            if other.get(key).unwrap_or(&0) != self.0.get(key).unwrap_or(&0) {
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
        for key in other.keys() {
            if !self.0.contains_key(key) {
                return Some(Ordering::Greater);
            }
            let others = other.get(key).unwrap_or(&0);
            let mine = self.get(key).unwrap_or(&0);
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
