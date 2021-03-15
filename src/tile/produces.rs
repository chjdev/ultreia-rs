use crate::good::{Good, Inventory, InventoryAmount};
use crate::tile::consumes::Consumes;
use derive_more::{Constructor, Deref, DerefMut, From, Index, IndexMut, Into, IntoIterator};

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
    Index,
    IndexMut,
    IntoIterator,
    PartialOrd,
)]
pub struct Produces {
    pub inventory: Inventory<Consumes>,
}

impl Produces {
    pub fn from(pairs: &[<Inventory<Consumes> as InventoryAmount>::Entry]) -> Self {
        Self::new(pairs.into_iter().cloned().collect())
    }
}
