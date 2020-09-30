use crate::good::{Inventory, Good, InventoryAmount};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use crate::tile::helpers::{add_assign, sub_assign};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Consumes(Inventory);

impl Deref for Consumes {
    type Target = Inventory;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Consumes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Consumes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(pairs: &[(Good, InventoryAmount)]) -> Self {
        Self(Inventory::from_iter(pairs.into_iter().cloned()))
    }
}

impl FromIterator<(Good, InventoryAmount)> for Consumes {
    fn from_iter<T: IntoIterator<Item=(Good, InventoryAmount)>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<Inventory>())
    }
}

impl<'a> FromIterator<&'a (Good, InventoryAmount)> for Consumes {
    fn from_iter<T: IntoIterator<Item=&'a (Good, InventoryAmount)>>(iter: T) -> Self {
        Self(iter.into_iter().cloned().collect::<Inventory>())
    }
}

impl std::ops::AddAssign<(&Good, &InventoryAmount)> for Consumes {
    fn add_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        add_assign(self, rhs)
    }
}

impl std::ops::AddAssign<(&Good, &InventoryAmount)> for &mut Consumes {
    fn add_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        add_assign(*self, rhs)
    }
}

impl std::ops::SubAssign<(&Good, &InventoryAmount)> for Consumes {
    fn sub_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        sub_assign(self, rhs)
    }
}

impl std::ops::SubAssign<(&Good, &InventoryAmount)> for &mut Consumes {
    fn sub_assign(&mut self, rhs: (&Good, &InventoryAmount)) {
        sub_assign(*self, rhs)
    }
}