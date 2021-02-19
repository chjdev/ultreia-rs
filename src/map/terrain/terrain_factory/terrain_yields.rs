use super::terrain_type::TERRAIN_CONSTANTS;
use crate::good::{Good, HarvestableGood, Inventory, NaturalGood};
use crate::map::terrain::{Elevation, Latitude, Longitude, Moisture, TerrainType};
use std::convert::TryFrom;
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Yield(u8);

const PERCENT100_YIELD: f64 = (u8::max_value() / 2) as f64;
const PERCENT200_YIELD: f64 = u8::max_value() as f64;

impl Yield {
    pub fn percent(&self) -> f64 {
        (self.0 as f64) / PERCENT100_YIELD
    }

    pub fn saturating_from(value: f64) -> Self {
        Yield((value.clamp(0., 2.) * PERCENT100_YIELD) as u8)
    }
}

impl Into<f64> for Yield {
    fn into(self) -> f64 {
        self.percent()
    }
}

impl TryFrom<f64> for Yield {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0. || value > 2. {
            Err("value outside of yield range")
        } else {
            Ok(Yield((value * PERCENT100_YIELD) as u8))
        }
    }
}

#[derive(Default, Eq)]
pub struct TerrainYields(Inventory<Yield>);

impl TerrainYields {
    pub fn yields(&self) -> &Inventory<Yield> {
        &self.0
    }
}

impl PartialEq for TerrainYields {
    fn eq(&self, other: &Self) -> bool {
        other.yields().len() == self.yields().len()
            && other
                .yields()
                .keys()
                .all(|k| other.yields().get(k) == (self.yields().get(k)))
    }
}

pub struct TerrainYieldsFactory {}

impl TerrainYieldsFactory {
    pub fn new() -> Self {
        TerrainYieldsFactory {}
    }

    pub fn create(
        &self,
        _latitude: Latitude,
        _longitude: Longitude,
        elevation: Elevation,
        moisture: Moisture,
        terrain_type: &TerrainType,
    ) -> TerrainYields {
        let mut yields = Inventory::<Yield>::new();
        // using match so we can't forget goods
        for good in NaturalGood::iter() {
            let mut yield_f64 = 0.;
            match good {
                NaturalGood::FreshWater => {
                    if terrain_type == &TerrainType::FreshWater {
                        yield_f64 = 1.
                            - ((1. - Into::<f64>::into(moisture))
                                / (1.
                                    - Into::<f64>::into(
                                        TERRAIN_CONSTANTS.freshwater_moisture_threshold,
                                    )))
                            // bias towards 100%
                            .powf(5.);
                    }
                }
                NaturalGood::ClayRepo => {
                    if elevation > TERRAIN_CONSTANTS.ocean_elevation_threshold
                        && elevation < TERRAIN_CONSTANTS.mountain_elevation_threshold
                        && terrain_type != &TerrainType::FreshWater
                        && terrain_type != &TerrainType::Snow
                        && terrain_type != &TerrainType::Marsh
                    {
                        yield_f64 = moisture.into();
                        if elevation > TERRAIN_CONSTANTS.hill_elevation_threshold {
                            yield_f64 *= 0.8;
                        }
                        // todo clay noise
                    }
                }
                NaturalGood::CoalRepo => {}
                //todo remove
                _ => (),
            };
            if yield_f64 > 0. {
                yields.insert(Good::NaturalGood(good), Yield::saturating_from(yield_f64));
            }
        }

        for good in HarvestableGood::iter() {
            let mut yield_f64 = 0.;
            match good {
                HarvestableGood::Game => {
                    yield_f64 = match terrain_type {
                        TerrainType::TropicalSeasonalForest
                        | TerrainType::TemperateDeciduousForest => 1.,
                        TerrainType::TropicalRainForest | TerrainType::TemperateRainForest => 0.85,
                        TerrainType::WoodedHills => 0.75,
                        TerrainType::Taiga => 0.65,
                        TerrainType::TaigaHills => 0.45,
                        _ => 0.,
                    }
                }
                HarvestableGood::Tree => {
                    yield_f64 = match terrain_type {
                        TerrainType::TropicalRainForest | TerrainType::TemperateRainForest => 1.,
                        TerrainType::TropicalSeasonalForest
                        | TerrainType::TemperateDeciduousForest => 0.9,
                        TerrainType::WoodedHills => 0.75,
                        TerrainType::Taiga => 0.65,
                        TerrainType::TaigaHills => 0.45,
                        _ => 0.,
                    }
                }
                //todo remove
                _ => (),
            }
            if yield_f64 > 0. {
                yields.insert(
                    Good::HarvestableGood(good),
                    Yield::saturating_from(yield_f64),
                );
            }
        }

        TerrainYields(yields)
    }
}
