use super::Good;
use derive_more::{AsRef, Deref, DerefMut, From, Into};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, Sub, SubAssign};

#[derive(Default, Clone, PartialEq, Eq, From, Into, Deref, DerefMut, AsRef)]
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

impl<T> Inventory<T> {
    pub fn get(&self, key: &Good) -> Option<&T> {
        self.0.get(key)
    }
    pub fn get_mut(&mut self, key: &Good) -> Option<&mut T> {
        self.0.get_mut(key)
    }
}

impl<T: PartialOrd> PartialOrd for Inventory<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // only valid if they are subsets
        if self.0.keys().all(|key| other.0.contains_key(key)) {
            let mut ordering_acc = Ordering::Equal;
            for (good, value) in self.0.iter() {
                let maybe_value_ordering = value.partial_cmp(other.0.get(good).unwrap());
                if let Some(value_ordering) = maybe_value_ordering {
                    if ordering_acc == Ordering::Equal {
                        ordering_acc = value_ordering;
                    } else if value_ordering != Ordering::Equal && ordering_acc != value_ordering {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            // the values are same: so do we have the same amount of keys?
            if ordering_acc == Ordering::Equal {
                return if self.0.len() == other.0.len() {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Less)
                };
            }
            // the values might be greater but do we miss keys?
            else if ordering_acc == Ordering::Greater {
                return if self.0.len() == other.0.len() {
                    Some(Ordering::Greater)
                } else {
                    return None;
                };
            }
            return Some(ordering_acc);
        }
        // other direction, sameish idea
        if other.0.keys().all(|key| self.0.contains_key(key)) {
            let mut ordering_acc = Ordering::Equal;
            for (good, value) in other.0.iter() {
                let maybe_value_ordering = value.partial_cmp(self.0.get(good).unwrap());
                if let Some(value_ordering) = maybe_value_ordering {
                    if ordering_acc == Ordering::Equal {
                        ordering_acc = value_ordering;
                    } else if value_ordering != Ordering::Equal && ordering_acc != value_ordering {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            // the values are same: so do we have the same amount of keys?
            if ordering_acc == Ordering::Equal {
                return if self.0.len() == other.0.len() {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Greater)
                };
            }
            // the values might be less but does the other miss keys?
            else if ordering_acc == Ordering::Less {
                return if self.0.len() == other.0.len() {
                    Some(Ordering::Less)
                } else {
                    return None;
                };
            }
            return Some(ordering_acc);
        }
        None
    }
}

impl<T: Default> Inventory<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> Index<&Good> for Inventory<T> {
    type Output = <Inventory<T> as InventoryAmount>::Amount;

    fn index(&self, index: &Good) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<&Good> for Inventory<T> {
    fn index_mut(&mut self, index: &Good) -> &mut Self::Output {
        self.0.get_mut(index).unwrap()
    }
}

impl<T> IntoIterator for Inventory<T> {
    type Item = (Good, T);
    type IntoIter = std::collections::hash_map::IntoIter<Good, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> FromIterator<<Self as InventoryAmount>::Entry> for Inventory<T> {
    fn from_iter<I: IntoIterator<Item = <Self as InventoryAmount>::Entry>>(iter: I) -> Self {
        Self(iter.into_iter().collect::<HashMap<Good, T>>())
    }
}

impl<T: AddAssign + Copy> AddAssign<&Inventory<T>> for Inventory<T> {
    fn add_assign(&mut self, rhs: &Self) {
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

impl<T: AddAssign + Copy> AddAssign for Inventory<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs);
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

impl<T: SubAssign + Copy> SubAssign<&Inventory<T>> for Inventory<T> {
    fn sub_assign(&mut self, rhs: &Self) {
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

impl<T: SubAssign + Copy> SubAssign for Inventory<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs);
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

pub struct SpecializedInventory<P, T = u32> {
    inventory: Inventory<T>,
    phantom: PhantomData<P>,
}

impl<P, T> SpecializedInventory<P, T> {
    fn new(inventory: Inventory<T>) -> Self {
        Self {
            inventory,
            phantom: PhantomData,
        }
    }

    pub fn contains_key(&self, key: &Good) -> bool {
        self.inventory.contains_key(key)
    }

    pub fn inventory(&self) -> &Inventory<T> {
        &self.inventory
    }

    pub fn inventory_mut(&mut self) -> &mut Inventory<T> {
        &mut self.inventory
    }
}

impl<P, T: Clone> Clone for SpecializedInventory<P, T> {
    fn clone(&self) -> Self {
        Self::new(self.inventory.clone())
    }
}

impl<P, T: PartialEq> PartialEq for SpecializedInventory<P, T> {
    fn eq(&self, other: &Self) -> bool {
        self.inventory.eq(other)
    }
}

impl<P, T: PartialEq + Eq> Eq for SpecializedInventory<P, T> {}

impl<P, T: PartialOrd> PartialOrd for SpecializedInventory<P, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inventory.partial_cmp(other)
    }
}

impl<P, T: Default> Default for SpecializedInventory<P, T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<P, T> InventoryAmount for SpecializedInventory<P, T> {
    type Amount = T;
    type Entry = (Good, Self::Amount);
}

impl<P, T> Deref for SpecializedInventory<P, T> {
    type Target = Inventory<T>;

    fn deref(&self) -> &Self::Target {
        self.inventory()
    }
}

impl<P, T> DerefMut for SpecializedInventory<P, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inventory_mut()
    }
}

impl<P, T> AsRef<Inventory<T>> for SpecializedInventory<P, T> {
    fn as_ref(&self) -> &Inventory<T> {
        self
    }
}

impl<P, T: Default> Index<&Good> for SpecializedInventory<P, T> {
    type Output = <Inventory<T> as InventoryAmount>::Amount;

    fn index(&self, index: &Good) -> &Self::Output {
        &self.inventory[index]
    }
}

impl<P, T: Default> IndexMut<&Good> for SpecializedInventory<P, T> {
    fn index_mut(&mut self, index: &Good) -> &mut Self::Output {
        &mut self.inventory[index]
    }
}

impl<P, T> IntoIterator for SpecializedInventory<P, T> {
    type Item = (Good, T);
    type IntoIter = std::collections::hash_map::IntoIter<Good, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inventory.into_iter()
    }
}

pub trait WithFromInventory {}

impl<P: WithFromInventory, T> From<Inventory<T>> for SpecializedInventory<P, T> {
    fn from(inventory: Inventory<T>) -> Self {
        Self::new(inventory)
    }
}

impl<P: WithFromInventory, T> FromIterator<<Self as InventoryAmount>::Entry>
    for SpecializedInventory<P, T>
{
    fn from_iter<I: IntoIterator<Item = <Self as InventoryAmount>::Entry>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect::<Inventory<T>>())
    }
}

