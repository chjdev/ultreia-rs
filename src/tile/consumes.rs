use crate::good::{Inventory, InventoryAmount, SpecializedInventory};
use derive_more::{
    Add, AddAssign, Constructor, Deref, DerefMut, From, Index, IndexMut, Into, Sub, SubAssign,
};
use std::iter::FromIterator;
use std::ops::Add;

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
pub struct Consumes {
    pub inventory: Inventory,
}

impl Consumes {
    pub fn from(pairs: &[<Inventory as InventoryAmount>::Entry]) -> Self {
        Self::new(Inventory::from_iter(pairs.into_iter().cloned()))
    }
}

// pub struct CM;
// pub struct PM;
// pub type Cons = SpecializedInventory<CM>;
// pub type Pons = SpecializedInventory<PM>;
// impl Add<Cons> for Pons {
//     type Output = ();
//
//     fn add(self, rhs: Cons) -> Self::Output {
//         unimplemented!()
//     }
}
