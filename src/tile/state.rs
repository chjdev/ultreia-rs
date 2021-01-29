use crate::good::{Good, Inventory, InventoryAmount};
use crate::tile::consumes::Consumes;
use crate::tile::costs::Costs;
use crate::tile::helpers::{add_assign, sub_assign};
use crate::tile::produces::Produces;
use std::cmp::Ordering;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct State(Inventory);

impl Deref for State {
    type Target = Inventory;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(
        maybe_consumes: Option<&Consumes>,
        maybe_produces: Option<&Produces>,
    ) -> Option<Self> {
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
}

impl FromIterator<(Good, InventoryAmount)> for State {
    fn from_iter<T: IntoIterator<Item = (Good, InventoryAmount)>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<Inventory>())
    }
}

impl<'a> FromIterator<&'a (Good, InventoryAmount)> for State {
    fn from_iter<T: IntoIterator<Item = &'a (Good, InventoryAmount)>>(iter: T) -> Self {
        Self(iter.into_iter().cloned().collect::<Inventory>())
    }
}

impl IntoIterator for State {
    type Item = (Good, InventoryAmount);
    type IntoIter = std::collections::hash_map::IntoIter<Good, InventoryAmount>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a State {
    type Item = (&'a Good, &'a InventoryAmount);
    type IntoIter = std::collections::hash_map::Iter<'a, Good, InventoryAmount>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut State {
    type Item = (&'a Good, &'a mut InventoryAmount);
    type IntoIter = std::collections::hash_map::IterMut<'a, Good, InventoryAmount>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::AddAssign<(&Good, &InventoryAmount)> for State {
    fn add_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        add_assign(self, rhs)
    }
}

impl std::ops::AddAssign<(&Good, &InventoryAmount)> for &mut State {
    fn add_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        add_assign(*self, rhs)
    }
}

impl std::ops::SubAssign<(&Good, &InventoryAmount)> for State {
    fn sub_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        sub_assign(self, rhs)
    }
}

impl std::ops::SubAssign<(&Good, &InventoryAmount)> for &mut State {
    fn sub_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        sub_assign(*self, rhs)
    }
}

impl std::ops::AddAssign<&Inventory> for State {
    fn add_assign(&mut self, rhs: &Inventory) {
        for tuple in rhs {
            *self += tuple;
        }
    }
}

impl std::ops::AddAssign<&Inventory> for &mut State {
    fn add_assign(&mut self, rhs: &Inventory) {
        for tuple in rhs {
            *self += tuple;
        }
    }
}

impl std::ops::SubAssign<&Inventory> for State {
    fn sub_assign(&mut self, rhs: &Inventory) {
        for tuple in rhs {
            *self -= tuple;
        }
    }
}

impl std::ops::SubAssign<&Inventory> for &mut State {
    fn sub_assign(&mut self, rhs: &Inventory) {
        for tuple in rhs {
            *self -= tuple;
        }
    }
}

impl std::ops::AddAssign<&State> for State {
    fn add_assign(&mut self, rhs: &State) {
        *self += &**rhs
    }
}

impl std::ops::AddAssign<&State> for &mut State {
    fn add_assign(&mut self, rhs: &State) {
        *self += &**rhs
    }
}

impl std::ops::SubAssign<&State> for State {
    fn sub_assign(&mut self, rhs: &State) {
        *self -= &**rhs
    }
}

impl std::ops::SubAssign<&State> for &mut State {
    fn sub_assign(&mut self, rhs: &State) {
        *self -= &**rhs
    }
}

impl std::ops::SubAssign<&Costs> for State {
    fn sub_assign(&mut self, rhs: &Costs) {
        *self -= &**rhs
    }
}

impl std::ops::SubAssign<&Costs> for &mut State {
    fn sub_assign(&mut self, rhs: &Costs) {
        *self -= &**rhs
    }
}

impl PartialEq<Costs> for State {
    fn eq(&self, other: &Costs) -> bool {
        **self == **other
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
        Some(if is_less {
            Ordering::Less
        } else {
            Ordering::Equal
        })
    }
}

impl PartialEq<&Costs> for State {
    fn eq(&self, other: &&Costs) -> bool {
        self.eq(*other)
    }
}

impl PartialOrd<&Costs> for State {
    fn partial_cmp(&self, other: &&Costs) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}
