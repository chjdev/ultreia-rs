use crate::good::{Good, Inventory, InventoryAmount};
use crate::tile::helpers::{add_assign, sub_assign};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Costs(Inventory);

impl Deref for Costs {
    type Target = Inventory;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Costs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Costs {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(pairs: &[(Good, InventoryAmount)]) -> Self {
        Self(Inventory::from_iter(pairs.into_iter().cloned()))
    }
}

impl FromIterator<(Good, InventoryAmount)> for Costs {
    fn from_iter<T: IntoIterator<Item = (Good, InventoryAmount)>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<Inventory>())
    }
}

impl<'a> FromIterator<&'a (Good, InventoryAmount)> for Costs {
    fn from_iter<T: IntoIterator<Item = &'a (Good, InventoryAmount)>>(iter: T) -> Self {
        Self(iter.into_iter().cloned().collect::<Inventory>())
    }
}

impl<'a> IntoIterator for &'a Costs {
    type Item = (&'a Good, &'a InventoryAmount);
    type IntoIter = std::collections::hash_map::Iter<'a, Good, InventoryAmount>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Costs {
    type Item = (&'a Good, &'a mut InventoryAmount);
    type IntoIter = std::collections::hash_map::IterMut<'a, Good, InventoryAmount>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::AddAssign<(&Good, &InventoryAmount)> for Costs {
    fn add_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        add_assign(self, rhs)
    }
}

impl std::ops::AddAssign<(&Good, &InventoryAmount)> for &mut Costs {
    fn add_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        add_assign(*self, rhs)
    }
}

impl std::ops::SubAssign<(&Good, &InventoryAmount)> for Costs {
    fn sub_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        sub_assign(self, rhs)
    }
}

impl std::ops::SubAssign<(&Good, &InventoryAmount)> for &mut Costs {
    fn sub_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        sub_assign(*self, rhs)
    }
}
