use crate::coordinate::Coordinate;
use noise::{Perlin, Seedable, NoiseFn};

#[repr(C)]
pub enum TerrainType {
    Bare,
    Beach,
    Grassland,
    None,
    Ocean,
    Scorched,
    Shrubland,
    Snow,
    SubtropicalDesert,
    Taiga,
    TemperateDeciduousForest,
    TemperateDesert,
    TemperateRainForest,
    TropicalRainForest,
    TropicalSeasonalForest,
    Tundra,
}

impl TerrainType {
    pub fn new(elevation: f64, moisture: f64) -> Self {
        if elevation < 0.1 {
            return TerrainType::Ocean;
        }
        if elevation < 0.12 {
            return TerrainType::Beach;
        }

        if elevation > 0.8 {
            if moisture < 0.1 {
                return TerrainType::Scorched;
            }
            if moisture < 0.2 {
                return TerrainType::Bare;
            }
            if moisture < 0.5 {
                return TerrainType::Tundra;
            }
            return TerrainType::Snow;
        }

        if elevation > 0.6 {
            if moisture < 0.33 {
                return TerrainType::TemperateDesert;
            }
            if moisture < 0.66 {
                return TerrainType::Shrubland;
            }
            return TerrainType::Taiga;
        }

        if elevation > 0.3 {
            if moisture < 0.16 {
                return TerrainType::TemperateDesert;
            }
            if moisture < 0.50 {
                return TerrainType::Grassland;
            }
            if moisture < 0.83 {
                return TerrainType::TemperateDeciduousForest;
            }
            return TerrainType::TemperateRainForest;
        }

        if moisture < 0.16 {
            return TerrainType::SubtropicalDesert;
        }
        if moisture < 0.33 {
            return TerrainType::Grassland;
        }
        if moisture < 0.66 {
            return TerrainType::TropicalSeasonalForest;
        }
        return TerrainType::TropicalRainForest;
    }
}


#[repr(C)]
pub struct TerrainTile {
    pub elevation: f64,
    pub moisture: f64,
    pub terrain_type: TerrainType,
}

const NONE_TERRAIN_TILE: TerrainTile = TerrainTile {
    elevation: 0.0,
    moisture: 0.0,
    terrain_type: TerrainType::None,
};

impl TerrainTile {
    pub fn new(elevation: f64, moisture: f64, terrain_type: TerrainType) -> Self {
        TerrainTile {
            elevation,
            moisture,
            terrain_type,
        }
    }

    pub fn none() -> Self {
        NONE_TERRAIN_TILE
    }
}

pub struct Terrain {
    perlin: Perlin
}

impl Terrain {
    pub fn new_seeded(seed: u32) -> Self {
        let perlin = Perlin::new();
        perlin.set_seed(seed);
        Terrain {
            perlin
        }
    }

    pub fn new() -> Self {
        Terrain::new_seeded(0)
    }

    fn noise(&self, x: f64, y: f64) -> f64 {
        (self.perlin.get([x, y]) + 1.) / 2.
    }

    // https://www.redblobgames.com/maps/terrain-from-noise/#islands
    pub fn get(&self, _coordinate: &Coordinate) -> TerrainTile {
        let x = 0.0;
        let y = 0.0;
        let width = 0.0;
        let height = 0.0;
        let nx: f64 = x / width - 0.5;
        let ny: f64 = y / height - 0.5;
        // 3 octaves and valley smoothing via pow
        let elevation: f64 = (1. * self.noise(1. * nx, 1. * ny)
            + 0.5 * self.noise(2. * nx, 2. * ny)
            + 0.25 * self.noise(4. * nx, 4. * ny)).powf(1.28);
        // 3 octaves
        let moisture: f64 = 1. * self.noise(1. * nx, 1. * ny)
            + 0.5 * self.noise(2. * nx, 2. * ny)
            + 0.25 * self.noise(4. * nx, 4. * ny);
        let terrain_type = TerrainType::new(elevation, moisture);
        TerrainTile::new(
            elevation,
            moisture,
            terrain_type,
        )
        // NONE_TERRAIN_TILE
    }
}
