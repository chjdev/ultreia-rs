use crate::good::Inventory;
use crate::tile::consumes::Consumes;
use crate::tile::produces::Produces;
use derive_more::{Add, AddAssign, Deref, DerefMut, From, Index, IndexMut, Into, Sub, SubAssign};

#[derive(
    Default,
    Clone,
    PartialEq,
    Eq,
    From,
    Into,
    Deref,
    DerefMut,
    Index,
    IndexMut,
    Add,
    AddAssign,
    Sub,
    SubAssign,
)]
pub struct State(Inventory);

impl State {
    pub fn new() -> Self {
        Default::default()
    }

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
