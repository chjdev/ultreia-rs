use crate::saturating_from::SaturatingInto;
use derive_more::Into;
use noise::{NoiseFn, Perlin, Seedable};
use std::cmp::Ordering;
use std::ops::Mul;

#[derive(PartialEq, PartialOrd, Copy, Clone, Default, Into)]
pub struct Moisture(f64);

impl Moisture {
    const fn new(moisture: f64) -> Self {
        Moisture(moisture)
    }
}

impl PartialEq<f64> for Moisture {
    fn eq(&self, other: &f64) -> bool {
        Into::<f64>::into(*self).eq(other)
    }
}

impl PartialOrd<f64> for Moisture {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        Into::<f64>::into(*self).partial_cmp(other)
    }
}

impl SaturatingInto<Moisture> for f64 {
    fn saturating_from(moisture: &f64) -> Moisture {
        Moisture::new(moisture.max(0.))
    }
}

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

    pub fn create(&self, nx: f64, ny: f64) -> Moisture {
        self.random_moisture(self.moisture_noise * nx, self.moisture_noise * ny)
            .mul(1.1)
            // tropics no desert
            .max(if ny.abs() < 0.083 { 0.1 } else { 0. })
            .saturating_into()
    }
}
