mod terrain_tile;
mod terrain_type;
mod terrain_yields;

use crate::coordinate::range::Range;
use crate::coordinate::{Coordinate, Offset};
use noise::{NoiseFn, Perlin, Seedable};
use std::ops::Mul;
pub use terrain_tile::TerrainTile;
pub use terrain_type::TerrainType;
pub use terrain_yields::{TerrainYields, Yield};

pub struct Terrain {
    width: f64,
    height: f64,
    island_noise: f64,
    random_latitude: Perlin,
    random_elevation: Perlin,
    random_moisture: Perlin,
}

impl Terrain {
    pub fn new_seeded(rows: usize, columns: usize, island_noise: f64, seed: u32) -> Self {
        let random_elevation = Perlin::new().set_seed(seed);
        let random_moisture = Perlin::new().set_seed(3 * seed);
        let random_latitude = Perlin::new().set_seed(7 * seed);
        Terrain {
            width: columns as f64,
            height: rows as f64,
            island_noise,
            random_latitude,
            random_elevation,
            random_moisture,
        }
    }

    pub fn new(rows: usize, columns: usize, island_noise: f64) -> Self {
        Terrain::new_seeded(rows, columns, island_noise, 1234)
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
        // offset 0,0 to middle of width/height
        let x = offset.column() as f64 + self.width / 2.;
        let y = offset.row() as f64 + self.height / 2.;
        let nx = 2.0 * ((x / self.width) - 0.5);
        let true_ny = 2.0 * ((y / self.height) - 0.5);
        let smudged_ny = self.smudge_latitude(nx, true_ny);
        let mut elevation: f64 = (self.random_elevation(nx, smudged_ny)
            + self.random_elevation(self.island_noise * nx, self.island_noise * smudged_ny))
        .mul(0.5)
        .powf(3.);
        if elevation > 0.12 {
            elevation = self
                .random_elevation(
                    nx * self.island_noise.powf(2.),
                    smudged_ny * self.island_noise.powf(2.),
                )
                .powf(3.)
                .max(0.12);
        }
        let smudged_latitude: f64 = (smudged_ny * std::f64::consts::FRAC_PI_2).sin() * 90.;
        let moisture: f64 = self
            .random_moisture(
                self.island_noise * 4. * nx,
                self.island_noise * 4. * smudged_ny,
            )
            .mul(1.1)
            // tropics no desert
            .max(if smudged_latitude.abs() < 7.5 {
                0.1
            } else {
                0.
            });
        let true_longitude: f64 = nx * 180.;
        TerrainTile::new(smudged_latitude, true_longitude, elevation, moisture)
    }

    pub fn range(&self, range: &Range) -> Vec<TerrainTile> {
        range.into_iter().map(|c| self.get(c)).collect()
    }

    pub fn minimap(&self, width: u16, height: u16) -> Vec<TerrainType> {
        // with_capacity does not work in godot context for some reason
        let mut minimap: Vec<TerrainType> = Vec::new();
        let scale_x = self.width / (width as f64);
        let scale_y = self.height / (height as f64);
        let height_half = (height / 2) as i16;
        let width_half = (width / 2) as i16;
        for y in -height_half..height_half {
            let row_id = (y + height_half) as usize * width as usize;
            let row = (y as f64 * scale_y) as i32;
            for x in -width_half..width_half {
                let idx = row_id + (x + width_half) as usize;
                let column = (x as f64 * scale_x) as i32;
                let coordinate: Coordinate = Offset::new(column, row).into();
                minimap.insert(idx as usize, self.get(&coordinate).terrain_type());
            }
        }
        minimap
    }
}
