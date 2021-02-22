mod terrain_elevation;
mod terrain_moisture;
mod terrain_type;
mod terrain_yields;

use crate::map::terrain::{Latitude, Longitude};
use crate::saturating_from::SaturatingInto;
pub use terrain_elevation::Elevation;
use terrain_elevation::TerrainElevationFactory;
pub use terrain_moisture::Moisture;
use terrain_moisture::TerrainMoistureFactory;
pub use terrain_type::TerrainType;
use terrain_type::TerrainTypeFactory;
pub use terrain_yields::TerrainYields;
use terrain_yields::TerrainYieldsFactory;

#[derive(Default, Clone)]
pub struct TerrainMeta {
    elevation: Elevation,
    moisture: Moisture,
    terrain_type: TerrainType,
    yields: TerrainYields,
}

impl TerrainMeta {
    pub fn terrain_type(&self) -> TerrainType {
        self.terrain_type
    }

    pub fn yields(&self) -> &TerrainYields {
        &self.yields
    }

    pub fn elevation(&self) -> Elevation {
        self.elevation
    }

    pub fn moisture(&self) -> Moisture {
        self.moisture
    }
}

pub struct TerrainFactory {
    elevation_factory: TerrainElevationFactory,
    moisture_factory: TerrainMoistureFactory,
    yields_factory: TerrainYieldsFactory,
    type_factory: TerrainTypeFactory,
}

impl TerrainFactory {
    pub fn new(seed: u32, island_noise: f64) -> Self {
        TerrainFactory {
            elevation_factory: TerrainElevationFactory::new(seed, island_noise),
            moisture_factory: TerrainMoistureFactory::new(seed * 3, island_noise * 4.),
            yields_factory: TerrainYieldsFactory::new(seed * 4),
            type_factory: TerrainTypeFactory::new(),
        }
    }

    // a quicker version for minimap and such
    pub fn create_terrain_type(&self, nx: f64, ny: f64) -> TerrainType {
        let elevation = self.elevation_factory.create(nx, ny);
        let moisture = self.moisture_factory.create(nx, ny);
        let latitude: Latitude = ny.saturating_into();
        self.type_factory.create(latitude, elevation, moisture)
    }

    pub fn create(&self, nx: f64, ny: f64) -> TerrainMeta {
        let elevation = self.elevation_factory.create(nx, ny);
        let moisture = self.moisture_factory.create(nx, ny);
        let latitude: Latitude = ny.saturating_into();
        let longitude: Longitude = nx.saturating_into();
        let terrain_type = self.type_factory.create(latitude, elevation, moisture);
        let yields =
            self.yields_factory
                .create(latitude, longitude, elevation, moisture, &terrain_type);
        TerrainMeta {
            elevation,
            moisture,
            terrain_type,
            yields,
        }
    }
}
