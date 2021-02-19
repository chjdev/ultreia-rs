use noise::{NoiseFn, Perlin, Seedable};
use std::ops::Mul;

pub struct TerrainMoistureFactory {
    random_moisture: Perlin,
    moisture_noise: f64,
}

impl TerrainMoistureFactory {
    pub fn new(seed: u32, moisture_noise: f64) -> Self {
        let random_moisture = Perlin::new().set_seed(seed);
        TerrainMoistureFactory {
            random_moisture,
            moisture_noise,
        }
    }

    fn random_moisture(&self, x: f64, y: f64) -> f64 {
        (self.random_moisture.get([x, y]) + 1.) / 2.
    }

    pub fn create(&self, nx: f64, ny: f64) -> f64 {
        self.random_moisture(self.moisture_noise * nx, self.moisture_noise * ny)
            .mul(1.1)
            // tropics no desert
            .max(if ny.abs() < 0.083 { 0.1 } else { 0. })
    }
}
