use crate::good::{Good, Inventory, InventoryAmount};
use crate::tile::consumes::Consumes;
use derive_more::{Deref, DerefMut, From, Index, IndexMut, Into, IntoIterator};

#[derive(
    Default, Clone, PartialEq, Eq, From, Into, Deref, DerefMut, Index, IndexMut, IntoIterator,
)]
pub struct Produces(Inventory<Consumes>);

impl Produces {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(pairs: &[<Inventory<Consumes> as InventoryAmount>::Entry]) -> Self {
        Self(pairs.into_iter().cloned().collect())
    }
}

impl<'data> IntoIterator for &'data Produces {
    type Item = (&'data Good, &'data Consumes);
    type IntoIter = std::collections::hash_map::Iter<'data, Good, Consumes>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
