use crate::good::Inventory;
use crate::tile::consumes::Consumes;
use crate::tile::produces::Produces;
use derive_more::{
    Add, AddAssign, Constructor, Deref, DerefMut, From, Index, IndexMut, Into, Sub, SubAssign,
};
use std::ops::{AddAssign, SubAssign};

#[derive(
    Constructor,
    Default,
    Clone,
    PartialEq,
    Eq,
    From,
    Into,
    Deref,
    DerefMut,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Index,
    IndexMut,
)]
pub struct State(Inventory);

impl State {
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

// todo duplicates code in inventory
impl AddAssign<&State> for State {
    fn add_assign(&mut self, rhs: &State) {
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

// todo duplicates code in inventory, overflow!
impl SubAssign<&State> for State {
    fn sub_assign(&mut self, rhs: &State) {
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
