/// located in this mod to gain access to mutable buildings
use crate::map::minimap::GetRefByCoordinate;
use crate::map::territories::TerritoryID;
use crate::map::MapStorage;
use crate::tile::state::State;
use crate::tile::{TileInstance, TileName};
use std::cmp::Ordering;
use std::ops::{AddAssign, Deref, SubAssign};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub struct FrozenMutState<'reference> {
    map_guard: SomeRwLockGuard<'reference>,
    write_guards: Vec<RwLockWriteGuard<'reference, TileInstance>>,
    frozen_warehouse: TileInstance,
}

impl<'reference> FrozenMutState<'reference> {
    fn new(
        map_guard: SomeRwLockGuard<'reference>,
        write_guards: Vec<RwLockWriteGuard<'reference, TileInstance>>,
    ) -> Self {
        let mut frozen_warehouse = TileInstance::from_name(&TileName::Warehouse);
        let mut frozen_state = frozen_warehouse.state_mut().unwrap();
        for guard in write_guards.iter() {
            assert!(
                guard.tile().name() == &TileName::Warehouse,
                "only warehouses supported atm"
            );
            if let Some(state) = guard.state() {
                frozen_state += state;
            }
        }
        Self {
            map_guard,
            write_guards,
            frozen_warehouse,
        }
    }

    pub fn update(&mut self) {
        let mut frozen_warehouse = TileInstance::from_name(&TileName::Warehouse);
        let mut frozen_state = frozen_warehouse.state_mut().unwrap();
        for guard in self.write_guards.iter() {
            if let Some(state) = guard.state() {
                frozen_state += state;
            }
        }
        self.frozen_warehouse = frozen_warehouse;
    }
}

impl FrozenMutState<'_> {
    fn fair_match_diff(&mut self, state: &State) {
        let mut change = false;
        for (good, amount) in state.iter() {
            if !self.state().contains_key(good) {
                panic!("can't match difference, other state contains an invalid key!")
            }
            let mut diff = (*amount as i64) - (self.state()[good] as i64);
            let step: i8 = if diff < 0 { 1 } else { -1 };
            while diff != 0 {
                let mut check_impl = false;
                for instance in self.write_guards.iter_mut() {
                    if let Some(state) = instance.state_mut() {
                        if !state.contains_key(good) {
                            continue;
                        }
                        if step < 0 && state[good] < u32::max_value() || step > 0 && state[good] > 0
                        {
                            if step < 0 {
                                state[good] += step.abs() as u32;
                            } else {
                                state[good] -= step.abs() as u32;
                            }
                            diff += step as i64;
                            check_impl = true;
                            change = true;
                        }
                    }
                }
                if !check_impl {
                    panic!("goods in states don't match up or state difference can't be satisfied",)
                }
            }
        }
        if change {
            self.update();
        }
    }
}

impl SubAssign<&State> for FrozenMutState<'_> {
    fn sub_assign(&mut self, rhs: &State) {
        let mut new_state: State = self.state().clone();
        new_state -= rhs;
        self.fair_match_diff(&new_state);
    }
}

impl AddAssign<&State> for FrozenMutState<'_> {
    fn add_assign(&mut self, rhs: &State) {
        let mut new_state: State = self.state().clone();
        new_state += rhs;
        self.fair_match_diff(&new_state);
    }
}

macro_rules! common_frozen {
    ($type:ty) => {
        impl Deref for $type {
            type Target = State;

            fn deref(&self) -> &Self::Target {
                self.state()
            }
        }

        impl AsRef<State> for $type {
            fn as_ref(&self) -> &State {
                self
            }
        }
        impl PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.state().eq(other.state())
            }
        }

        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.state().partial_cmp(other.state())
            }
        }

        impl $type {
            pub fn state(&self) -> &State {
                self.frozen_warehouse.state().unwrap()
            }
        }
    };
}

common_frozen!(FrozenMutState<'_>);

pub struct FrozenState<'reference> {
    map_guard: SomeRwLockGuard<'reference>,
    read_guards: Vec<RwLockReadGuard<'reference, TileInstance>>,
    frozen_warehouse: TileInstance,
}

impl<'reference> FrozenState<'reference> {
    fn new(
        map_guard: SomeRwLockGuard<'reference>,
        read_guards: Vec<RwLockReadGuard<'reference, TileInstance>>,
    ) -> Self {
        let mut frozen_warehouse = TileInstance::from_name(&TileName::Warehouse);
        let mut frozen_state = frozen_warehouse.state_mut().unwrap();
        for guard in read_guards.iter() {
            if let Some(state) = guard.state() {
                frozen_state += state;
            }
        }
        Self {
            map_guard,
            read_guards,
            frozen_warehouse,
        }
    }
}

common_frozen!(FrozenState<'_>);

enum SomeRwLockGuard<'reference> {
    Read(&'reference RwLockReadGuard<'reference, MapStorage>),
    Write(&'reference RwLockWriteGuard<'reference, MapStorage>),
}

pub struct TerritoriesState;

pub trait TerritoriesStateRw<'reference, T> {
    fn freeze(map: &'reference T, territory_id: &TerritoryID) -> FrozenState<'reference>;
    fn freeze_mut(map: &'reference T, territory_id: &TerritoryID) -> FrozenMutState<'reference>;
}

impl<'reference> TerritoriesStateRw<'reference, RwLockReadGuard<'reference, MapStorage>>
    for TerritoriesState
{
    fn freeze(
        map: &'reference RwLockReadGuard<'reference, MapStorage>,
        territory_id: &TerritoryID,
    ) -> FrozenState<'reference> {
        let warehouse_read_guards = TerritoriesState::read_guards(map.deref(), territory_id);
        FrozenState::new(SomeRwLockGuard::Read(map), warehouse_read_guards)
    }

    fn freeze_mut(
        map: &'reference RwLockReadGuard<'reference, MapStorage>,
        territory_id: &TerritoryID,
    ) -> FrozenMutState<'reference> {
        let warehouse_write_guards = TerritoriesState::write_guards(map.deref(), territory_id);
        FrozenMutState::new(SomeRwLockGuard::Read(map), warehouse_write_guards)
    }
}

