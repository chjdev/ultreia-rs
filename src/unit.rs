use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
use crate::good::costs::Costs;
use crate::good::Inventory;
use crate::map::Map;
use std::collections::HashMap;
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString, EnumVariantNames};

#[derive(Copy, Clone, PartialEq, Eq, Hash, EnumIter, AsRefStr, EnumString, EnumVariantNames)]
pub enum UnitName {
    Scout,
}

impl Default for UnitName {
    fn default() -> Self {
        UnitName::Scout
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, EnumIter, AsRefStr, EnumString, EnumVariantNames)]
pub enum CombatType {
    Close,
    Ranged,
    Bombard,
}

pub type CombatRating = HashMap<CombatType, usize>;

pub struct CombatMeta {
    pub max_health: usize,
    pub attack: CombatRating,
    pub defense: CombatRating,
}

pub type SomeUnit = Box<dyn Unit>;

pub trait Unit: Send + Sync {
    fn name(&self) -> &UnitName;
    fn costs(&self) -> Option<&Costs> {
        None
    }
    fn inventory(&self) -> Option<&Inventory> {
        None
    }
    fn combat(&self) -> &CombatMeta;
    fn movement_at(&self, at: &Coordinate) -> Range;
    fn movement(&self) -> Range {
        self.movement_at(&Default::default())
    }
    fn create(&self) -> SomeUnitInstance {
        // DefaultInstance::from(self)
        unimplemented!()
    }
    fn allowed(&self, _at: &Coordinate, _map: &Map) -> bool {
        false
    }
}

pub type SomeUnitInstance = Arc<dyn UnitInstance>;

pub trait UnitInstance: Send + Sync {
    fn unit(&self) -> &'static dyn Unit;
    fn inventory(&self) -> Option<RwLockReadGuard<'_, Inventory>> {
        None
    }
    fn inventory_mut(&self) -> Option<RwLockWriteGuard<'_, Inventory>> {
        None
    }
    fn move_by(&self, steps: usize);
    fn movement_available(&self, steps: usize) -> usize;
    fn update(&self);
}

pub struct UnitFactory {
    units: HashMap<UnitName, SomeUnit>,
}

impl UnitFactory {
    pub fn new() -> Self {
        let mut units: HashMap<UnitName, SomeUnit> = HashMap::new();
        // so we don't forget one, match has to be exhaustive
        // for unit_name in UnitName::iter() {
        //     let unit: SomeUnit = match unit_name {
        //         UnitName::Scout => Box::new(Pioneer::new()),
        //     };
        //     units.insert(unit_name, unit);
        // }
        UnitFactory { units }
    }

    pub fn create(&self, tile: &UnitName) -> SomeUnitInstance {
        self.units.get(&tile).unwrap().create()
    }

    pub fn unit(&self, tile: &UnitName) -> &dyn Unit {
        self.units.get(tile).unwrap().as_ref()
    }
}
