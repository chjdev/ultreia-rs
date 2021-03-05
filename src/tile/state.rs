use crate::good::{Good, Inventory, InventoryAmount};
use crate::tile::consumes::Consumes;
use crate::tile::produces::Produces;
use derive_more::{Add, AddAssign, Constructor, Deref, DerefMut, From, Into, Sub, SubAssign};
use std::ops::{Index, IndexMut};

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

impl Index<&Good> for State {
    type Output = <Inventory as InventoryAmount>::Amount;

    fn index(&self, index: &Good) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<&Good> for State {
    fn index_mut(&mut self, index: &Good) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<'a> Into<&'a mut Inventory> for &'a mut State {
    fn into(self) -> &'a mut Inventory {
        &mut self.0
    }
}
