use crate::coordinate::{Coordinate, Offset};
use noise::{Perlin, Seedable, NoiseFn};
use crate::coordinate::range::{Range, RangeFrom};

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

impl Default for TerrainType {
    fn default() -> Self {
        Self::None
    }
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
#[derive(Default)]
pub struct TerrainTile {
    elevation: f64,
    moisture: f64,
    terrain_type: TerrainType,
}

impl TerrainTile {
    pub fn new(elevation: f64, moisture: f64, terrain_type: TerrainType) -> Self {
        TerrainTile {
            elevation,
            moisture,
            terrain_type,
        }
    }
}

pub struct Terrain {
    width: f64,
    height: f64,
    random_elevation: Perlin,
    random_moisture: Perlin,
}

impl Terrain {
    pub fn new_seeded(rows: usize, columns: usize, seed: u32) -> Self {
        let random_elevation = Perlin::new();
        random_elevation.set_seed(seed);
        let random_moisture = Perlin::new();
        // todo
        random_moisture.set_seed(3 * seed);
        Terrain {
            width: rows as f64,
            height: columns as f64,
            random_elevation,
            random_moisture,
        }
    }

    pub fn new(rows: usize, columns: usize) -> Self {
        Terrain::new_seeded(rows, columns, 0)
    }

    fn random_elevation(&self, x: f64, y: f64) -> f64 {
        (self.random_elevation.get([x, y]) + 1.) / 2.
    }

    fn random_moisture(&self, x: f64, y: f64) -> f64 {
        (self.random_moisture.get([x, y]) + 1.) / 2.
    }

    // https://www.redblobgames.com/maps/terrain-from-noise/#islands
    pub fn get(&self, coordinate: &Coordinate) -> TerrainTile {
        let offset: Offset = coordinate.into();
        let x = offset.column as f64;
        let y = offset.row as f64;
        if x > self.width || y > self.height {
            return Default::default();
        }
        let nx = 2.0 * (x / self.width - 0.5);
        let ny = 2.0 * (y / self.height - 0.5);
        // 4 octaves and valley smoothing via pow
        let elevation: f64 = self.random_elevation(8. * nx, 8. * ny).powf(1.5);
        // 4 octaves
        let moisture: f64 = self.random_moisture(8. * nx, 8. * ny);
        let terrain_type = TerrainType::new(elevation, moisture);
        TerrainTile::new(
            elevation,
            moisture,
            terrain_type,
        )
    }

    pub fn range(&self, range: &Range) -> Vec<TerrainTile> {
        range.into_iter().map(|c| self.get(c)).collect()
    }

    pub fn rectangle(&self, from_corner: &Coordinate, to_corner: &Coordinate) -> Vec<TerrainTile> {
        self.range(&from_corner.rectangle_to(to_corner))
    }
}
