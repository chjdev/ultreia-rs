use crate::coordinate::Coordinate;
use crate::good::Good;
use crate::map::minimap::GetRefByCoordinate;
use crate::map::minimap::{FillByCoordinate, GetByCoordinate, SetByCoordinate};
use crate::map::territories::{TerritoriesState, TerritoriesStateRw, TerritoryID};
use crate::map::MapStorage;
use crate::tile::{Tile, TileInstance, TileName};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

#[derive(Debug, AsRefStr, EnumString, EnumVariantNames)]
pub enum ConstructionError {
    InvalidTerrain,
    InvalidTerritory,
    InsufficientResources,
    CoordinateOccupied,
}

impl fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Error for ConstructionError {}

pub struct BuildingsController {
    map_storage: Arc<RwLock<MapStorage>>,
}

impl BuildingsController {
    pub fn new(map_storage: Arc<RwLock<MapStorage>>) -> Self {
        Self { map_storage }
    }

    pub fn try_construct(
        &self,
        coordinate: Coordinate,
        tile_name: &TileName,
    ) -> Result<(), ConstructionError> {
        let map = self.map_storage.write().unwrap();

        // are we in a valid territory?
        let maybe_territory_id: Option<TerritoryID> = map.territories.get(&coordinate);
        let is_warehouse = tile_name == &TileName::Warehouse;
        if !maybe_territory_id.is_some() && !is_warehouse {
            // todo is territory owned by player?
            return Err(ConstructionError::InvalidTerritory);
        }

        // is the coordinate free?
        if map.buildings.get(&coordinate).is_some() {
            return Err(ConstructionError::CoordinateOccupied);
        }

        // is this tile even allowed here?
        let tile: &'static dyn Tile = tile_name.into();
        if !tile.allowed(&coordinate, &map) {
            return Err(ConstructionError::InvalidTerrain);
        }

        // do we have enough resources?
        if let (Some(costs), Some(territory_id)) = (tile.costs(), maybe_territory_id) {
            let mut territory_state = TerritoriesState::freeze_mut(&map, &territory_id);
            if territory_state.state() < costs {
                return Err(ConstructionError::InsufficientResources);
            }
            // we are updating it here so we can free up the state freeze and don't run into borrow mut after borrow immut
            let costs_sub = territory_state
                .state()
                .blueprint(costs)
                .expect("implementation error: costs is not a subset of the territory state!");
            territory_state -= &costs_sub;
        }
        // WARNING: after the resource update the construction may _NOT_ fail anymore
        Ok(Self::do_construct(map, coordinate, tile))
    }

    pub fn do_construct(
        mut map: RwLockWriteGuard<MapStorage>,
        coordinate: Coordinate,
        tile: &'static dyn Tile,
    ) {
        // create the building
        map.buildings
            .set(coordinate, Some(TileInstance::from(tile)));

        // update the fog of war
        let influence = tile.influence_at(&coordinate);
        map.fow.fill(influence.clone(), true);

        // extend the territory
        let is_warehouse = tile.name() == &TileName::Warehouse;
        if is_warehouse {
            let maybe_territory_id: Option<TerritoryID> = map.territories.get(&coordinate);
            if let Some(territory_id) = maybe_territory_id {
                map.territories.extend(&territory_id, influence);
            } else {
                map.territories.create(influence);
                // todo move to settler unit or something, just for testing atm
                let mut instance = map.buildings.get_mut(&coordinate).unwrap();
                let mut state = instance.state_mut().unwrap();
                let add = state
                    .blueprint_from_iter(vec![(Good::Money(), 1000)])
                    .unwrap();
                state += add;
            }
        }
    }
}
