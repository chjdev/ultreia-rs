use crate::coordinate::Coordinate;
use crate::map::minimap::TrySetByCoordinate;
use crate::map::minimap::{FillByCoordinate, GetByCoordinate};
use crate::map::territories::{TerritoriesState, TerritoriesStateRw, TerritoryID};
use crate::map::MapStorage;
use crate::tile::state::State;
use crate::tile::{Tile, TileInstance, TileName};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock};
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

    pub fn territory_state(&mut self, _coordinate: &Coordinate) -> Option<State> {
        // let territory_range: Option<Range> = self.map.territories().get(coordinate);
        // // territory_range.map(|range| range.into_iter().map(|coordinate| self.map.buildings().get(&coordinate)).filter(|some_tile_instance| ))
        //
        // if maybe_territory_id.is_none() {
        //     return None;
        // }

        Default::default()
    }

    pub fn can_construct(
        &self,
        _coordinate: &Coordinate,
        _tile_name: &TileName,
    ) -> Option<ConstructionError> {
        None
    }

    pub fn try_construct(
        &self,
        coordinate: Coordinate,
        tile_name: &TileName,
    ) -> Result<(), ConstructionError> {
        // if let Some(construction_error) = self.can_construct(&coordinate, tile_name) {
        //     return Err(construction_error);
        // }
        // Err(ConstructionError::InsufficientResources)
        let tile: &'static dyn Tile = tile_name.into();
        let is_warehouse = tile_name == &TileName::Warehouse;

        let mut map = self.map_storage.write().unwrap();

        let territory_id: Option<TerritoryID> = map.territories.get(&coordinate);

        {
            let a = TerritoriesState::freeze(&map, &territory_id.unwrap());
        }
        if territory_id.is_some() || is_warehouse {
            if !map
                .buildings
                .try_set(coordinate, Some(TileInstance::from(tile)))
            {
                return Err(ConstructionError::CoordinateOccupied);
            }
        } else {
            return Err(ConstructionError::InvalidTerritory);
        }

        let influence = tile.influence_at(&coordinate);
        map.fow.fill(influence.clone(), true);

        if is_warehouse {
            let maybe_territory_id: Option<TerritoryID> = map.territories.get(&coordinate);
            if let Some(territory_id) = maybe_territory_id {
                map.territories.extend(&territory_id, influence)
            }
        }

        Ok(())
    }
}
