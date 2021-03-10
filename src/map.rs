use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use buildings::buildings_controller::BuildingsController;

use crate::clock::Clock;
use crate::map::buildings::buildings_updater::BuildingsUpdater;
use crate::map::buildings::Buildings;
use crate::map::fow::FOW;
use crate::map::terrain::Terrain;
use crate::map::territories::Territories;

pub mod buildings;
pub mod fow;
pub mod minimap;
pub mod terrain;
pub mod territories;

pub struct MapStorage {
    terrain: RwLock<Terrain>,
    territories: RwLock<Territories>,
    fow: RwLock<FOW>,
    buildings: RwLock<Buildings>,
}

impl MapStorage {
    pub fn terrain(&self) -> RwLockReadGuard<Terrain> {
        self.terrain.read().unwrap()
    }

    pub fn territories(&self) -> RwLockReadGuard<Territories> {
        self.territories.read().unwrap()
    }

    pub fn fow(&self) -> RwLockReadGuard<FOW> {
        self.fow.read().unwrap()
    }

    pub fn buildings(&self) -> RwLockReadGuard<Buildings> {
        self.buildings.read().unwrap()
    }

    fn terrain_mut(&self) -> RwLockWriteGuard<Terrain> {
        self.terrain.write().unwrap()
    }

    fn territories_mut(&self) -> RwLockWriteGuard<Territories> {
        self.territories.write().unwrap()
    }

    fn fow_mut(&self) -> RwLockWriteGuard<FOW> {
        self.fow.write().unwrap()
    }

    fn buildings_mut(&self) -> RwLockWriteGuard<Buildings> {
        self.buildings.write().unwrap()
    }
}

pub struct Map {
    map_storage: Arc<MapStorage>,
    buildings_controller: BuildingsController,
    buildings_updater: Arc<BuildingsUpdater>,
}

impl Map {
    pub fn new(clock: &Clock, rows: usize, columns: usize, island_noise: f64) -> Self {
        let map_storage = Arc::new(MapStorage {
            terrain: RwLock::new(Terrain::new(rows, columns, island_noise)),
            territories: Default::default(),
            fow: Default::default(),
            buildings: Default::default(),
        });

        Map {
            buildings_controller: BuildingsController::new(map_storage.clone()),
            buildings_updater: BuildingsUpdater::new(clock, Arc::downgrade(&map_storage)),
            map_storage,
        }
    }

    pub fn terrain(&self) -> RwLockReadGuard<Terrain> {
        self.map_storage.terrain()
    }

    pub fn territories(&self) -> RwLockReadGuard<Territories> {
        self.map_storage.territories()
    }

    pub fn fow(&self) -> RwLockReadGuard<FOW> {
        self.map_storage.fow()
    }

    pub fn buildings(&self) -> RwLockReadGuard<Buildings> {
        self.map_storage.buildings()
    }

    pub fn buildings_controller(&self) -> &BuildingsController {
        &self.buildings_controller
    }
}
