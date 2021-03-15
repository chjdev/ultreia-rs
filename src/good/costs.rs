use crate::good::{SpecializedInventory, WithFromInventory};

pub struct CostsMarker;
impl WithFromInventory for CostsMarker {}

pub type Costs = SpecializedInventory<CostsMarker>;
