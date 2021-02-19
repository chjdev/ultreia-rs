use noise::{NoiseFn, Perlin, Seedable};
use std::ops::Mul;

pub struct TerrainElevationFactory {
    random_elevation: Perlin,
    island_noise: f64,
}

impl TerrainElevationFactory {
    pub fn new(seed: u32, island_noise: f64) -> Self {
        let random_elevation = Perlin::new().set_seed(seed);
        TerrainElevationFactory {
            random_elevation,
            island_noise,
        }
    }

    fn random_elevation(&self, x: f64, y: f64) -> f64 {
        (self.random_elevation.get([x, y]) + 1.) / 2.
    }

    pub fn create(&self, nx: f64, ny: f64) -> f64 {
        let mut elevation: f64 = (self.random_elevation(nx, ny)
            + self.random_elevation(self.island_noise * nx, self.island_noise * ny))
        .mul(0.5)
        .powf(3.);
        if elevation > 0.12 {
            elevation = self
                .random_elevation(
                    nx * self.island_noise.powf(2.),
                    ny * self.island_noise.powf(2.),
                )
                .powf(3.)
                .max(0.12);
        }
        elevation
    }
}
