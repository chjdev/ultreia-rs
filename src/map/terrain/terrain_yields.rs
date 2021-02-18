use crate::good::{Good, HarvestableGood, Inventory, NaturalGood};
use crate::map::terrain::terrain_type::{
    FRESHWATER_MOISTURE_THRESHOLD, HILL_ELEVATION_THRESHOLD, MOUNTAIN_ELEVATION_THRESHOLD,
    OCEAN_ELEVATION_THRESHOLD,
};
use crate::map::terrain::TerrainType;
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
    pub fn new(
        _latitude: f64,
        _longitude: f64,
        elevation: f64,
        moisture: f64,
        terrain_type: &TerrainType,
    ) -> Self {
        let mut yields = Inventory::<Yield>::new();

        // using match so we can't forget goods
        for good in NaturalGood::iter() {
            let mut yield_f64 = 0.;
            match good {
                NaturalGood::FreshWater => {
                    if terrain_type == &TerrainType::FreshWater {
                        yield_f64 = 1.
                            - ((1. - moisture) / (1. - FRESHWATER_MOISTURE_THRESHOLD))
                                // bias towards 100%
                                .powf(5.);
                    }
                }
                NaturalGood::ClayRepo => {
                    if elevation > OCEAN_ELEVATION_THRESHOLD
                        && elevation < MOUNTAIN_ELEVATION_THRESHOLD
                        && terrain_type != &TerrainType::FreshWater
                        && terrain_type != &TerrainType::Snow
                        && terrain_type != &TerrainType::Marsh
                    {
                        yield_f64 = moisture;
                        if elevation > HILL_ELEVATION_THRESHOLD {
                            yield_f64 *= 0.8;
                        }
                        // todo clay noise
                    }
                }
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

        Self(yields)
    }

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
