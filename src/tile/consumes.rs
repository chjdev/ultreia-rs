use crate::good::{SpecializedInventory, WithFromInventory};

pub struct ConsumesMarker;
impl WithFromInventory for ConsumesMarker {}

pub type Consumes = SpecializedInventory<ConsumesMarker>;
