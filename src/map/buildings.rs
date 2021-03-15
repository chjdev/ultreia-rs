use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockResult};

use crossbeam::utils::Backoff;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::coordinate::indexed::CoordinateIndexed;
use crate::coordinate::Coordinate;
use crate::map::minimap::{GetRefByCoordinate, SetByCoordinate, TrySetByCoordinate, WithGrid};
use crate::observable::{Observable, Observers};
use crate::tile::{TileInstance, TileName};

pub mod buildings_controller;
pub mod buildings_updater;
pub mod territories_state;

pub type SynchronizedInstance = RwLock<TileInstance>;

#[derive(Default)]
pub struct Buildings {
    buildings: CoordinateIndexed<SynchronizedInstance>,
    rows: usize,
    columns: usize,
    creators: Observers<BuildingCreated>,
    destroyers: Observers<BuildingDestroyed>,
}

impl Buildings {
    pub fn new(rows: usize, columns: usize) -> Self {
        Buildings {
            buildings: Default::default(),
            rows,
            columns,
            creators: Default::default(),
            destroyers: Default::default(),
        }
    }

    fn get_lock(&self, coordinate: &Coordinate) -> Option<&SynchronizedInstance> {
        self.buildings.get(coordinate)
    }

    fn get_mut(&self, coordinate: &Coordinate) -> Option<RwLockWriteGuard<TileInstance>> {
        self.buildings
            .get(coordinate)
            .map(|sync| sync.write().unwrap())
    }

    fn try_get_mut(
        &self,
        coordinate: &Coordinate,
    ) -> Option<TryLockResult<RwLockWriteGuard<TileInstance>>> {
        self.buildings.get(coordinate).map(|sync| sync.try_write())
    }

    fn spin_get_mut(&self, coordinate: &Coordinate) -> RwLockWriteGuard<TileInstance> {
        let backoff = Backoff::new();
        let mut try_mut_instance = self.try_get_mut(coordinate).unwrap();
        while try_mut_instance.is_err() {
            try_mut_instance = self.try_get_mut(coordinate).unwrap();
            backoff.spin()
        }
        try_mut_instance.unwrap()
    }

    pub fn par_coordinates(&self) -> impl ParallelIterator<Item = &Coordinate> {
        self.buildings.keys().par_bridge()
    }
}

impl WithGrid for Buildings {
    fn rows(&self) -> usize {
        self.rows
    }

    fn columns(&self) -> usize {
        self.columns
    }
}

impl<'reference, 'me: 'reference>
    GetRefByCoordinate<'me, Option<RwLockReadGuard<'reference, TileInstance>>> for Buildings
{
    fn get(
        &'me self,
        coordinate: &Coordinate,
    ) -> Option<RwLockReadGuard<'reference, TileInstance>> {
        self.buildings
            .get(coordinate)
            .map(|sync| sync.read().unwrap())
    }
}

impl SetByCoordinate<Option<TileInstance>> for Buildings {
    fn set(&mut self, coordinate: Coordinate, maybe_instance: Option<TileInstance>) {
        match maybe_instance {
            Some(instance) => {
                let tile_name: TileName = instance.tile().into();
                self.buildings.insert(coordinate, RwLock::new(instance));
                self.notify_all(BuildingCreated {
                    coordinate,
                    tile_name,
                });
            }
            None => {
                self.buildings.remove(&coordinate);
                self.notify_all(BuildingDestroyed { coordinate });
            }
        };
    }
}

impl TrySetByCoordinate<Option<TileInstance>> for Buildings {
    fn try_set(&mut self, coordinate: Coordinate, maybe_instance: Option<TileInstance>) -> bool {
        if self.buildings.contains_key(&coordinate) {
            return false;
        }
        self.set(coordinate, maybe_instance);
        true
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BuildingCreated {
    pub coordinate: Coordinate,
    pub tile_name: TileName,
}

impl Observable<BuildingCreated> for Buildings {
    fn observers(&self) -> &Observers<BuildingCreated> {
        &self.creators
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BuildingDestroyed {
    pub coordinate: Coordinate,
}

impl Observable<BuildingDestroyed> for Buildings {
    fn observers(&self) -> &Observers<BuildingDestroyed> {
        &self.destroyers
    }
}
