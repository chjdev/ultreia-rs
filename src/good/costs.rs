use crate::good::{Inventory, InventoryAmount};
use derive_more::{
    Add, AddAssign, Constructor, Deref, DerefMut, From, Index, IndexMut, Into, Sub, SubAssign,
};
use std::iter::FromIterator;

#[derive(
    Constructor,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
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
pub struct Costs {
    pub inventory: Inventory,
}

impl Costs {
    pub fn from(pairs: &[<Inventory as InventoryAmount>::Entry]) -> Self {
        Self::new(Inventory::from_iter(pairs.into_iter().cloned()))
    }
}
