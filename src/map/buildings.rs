use crate::coordinate::indexed::CoordinateIndexed;
use crate::coordinate::Coordinate;
use crate::map::minimap::{
    GetMutRefByCoordinate, GetRefByCoordinate, SetByCoordinate, TrySetByCoordinate, WithGrid,
};
use crate::observable::{Observable, Observers};
use crate::tile::{TileInstance, TileName};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockResult};

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

    pub fn try_mut(
        &self,
        coordinate: &Coordinate,
    ) -> Option<TryLockResult<RwLockWriteGuard<TileInstance>>> {
        self.buildings.get(coordinate).map(|sync| sync.try_write())
    }
}

impl IntoIterator for Buildings {
    type Item = (Coordinate, SynchronizedInstance);
    type IntoIter = std::collections::hash_map::IntoIter<Coordinate, SynchronizedInstance>;

    fn into_iter(self) -> Self::IntoIter {
        self.buildings.into_iter()
    }
}

impl<'data> IntoParallelIterator for &'data Buildings {
    type Iter = rayon::collections::hash_map::Iter<'data, Coordinate, SynchronizedInstance>;
    type Item = (&'data Coordinate, &'data SynchronizedInstance);

    fn into_par_iter(self) -> Self::Iter {
        self.buildings.par_iter()
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

impl<'reference, 'me: 'reference>
    GetMutRefByCoordinate<'me, Option<RwLockWriteGuard<'reference, TileInstance>>> for Buildings
{
    fn get_mut(
        &'me self,
        coordinate: &Coordinate,
    ) -> Option<RwLockWriteGuard<'reference, TileInstance>> {
        self.buildings
            .get(coordinate)
            .map(|sync| sync.write().unwrap())
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
