use crate::map::minimap::GetRefByCoordinate;
use crate::map::territories::TerritoryID;
use crate::map::MapStorage;
use crate::tile::state::State;
use crate::tile::{TileInstance, TileName};
use std::ops::{AddAssign, Deref, SubAssign};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub struct FrozenMutState<'reference> {
    map_guard: SomeRwLockGuard<'reference>,
    write_guards: Vec<RwLockWriteGuard<'reference, TileInstance>>,
    frozen_state: State,
}

impl<'reference> FrozenMutState<'reference> {
    fn new(
        map_guard: SomeRwLockGuard<'reference>,
        write_guards: Vec<RwLockWriteGuard<'reference, TileInstance>>,
    ) -> Self {
        let mut frozen_state = State::default();
        for guard in write_guards.iter() {
            if let Some(state) = guard.state() {
                frozen_state += state;
            }
        }
        Self {
            map_guard,
            write_guards,
            frozen_state,
        }
    }
}

impl FrozenMutState<'_> {
    fn fair_match_diff(&mut self, state: &State) {}
}

impl SubAssign<&State> for FrozenMutState<'_> {
    fn sub_assign(&mut self, rhs: &State) {
        let mut new_state = self.frozen_state.clone();
        new_state -= rhs;
        self.fair_match_diff(&new_state);
    }
}

impl AddAssign<&State> for FrozenMutState<'_> {
    fn add_assign(&mut self, rhs: &State) {
        let mut new_state = self.frozen_state.clone();
        new_state += rhs;
        self.fair_match_diff(&new_state);
    }
}

impl Deref for FrozenMutState<'_> {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.frozen_state
    }
}

impl AsRef<State> for FrozenMutState<'_> {
    fn as_ref(&self) -> &State {
        self
    }
}

impl Into<State> for FrozenMutState<'_> {
    fn into(self) -> State {
        self.frozen_state
    }
}

pub struct FrozenState<'reference> {
    map_guard: SomeRwLockGuard<'reference>,
    read_guards: Vec<RwLockReadGuard<'reference, TileInstance>>,
    frozen_state: State,
}

impl<'reference> FrozenState<'reference> {
    fn new(
        map_guard: SomeRwLockGuard<'reference>,
        read_guards: Vec<RwLockReadGuard<'reference, TileInstance>>,
    ) -> Self {
        let mut frozen_state = State::default();
        for guard in read_guards.iter() {
            if let Some(state) = guard.state() {
                frozen_state += state;
            }
        }
        Self {
            map_guard,
            read_guards,
            frozen_state,
        }
    }
}

impl Deref for FrozenState<'_> {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.frozen_state
    }
}

impl AsRef<State> for FrozenState<'_> {
    fn as_ref(&self) -> &State {
        self
    }
}

impl Into<State> for FrozenState<'_> {
    fn into(self) -> State {
        self.frozen_state
    }
}

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
