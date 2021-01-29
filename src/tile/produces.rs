use crate::good::{Good, Inventory, InventoryAmount};
use crate::tile::helpers::{add_assign, sub_assign};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Produces(Inventory);

impl Deref for Produces {
    type Target = Inventory;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Produces {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Produces {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(pairs: &[(Good, InventoryAmount)]) -> Self {
        Self(Inventory::from_iter(pairs.into_iter().cloned()))
    }
}

impl FromIterator<(Good, InventoryAmount)> for Produces {
    fn from_iter<T: IntoIterator<Item = (Good, InventoryAmount)>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<Inventory>())
    }
}

impl<'a> FromIterator<&'a (Good, InventoryAmount)> for Produces {
    fn from_iter<T: IntoIterator<Item = &'a (Good, InventoryAmount)>>(iter: T) -> Self {
        Self(iter.into_iter().cloned().collect::<Inventory>())
    }
}

impl std::ops::AddAssign<(&Good, &InventoryAmount)> for Produces {
    fn add_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        add_assign(self, rhs)
    }
}

impl std::ops::AddAssign<(&Good, &InventoryAmount)> for &mut Produces {
    fn add_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        add_assign(*self, rhs)
    }
}

impl std::ops::SubAssign<(&Good, &InventoryAmount)> for Produces {
    fn sub_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        sub_assign(self, rhs)
    }
}

impl std::ops::SubAssign<(&Good, &InventoryAmount)> for &mut Produces {
    fn sub_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        sub_assign(*self, rhs)
    }
}
