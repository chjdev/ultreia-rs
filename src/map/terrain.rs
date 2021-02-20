pub mod latlon;
mod terrain_factory;

use crate::coordinate::{Coordinate, Offset};
use crate::map::minimap::{GetByCoordinate, Minimap, WithSize};
pub use latlon::{Latitude, Longitude};
use noise::{NoiseFn, Perlin, Seedable};
use terrain_factory::TerrainFactory;
pub use terrain_factory::{Elevation, Moisture, TerrainMeta, TerrainType, TerrainYields};

pub struct Terrain {
    rows: usize,
    columns: usize,
    tile_factory: TerrainFactory,
    random_latitude: Perlin,
}

impl Terrain {
    pub fn new_seeded(seed: u32, rows: usize, columns: usize, island_noise: f64) -> Self {
        let random_latitude = Perlin::new().set_seed(7 * seed);
        Terrain {
            rows,
            columns,
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
}

impl WithSize for Terrain {
    fn rows(&self) -> usize {
        self.rows
    }

    fn columns(&self) -> usize {
        self.columns
    }
}

impl GetByCoordinate<TerrainMeta> for Terrain {
    // https://www.redblobgames.com/maps/terrain-from-noise/#islands
    fn get(&self, coordinate: &Coordinate) -> TerrainMeta {
        let offset: Offset = coordinate.into();
        // offset 0,0 to middle of width/height
        let x = offset.column() as f64 + self.columns() as f64 / 2.;
        let y = offset.row() as f64 + self.rows() as f64 / 2.;
        let nx = 2.0 * ((x / self.columns() as f64) - 0.5);
        let true_ny = 2.0 * ((y / self.rows() as f64) - 0.5);
        let smudged_ny = self.smudge_latitude(nx, true_ny);
        self.tile_factory.create(nx, smudged_ny)
    }
}

impl GetByCoordinate<TerrainType> for Terrain {
    // https://www.redblobgames.com/maps/terrain-from-noise/#islands
    fn get(&self, coordinate: &Coordinate) -> TerrainType {
        let terrain_meta: TerrainMeta = self.get(coordinate);
        terrain_meta.terrain_type()
    }
}

impl Minimap<TerrainType> for Terrain {}
