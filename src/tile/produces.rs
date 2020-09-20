use crate::good::{Inventory, Good};
use std::iter::FromIterator;
use std::collections::hash_map::Keys;

#[derive(Default)]
pub struct Produces(Inventory);

impl Produces {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(pairs: &[(Good, i32)]) -> Self {
        Self::from_iter(pairs.iter())
    }

    pub fn keys(&self) -> Keys<'_, Good, i32> {
        self.0.keys()
    }

    pub fn get(&self, key: &Good) -> Option<&i32> {
        self.0.get(key)
    }
}

impl FromIterator<(Good, i32)> for Produces {
    fn from_iter<T: IntoIterator<Item=(Good, i32)>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<Inventory>())
    }
}

impl<'a> FromIterator<&'a (Good, i32)> for Produces {
    fn from_iter<T: IntoIterator<Item=&'a (Good, i32)>>(iter: T) -> Self {
        Self(iter.into_iter().cloned().collect::<Inventory>())
    }
}