impl<'reference> TerritoriesStateRw<'reference, RwLockWriteGuard<'reference, MapStorage>>
    for TerritoriesState
{
    fn freeze(
        map: &'reference RwLockWriteGuard<'reference, MapStorage>,
        territory_id: &TerritoryID,
    ) -> FrozenState<'reference> {
        let warehouse_read_guards = TerritoriesState::read_guards(map.deref(), territory_id);
        FrozenState::new(SomeRwLockGuard::Write(map), warehouse_read_guards)
    }

    fn freeze_mut(
        map: &'reference RwLockWriteGuard<'reference, MapStorage>,
        territory_id: &TerritoryID,
    ) -> FrozenMutState<'reference> {
        let warehouse_write_guards = TerritoriesState::write_guards(map.deref(), territory_id);
        FrozenMutState::new(SomeRwLockGuard::Write(map), warehouse_write_guards)
    }
}

impl<'reference> TerritoriesState {
    fn read_guards(
        map: &'reference MapStorage,
        territory_id: &TerritoryID,
    ) -> Vec<RwLockReadGuard<'reference, TileInstance>> {
        let mut warehouse_read_guards = vec![];
        for coordinate in map.territories.get_territory(territory_id).unwrap() {
            if let Some(instance) = map.buildings.get(&coordinate) {
                if instance.tile().name() == &TileName::Warehouse {
                    warehouse_read_guards.push(instance);
                }
            }
        }
        warehouse_read_guards
    }
    fn write_guards(
        map: &'reference MapStorage,
        territory_id: &TerritoryID,
    ) -> Vec<RwLockWriteGuard<'reference, TileInstance>> {
        let mut warehouse_write_guards = vec![];
        for coordinate in map.territories.get_territory(territory_id).unwrap() {
            if let Some(instance) = map.buildings.get_mut(&coordinate) {
                if instance.tile().name() == &TileName::Warehouse {
                    warehouse_write_guards.push(instance);
                }
            }
        }
        warehouse_write_guards
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordinate::Coordinate;
    use crate::good::{Good, ImmaterialGood};
    use crate::map::buildings::buildings_controller::BuildingsController;
    use crate::map::minimap::GetByCoordinate;
    use crate::map::terrain::Terrain;
    use std::sync::{Arc, RwLock};

    #[test]
    fn test_simple_update() {
        let map_storage = Arc::new(RwLock::new(MapStorage {
            terrain: Terrain::new_seeded(3, 20, 20, 0.),
            territories: Default::default(),
            fow: Default::default(),
            buildings: Default::default(),
        }));
        BuildingsController::do_construct(
            map_storage.write().unwrap(),
            Coordinate::default(),
            (&TileName::Warehouse).into(),
        );
        BuildingsController::do_construct(
            map_storage.write().unwrap(),
            Coordinate::new(1, 1),
            (&TileName::Warehouse).into(),
        );
        let map_storage_mut = map_storage.write().unwrap();
        {
            let warehouse1 = map_storage_mut
                .buildings
                .get(&Coordinate::default())
                .unwrap();
            assert!(warehouse1.tile().name() == &TileName::Warehouse);
            let warehouse2 = map_storage_mut
                .buildings
                .get(&Coordinate::new(1, 1))
                .unwrap();
            assert!(warehouse2.tile().name() == &TileName::Warehouse);
        }
        let territory_id: Option<TerritoryID> =
            map_storage_mut.territories.get(&Coordinate::default());
        assert!(territory_id.unwrap() == TerritoryID::default());
        {
            let mut state = TerritoriesState::freeze_mut(&map_storage_mut, &territory_id.unwrap());
            assert_eq!(state[&Good::ImmaterialGood(ImmaterialGood::Money)], 1000);
            state += &state
                .state()
                .blueprint_from_iter(vec![(Good::Money(), 10)])
                .unwrap();
        }
        {
            let state = TerritoriesState::freeze(&map_storage_mut, &territory_id.unwrap());
            assert_eq!(state[&Good::Money()], 1010);
        }
        {
            let warehouse1 = map_storage_mut
                .buildings
                .get(&Coordinate::default())
                .unwrap();
            assert_eq!(warehouse1.state().unwrap()[&Good::Money()], 1005);
            let warehouse2 = map_storage_mut
                .buildings
                .get(&Coordinate::new(1, 1))
                .unwrap();
            assert_eq!(warehouse2.state().unwrap()[&Good::Money()], 5);
        }
        {
            let mut state = TerritoriesState::freeze_mut(&map_storage_mut, &territory_id.unwrap());
            assert_eq!(state[&Good::Money()], 1010);
            state -= &state
                .state()
                .blueprint_from_iter(vec![(Good::Money(), 25)])
                .unwrap();
        }
        {
            let state = TerritoriesState::freeze(&map_storage_mut, &territory_id.unwrap());
            assert_eq!(state[&Good::Money()], 1010 - 25);
        }
        {
            let warehouse1 = map_storage_mut
                .buildings
                .get(&Coordinate::default())
                .unwrap();
            assert_eq!(warehouse1.state().unwrap()[&Good::Money()], 1005 - 20);
            let warehouse2 = map_storage_mut
                .buildings
                .get(&Coordinate::new(1, 1))
                .unwrap();
            assert_eq!(warehouse2.state().unwrap()[&Good::Money()], 0);
        }
    }
}
