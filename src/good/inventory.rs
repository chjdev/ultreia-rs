use super::Good;
use derive_more::{Deref, DerefMut, From, Index, IndexMut, Into, IntoIterator};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(
    Default, Clone, PartialEq, Eq, From, Into, Deref, DerefMut, Index, IndexMut, IntoIterator,
)]
pub struct Inventory<T = u32>(HashMap<Good, T>);

pub trait InventoryAmount {
    type Amount;
    type Entry;
}

impl<T> InventoryAmount for Inventory<T> {
    type Amount = T;
    type Entry = (Good, Self::Amount);
}

impl Inventory {
    pub fn contains_key(&self, key: &Good) -> bool {
        self.0.contains_key(key)
    }
}

impl<T: Default> Inventory<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> FromIterator<<Self as InventoryAmount>::Entry> for Inventory<T> {
    fn from_iter<I: IntoIterator<Item = <Self as InventoryAmount>::Entry>>(iter: I) -> Self {
        Self(iter.into_iter().collect::<HashMap<Good, T>>())
    }
}

impl<T: AddAssign + Copy> AddAssign for Inventory<T> {
    fn add_assign(&mut self, rhs: Self) {
        for (good, amount) in rhs.iter() {
            let maybe_current = self.0.get_mut(good);
            if let Some(current) = maybe_current {
                current.add_assign(*amount);
            } else {
                self.0.insert(*good, *amount);
            }
        }
    }
}

impl<T: Add<Output = T> + Copy> Add for Inventory<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_inventory: Inventory<T> = self.0.into_iter().collect();
        for (good, amount) in rhs.iter() {
            let maybe_current = new_inventory.insert(*good, *amount);
            if let Some(current) = maybe_current {
                new_inventory.insert(*good, current.add(*amount));
            }
        }
        new_inventory
    }
}

impl<T: SubAssign + Copy> SubAssign for Inventory<T> {
    fn sub_assign(&mut self, rhs: Self) {
        for (good, amount) in rhs.iter() {
            let maybe_current = self.0.get_mut(good);
            if let Some(current) = maybe_current {
                current.sub_assign(*amount);
            } else {
                self.0.insert(*good, *amount);
            }
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Inventory<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut new_inventory: Inventory<T> = self.0.into_iter().collect();
        for (good, amount) in rhs.iter() {
            let maybe_current = new_inventory.insert(*good, *amount);
            if let Some(current) = maybe_current {
                new_inventory.insert(*good, current.sub(*amount));
            }
        }
        new_inventory
    }
}
