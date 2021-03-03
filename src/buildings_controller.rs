use crate::clock::Tick;
use crate::coordinate::Coordinate;
use crate::map::minimap::TrySetByCoordinate;
use crate::map::minimap::{FillByCoordinate, GetByCoordinate};
use crate::map::territories::TerritoryID;
use crate::map::Map;
use crate::observable::Observer;
use crate::tile::state::State;
use crate::tile::{TileFactory, TileName};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock, Weak};
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
    // todo should it be weak? arc is more comfortable though
    map: Arc<RwLock<Map>>,
    tile_factory: TileFactory,
}

struct TileUpdater {
    map: Weak<RwLock<Map>>,
}

// impl Observer<Tick> for TileUpdater {
//     fn notify(&self, _event: &Tick) {
//         if let Some(map) = self.map.upgrade().map(|m| m.read().unwrap()) {
//             for building in map.buildings.par_iter() {}
//         }
//     }
// }

impl BuildingsController {
    pub fn new(map: Arc<RwLock<Map>>) -> Self {
        Self {
            map,
            tile_factory: TileFactory::new(),
        }
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
        let tile = self.tile_factory.tile(tile_name);
        let mut mut_map = self.map.write().unwrap();

        let is_warehouse = tile_name == &TileName::Warehouse;
        let territory_id: Option<TerritoryID> = mut_map.territories.get(&coordinate);
        if territory_id.is_some() || is_warehouse {
            if !mut_map
                .buildings
                .try_set(coordinate, Some(self.tile_factory.create(tile.name())))
            {
                return Err(ConstructionError::CoordinateOccupied);
            }
        } else {
            return Err(ConstructionError::InvalidTerritory);
        }

        let influence = tile.influence_at(&coordinate);
        mut_map.fow.fill(influence.clone(), true);

        if is_warehouse {
            let maybe_territory_id: Option<TerritoryID> = mut_map.territories.get(&coordinate);
            if let Some(territory_id) = maybe_territory_id {
                mut_map.territories.extend(&territory_id, influence)
            }
        }

        Ok(())
    }
}
