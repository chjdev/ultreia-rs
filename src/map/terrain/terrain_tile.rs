use crate::map::terrain::{TerrainType, TerrainYields};

#[derive(Default)]
pub struct TerrainTile {
    elevation: f64,
    moisture: f64,
    terrain_type: TerrainType,
    yields: TerrainYields,
}

impl TerrainTile {
    pub fn new(latitude: f64, longitude: f64, elevation: f64, moisture: f64) -> Self {
        let terrain_type = TerrainType::new(latitude, elevation, moisture);
        let yields = TerrainYields::new(latitude, longitude, elevation, moisture, &terrain_type);
        TerrainTile {
            elevation,
            moisture,
            terrain_type,
            yields,
        }
    }

    pub fn terrain_type(&self) -> TerrainType {
        self.terrain_type
    }

    pub fn yields(&self) -> &TerrainYields {
        &self.yields
    }

    pub fn elevation(&self) -> f64 {
        self.elevation
    }

    pub fn moisture(&self) -> f64 {
        self.moisture
    }
}
