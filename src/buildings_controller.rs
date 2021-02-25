use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
use crate::good::Inventory;
use crate::map::minimap::SetByCoordinate;
use crate::map::minimap::{FillByCoordinate, GetByCoordinate};
use crate::map::Map;
use crate::tile::state::State;
use crate::tile::{TileFactory, TileName};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock};
use strum_macros::AsRefStr;

#[derive(Debug, AsRefStr)]
pub enum ConstructionError {
    InvalidTerrain,
    InsufficientResources,
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

impl BuildingsController {
    pub fn new(map: Arc<RwLock<Map>>) -> Self {
        Self {
            map,
            tile_factory: TileFactory::new(),
        }
    }

    pub fn territory_state(&mut self, coordinate: &Coordinate) -> Option<State> {
        self.map
            .write()
            .unwrap()
            .fow
            .set(Coordinate::default(), true);
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
        let influence = tile.influence_at(&coordinate);
        let mut mut_map = self.map.write().unwrap();
        mut_map.fow.fill(&influence, true);

        mut_map.buildings.set(coordinate, Some(tile.create()));

        Ok(())
    }
}
