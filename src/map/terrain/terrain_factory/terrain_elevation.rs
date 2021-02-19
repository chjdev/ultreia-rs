use crate::saturating_from::SaturatingInto;
use derive_more::Into;
use noise::{NoiseFn, Perlin, Seedable};
use std::cmp::Ordering;
use std::ops::Mul;

#[derive(PartialEq, PartialOrd, Copy, Clone, Default, Into)]
pub struct Elevation(f64);

impl PartialEq<f64> for Elevation {
    fn eq(&self, other: &f64) -> bool {
        Into::<f64>::into(*self).eq(other)
    }
}

impl PartialOrd<f64> for Elevation {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        Into::<f64>::into(*self).partial_cmp(other)
    }
}

impl SaturatingInto<Elevation> for f64 {
    fn saturating_from(elevation: &f64) -> Elevation {
        Elevation(elevation.max(0.))
    }
}

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

    pub fn create(&self, nx: f64, ny: f64) -> Elevation {
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
        elevation.saturating_into()
    }
}
