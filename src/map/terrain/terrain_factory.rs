mod terrain_elevation;
mod terrain_moisture;
mod terrain_type;
mod terrain_yields;

use crate::map::terrain::{Latitude, Longitude};
use terrain_elevation::TerrainElevationFactory;
use terrain_moisture::TerrainMoistureFactory;
pub use terrain_type::TerrainType;
use terrain_type::TerrainTypeFactory;
use terrain_yields::TerrainYieldsFactory;
pub use terrain_yields::{TerrainYields, Yield};

#[derive(Default)]
pub struct TerrainMeta {
    elevation: f64,
    moisture: f64,
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

    pub fn elevation(&self) -> f64 {
        self.elevation
    }

    pub fn moisture(&self) -> f64 {
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
            yields_factory: TerrainYieldsFactory::new(),
            type_factory: TerrainTypeFactory::new(),
        }
    }

    pub fn create(&self, nx: f64, ny: f64) -> TerrainMeta {
        let elevation = self.elevation_factory.create(nx, ny);
        let moisture = self.moisture_factory.create(nx, ny);
        let longitude: Longitude = Longitude::saturating_from(nx);
        let latitude: Latitude = Latitude::saturating_from(ny);
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
