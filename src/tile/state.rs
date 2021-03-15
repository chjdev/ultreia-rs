use crate::good::costs::Costs;
use crate::good::SpecializedInventory;
use crate::tile::consumes::Consumes;
use crate::tile::produces::Produces;
use std::cmp::Ordering;

pub struct StateMarker;
pub type State = SpecializedInventory<StateMarker>;

impl State {
    pub fn from_consumes(consumes: &Consumes) -> State {
        Self::combine(Some(consumes), None).unwrap()
    }
    pub fn from_produces(produces: &Produces) -> State {
        Self::combine(None, Some(produces)).unwrap()
    }
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

    pub fn zero(state: &State) -> Self {
        let mut inventory = State::default();
        for good in state.keys() {
            inventory.inventory_mut().insert(*good, 0);
        }
        inventory
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
