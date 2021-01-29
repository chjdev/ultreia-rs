use crate::coordinate::range::Range;
use crate::coordinate::{Coordinate, Offset};
use noise::{NoiseFn, Perlin, Seedable};
use std::ops::Mul;
use strum_macros;

#[derive(PartialEq, Eq, Copy, Clone, strum_macros::EnumIter, strum_macros::EnumCount)]
pub enum TerrainType {
    None,
    Bare,
    Beach,
    Grassland,
    Ice,
    Marsh,
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
    TundraMarsh,
}

impl Default for TerrainType {
    fn default() -> Self {
        Self::None
    }
}

impl TerrainType {
    pub fn new(latitude: f64, elevation: f64, moisture: f64) -> Self {
        if latitude > 89.25 {
            if elevation < 0.1 {
                return TerrainType::Ice;
            }
            return TerrainType::Snow;
        }
        // arctic starts at 66.5
        if latitude > 83. {
            if elevation < 0.1 {
                if moisture > 0.5 {
                    return TerrainType::Ocean;
                }
                return TerrainType::Ice;
            }
            if moisture < 0.1 {
                return TerrainType::Scorched;
            }
            if moisture < 0.2 {
                return TerrainType::Bare;
            }
            if elevation < 0.7 {
                if moisture < 0.7 {
                    return TerrainType::Tundra;
                }
                return TerrainType::TundraMarsh;
            }
            return TerrainType::Snow;
        }

        if elevation < 0.1 {
            return TerrainType::Ocean;
        }
        if elevation < 0.12 {
            if moisture < 0.1 {
                return TerrainType::Scorched;
            }
            if moisture < 0.2 {
                return TerrainType::Beach;
            }
        }
        if elevation > 0.8 {
            if moisture < 0.1 {
                return TerrainType::Scorched;
            }
            if moisture < 0.2 {
                return TerrainType::Bare;
            }
            return TerrainType::Snow;
        }

        if latitude > 67. {
            if moisture < 0.1 {
                return TerrainType::Scorched;
            }
            if moisture < 0.2 {
                return TerrainType::Bare;
            }
            if moisture < 0.7 {
                return TerrainType::Taiga;
            }
            return TerrainType::Marsh;
        }

        if latitude > 30. {
            if elevation > 0.6 {
                if moisture < 0.33 {
                    return TerrainType::TemperateDesert;
                }
                if moisture < 0.66 {
                    return TerrainType::Shrubland;
                }
                return TerrainType::Grassland;
            }

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

    pub fn is_none(&self) -> bool {
        return self.terrain_type == TerrainType::None;
    }

    pub fn terrain_type(&self) -> TerrainType {
        self.terrain_type
    }

    pub fn elevation(&self) -> f64 {
        self.elevation
    }

    pub fn moisture(&self) -> f64 {
        self.moisture
    }
}

pub struct Terrain {
    width: f64,
    height: f64,
    random_latitude: Perlin,
    random_elevation: Perlin,
    random_moisture: Perlin,
}

impl Terrain {
    pub fn new_seeded(rows: usize, columns: usize, seed: u32) -> Self {
        let random_elevation = Perlin::new().set_seed(seed);
        let random_moisture = Perlin::new().set_seed(3 * seed);
        let random_latitude = Perlin::new().set_seed(7 * seed);
        Terrain {
            width: columns as f64,
            height: rows as f64,
            random_latitude,
            random_elevation,
            random_moisture,
        }
    }

    pub fn new(rows: usize, columns: usize) -> Self {
        Terrain::new_seeded(rows, columns, 123)
    }

    fn random_elevation(&self, x: f64, y: f64) -> f64 {
        (self.random_elevation.get([x, y]) + 1.) / 2.
    }

    fn random_moisture(&self, x: f64, y: f64) -> f64 {
        (self.random_moisture.get([x, y]) + 1.) / 2.
    }

    fn smudge_latitude(&self, x: f64, y: f64) -> f64 {
        y + (self.random_latitude.get([x * 4., y * 4.]) * y.abs().max(0.1)) / 10.
    }

    // https://www.redblobgames.com/maps/terrain-from-noise/#islands
    pub fn get(&self, coordinate: &Coordinate) -> TerrainTile {
        let offset: Offset = coordinate.into();
        let x = offset.column() as f64;
        let y = offset.row() as f64;
        let nx = 2.0 * ((x / self.width) - 0.5);
        let hard_ny = 2.0 * ((y / self.height) - 0.5);
        let ny = self.smudge_latitude(nx, hard_ny);
        let elevation: f64 = (self.random_elevation(nx, ny)
            + self.random_elevation(16. * nx, 16. * ny))
        .mul(0.5)
        .powf(3.);
        let moisture: f64 = self.random_moisture(64. * nx, 64. * ny);
        let latitude: f64 = ((ny * std::f64::consts::FRAC_PI_2).sin() * 90.).abs();
        let terrain_type = TerrainType::new(latitude, elevation, moisture);
        TerrainTile::new(elevation, moisture, terrain_type)
    }

    pub fn range(&self, range: &Range) -> Vec<TerrainTile> {
        range.into_iter().map(|c| self.get(c)).collect()
    }

    pub fn minimap(&self, width: u16, height: u16) -> Vec<TerrainType> {
        let mut minimap: Vec<TerrainType> = Vec::with_capacity((width * height) as usize);
        let scale_x = self.width / (width as f64);
        let scale_y = self.height / (height as f64);
        for y in 0..height {
            let row_id = y as usize * width as usize;
            let row = (y as f64 * scale_y) as i32;
            for x in 0..width {
                let idx = row_id + x as usize;
                let column = (x as f64 * scale_x) as i32;
                let coordinate: Coordinate = Offset::new(column, row).into();
                minimap.insert(idx as usize, self.get(&coordinate).terrain_type);
            }
        }
        minimap
    }
}
