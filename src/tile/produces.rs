use crate::good::{InventoryAmount, SpecializedInventory};
use crate::tile::consumes::Consumes;

pub struct ProducesMarker;

pub type Produces = SpecializedInventory<ProducesMarker, Consumes>;

impl Produces {
    fn from_consumes<I: IntoIterator<Item = <Self as InventoryAmount>::Entry>>(
        consumes: &Consumes,
        iter: I,
    ) -> Self {
        // todo not tooo happy with having default() hanging around, but ok for now (see also State)
        let mut produces = Produces::default();
        for (good, entry) in iter {
            if !entry.keys().all(|good| consumes.contains_key(good)) {
                panic!("Tried to create a production formula with a good not in consumption list!")
            }
            produces.inventory_mut().insert(good, entry);
        }
        produces
    }
}
