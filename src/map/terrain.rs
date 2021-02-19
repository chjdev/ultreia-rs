mod latlon;
mod terrain_factory;

use crate::coordinate::range::Range;
use crate::coordinate::{Coordinate, Offset};
pub use latlon::{Latitude, Longitude};
use noise::{NoiseFn, Perlin, Seedable};
use terrain_factory::TerrainFactory;
pub use terrain_factory::{TerrainMeta, TerrainType, TerrainYields, Yield};

pub struct Terrain {
    width: f64,
    height: f64,
    tile_factory: TerrainFactory,
    random_latitude: Perlin,
}

impl Terrain {
    pub fn new_seeded(seed: u32, rows: usize, columns: usize, island_noise: f64) -> Self {
        let random_latitude = Perlin::new().set_seed(7 * seed);
        Terrain {
            width: columns as f64,
            height: rows as f64,
            random_latitude,
            tile_factory: TerrainFactory::new(seed, island_noise),
        }
    }

    pub fn new(rows: usize, columns: usize, island_noise: f64) -> Self {
        Terrain::new_seeded(1234, rows, columns, island_noise)
    }

    fn smudge_latitude(&self, x: f64, y: f64) -> f64 {
        y + (self.random_latitude.get([x * 4., y * 4.]) * y.abs().max(0.1)) / 10.
    }

    // https://www.redblobgames.com/maps/terrain-from-noise/#islands
    pub fn get(&self, coordinate: &Coordinate) -> TerrainMeta {
        let offset: Offset = coordinate.into();
        // offset 0,0 to middle of width/height
        let x = offset.column() as f64 + self.width / 2.;
        let y = offset.row() as f64 + self.height / 2.;
        let nx = 2.0 * ((x / self.width) - 0.5);
        let true_ny = 2.0 * ((y / self.height) - 0.5);
        let smudged_ny = self.smudge_latitude(nx, true_ny);
        self.tile_factory.create(nx, smudged_ny)
    }

    pub fn range(&self, range: &Range) -> Vec<TerrainMeta> {
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
