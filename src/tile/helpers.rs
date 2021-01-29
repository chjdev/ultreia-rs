use crate::good::{Good, Inventory, InventoryAmount};
use std::ops::DerefMut;

pub fn add_assign(me: &mut impl DerefMut<Target = Inventory>, rhs: (&Good, &InventoryAmount)) {
    let (good, value) = rhs;
    let inventory: &mut Inventory = me.deref_mut();
    if let Some(old) = inventory.get_mut(good) {
        *old = old.saturating_add(*value);
    }
}

pub fn sub_assign(me: &mut impl DerefMut<Target = Inventory>, rhs: (&Good, &InventoryAmount)) {
    let (good, value) = rhs;
    let inventory: &mut Inventory = me.deref_mut();
    if let Some(old) = inventory.get_mut(good) {
        *old = old.saturating_sub(*value);
    }
}
