use crate::good::{Good, Inventory, InventoryAmount};
use crate::tile::helpers::{add_assign, sub_assign};
use derive_more::{Deref, DerefMut, From, Index, IndexMut, Into};
use std::iter::FromIterator;

#[derive(Default, Clone, PartialEq, Eq, From, Into, Deref, DerefMut, Index, IndexMut)]
pub struct Consumes(Inventory);

impl Consumes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(pairs: &[(Good, InventoryAmount)]) -> Self {
        Self(Inventory::from_iter(pairs.into_iter().cloned()))
    }
}

impl FromIterator<(Good, InventoryAmount)> for Consumes {
    fn from_iter<T: IntoIterator<Item = (Good, InventoryAmount)>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<Inventory>())
    }
}

impl<'a> FromIterator<&'a (Good, InventoryAmount)> for Consumes {
    fn from_iter<T: IntoIterator<Item = &'a (Good, InventoryAmount)>>(iter: T) -> Self {
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
