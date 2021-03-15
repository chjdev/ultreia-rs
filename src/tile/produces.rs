use crate::good::SpecializedInventory;
use crate::tile::consumes::Consumes;

pub struct ProducesMarker;
pub type Produces = SpecializedInventory<ProducesMarker, Consumes>;
