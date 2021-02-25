use crate::good::{Inventory, InventoryAmount};
use derive_more::{Add, AddAssign, Deref, DerefMut, From, Index, IndexMut, Into, Sub, SubAssign};
use std::iter::FromIterator;

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
pub struct Consumes(Inventory);

impl Consumes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(pairs: &[<Inventory as InventoryAmount>::Entry]) -> Self {
        Self(Inventory::from_iter(pairs.into_iter().cloned()))
    }
}
