use crate::coordinate::Coordinate;
use crate::map::Map;
use crate::tile::{TileFactory, TileName};
use std::error::Error;
use std::fmt;
use std::sync::Arc;
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
    map: Arc<Map>,
    tile_factory: TileFactory,
}

impl BuildingsController {
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
        if let Some(construction_error) = self.can_construct(&coordinate, tile_name) {
            return Err(construction_error);
        }
        Err(ConstructionError::InsufficientResources)
    }
}
