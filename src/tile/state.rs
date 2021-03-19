use crate::good::costs::Costs;
use crate::good::{Inventory, InventoryAmount, SpecializedInventory};
use crate::tile::consumes::Consumes;
use crate::tile::produces::Produces;
use std::cmp::Ordering;
use std::iter::FromIterator;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct StateMarker;
pub type State = SpecializedInventory<StateMarker>;

impl State {
    pub fn combine(
        maybe_consumes: Option<&Consumes>,
        maybe_produces: Option<&Produces>,
    ) -> Option<Self> {
        if maybe_consumes.is_none() && maybe_produces.is_none() {
            return None;
        }
        let mut inventory = State::default();
        if let Some(consumes) = maybe_consumes {
            for good in consumes.keys() {
                inventory.insert(*good, 0);
            }
        }
        if let Some(produces) = maybe_produces {
            for good in produces.keys() {
                inventory.insert(*good, 0);
            }
        }
        Some(inventory)
    }

    pub fn blueprint_zero(&self) -> Self {
        let mut inventory = State::default();
        for good in self.keys() {
            inventory.inventory_mut().insert(*good, 0);
        }
        inventory
    }

    pub fn blueprint(&self, other: &Inventory) -> Option<Self> {
        if !self.keys().all(|key| self.contains_key(key)) {
            return None;
        }
        let mut converted = self.blueprint_zero();
        converted.inventory_mut().add_assign(other);
        Some(converted)
    }

    pub fn blueprint_from_iter<I>(&self, iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = <Self as InventoryAmount>::Entry>,
    {
        self.blueprint(&Inventory::from_iter(iter))
    }
}

impl PartialEq<Costs> for State {
    fn eq(&self, other: &Costs) -> bool {
        self.inventory().eq(other.inventory())
    }
}

impl PartialOrd<Costs> for State {
    fn partial_cmp(&self, other: &Costs) -> Option<Ordering> {
        self.inventory().partial_cmp(other.inventory())
    }
}
