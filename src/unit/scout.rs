use crate::coordinate::range::{Range, RangeFrom};
use crate::coordinate::Coordinate;
use crate::unit::{CombatMeta, Unit, UnitName};

pub struct Scout {
    unit: UnitName,
}

impl Scout {
    pub fn new() -> Self {
        Scout {
            unit: UnitName::Scout,
        }
    }
}

impl Unit for Scout {
    fn name(&self) -> &UnitName {
        unimplemented!()
    }

    fn combat(&self) -> &CombatMeta {
        unimplemented!()
    }

    fn movement_at(&self, at: &Coordinate) -> Range {
        unimplemented!()
    }
}