impl<P, T: AddAssign + Copy> AddAssign<&SpecializedInventory<P, T>> for SpecializedInventory<P, T> {
    fn add_assign(&mut self, rhs: &Self) {
        self.inventory.add_assign(&rhs.inventory)
    }
}

impl<P, T: AddAssign + Copy> AddAssign for SpecializedInventory<P, T> {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs)
    }
}

impl<P, T: AddAssign + Copy> AddAssign<SpecializedInventory<P, T>>
    for &mut SpecializedInventory<P, T>
{
    fn add_assign(&mut self, rhs: SpecializedInventory<P, T>) {
        self.add_assign(&rhs)
    }
}

impl<P, T: AddAssign + Copy> AddAssign<&SpecializedInventory<P, T>>
    for &mut SpecializedInventory<P, T>
{
    fn add_assign(&mut self, rhs: &SpecializedInventory<P, T>) {
        self.inventory.add_assign(&rhs.inventory)
    }
}

impl<P, T: Add<Output = T> + Copy> Add for SpecializedInventory<P, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.inventory + rhs.inventory)
    }
}

impl<P, T: SubAssign + Copy> SubAssign<&SpecializedInventory<P, T>> for SpecializedInventory<P, T> {
    fn sub_assign(&mut self, rhs: &Self) {
        self.inventory.sub_assign(&rhs.inventory)
    }
}

impl<P, T: SubAssign + Copy> SubAssign for SpecializedInventory<P, T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs)
    }
}

impl<P, T: SubAssign + Copy> SubAssign<SpecializedInventory<P, T>>
    for &mut SpecializedInventory<P, T>
{
    fn sub_assign(&mut self, rhs: SpecializedInventory<P, T>) {
        self.sub_assign(&rhs)
    }
}

impl<P, T: SubAssign + Copy> SubAssign<&SpecializedInventory<P, T>>
    for &mut SpecializedInventory<P, T>
{
    fn sub_assign(&mut self, rhs: &SpecializedInventory<P, T>) {
        self.inventory.sub_assign(&rhs.inventory)
    }
}

impl<P, T: Sub<Output = T> + Copy> Sub for SpecializedInventory<P, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.inventory - rhs.inventory)
    }
}

impl<P, T> SpecializedInventory<P, T> {
    pub fn get(&self, key: &Good) -> Option<&T> {
        self.inventory.get(key)
    }
    pub fn get_mut(&mut self, key: &Good) -> Option<&mut T> {
        self.inventory.get_mut(key)
    }
}
