use std::sync::{Arc, RwLock, RwLockReadGuard};

use buildings::buildings_controller::BuildingsController;

use crate::clock::Clock;
use crate::map::buildings::buildings_updater::BuildingsUpdater;
use crate::map::buildings::Buildings;
use crate::map::fow::FOW;
use crate::map::terrain::Terrain;
use crate::map::territories::Territories;
use std::marker::PhantomData;
use std::ops::Deref;

pub mod buildings;
pub mod fow;
pub mod minimap;
pub mod terrain;
pub mod territories;

pub struct MapStorage {
    pub terrain: Terrain,
    pub territories: Territories,
    pub fow: FOW,
    pub buildings: Buildings,
}

pub struct Map {
    map_storage: Arc<RwLock<MapStorage>>,
    buildings_controller: BuildingsController,
    buildings_updater: Arc<BuildingsUpdater>,
}

pub trait GetRef<T> {
    fn get_ref(&self) -> &T;
}

macro_rules! map_get_ref {
    ($type:ty, $field:ident) => {
        impl GetRef<$type> for MapStorage {
            fn get_ref(&self) -> &$type {
                &self.$field
            }
        }
    };
}

map_get_ref!(Terrain, terrain);
map_get_ref!(Territories, territories);
map_get_ref!(FOW, fow);
map_get_ref!(Buildings, buildings);

pub struct MapReadRef<'reference, E, T: GetRef<E>> {
    read_guard: RwLockReadGuard<'reference, T>,
    phantom: PhantomData<E>,
}

impl<E, T: GetRef<E>> Deref for MapReadRef<'_, E, T> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.read_guard.get_ref()
    }
}

impl<E, T: GetRef<E>> AsRef<E> for MapReadRef<'_, E, T> {
    fn as_ref(&self) -> &E {
        self
    }
}

impl<'reference, E, T: GetRef<E>> From<RwLockReadGuard<'reference, T>>
    for MapReadRef<'reference, E, T>
{
    fn from(read_guard: RwLockReadGuard<'reference, T>) -> Self {
        MapReadRef {
            read_guard,
            phantom: PhantomData,
        }
    }
}

impl Map {
    pub fn new(clock: &Clock, rows: usize, columns: usize, island_noise: f64) -> Self {
        let map_storage = Arc::new(RwLock::new(MapStorage {
            terrain: Terrain::new(rows, columns, island_noise),
            territories: Default::default(),
            fow: Default::default(),
            buildings: Default::default(),
        }));

        Map {
            buildings_controller: BuildingsController::new(map_storage.clone()),
            buildings_updater: BuildingsUpdater::new(clock, map_storage.clone()),
            map_storage,
        }
    }

    fn map_storage(&self) -> RwLockReadGuard<MapStorage> {
        self.map_storage.read().unwrap()
    }

    pub fn terrain(&self) -> MapReadRef<Terrain, MapStorage> {
        self.map_storage().into()
    }

    pub fn territories(&self) -> MapReadRef<Territories, MapStorage> {
        self.map_storage().into()
    }

    pub fn fow(&self) -> MapReadRef<FOW, MapStorage> {
        self.map_storage().into()
    }

    pub fn buildings(&self) -> MapReadRef<Buildings, MapStorage> {
        self.map_storage().into()
    }

    pub fn buildings_controller(&self) -> &BuildingsController {
        &self.buildings_controller
    }
}
