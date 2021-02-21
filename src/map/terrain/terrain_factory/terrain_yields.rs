use crate::good::{Good, HarvestableGood, Inventory, NaturalGood};
use crate::map::terrain::terrain_factory::terrain_type::TERRAIN_CONSTANTS;
use crate::map::terrain::{Elevation, Latitude, Longitude, Moisture, TerrainType};
use crate::saturating_from::SaturatingInto;
use crate::yields::Yield;
use strum::IntoEnumIterator;

pub type TerrainYields = Inventory<Yield>;

pub struct TerrainYieldsFactory {}

impl TerrainYieldsFactory {
    pub fn new() -> Self {
        TerrainYieldsFactory {}
    }

    pub fn create(
        &self,
        latitude: Latitude,
        _longitude: Longitude,
        _elevation: Elevation,
        moisture: Moisture,
        terrain_type: &TerrainType,
    ) -> TerrainYields {
        let mut yields = TerrainYields::new();
        for good in NaturalGood::iter() {
            let yield_f64 = match good {
                NaturalGood::FreshWater if terrain_type == &TerrainType::FreshWater => {
                    1. - ((1. - Into::<f64>::into(moisture))
                        / (1. - Into::<f64>::into(TERRAIN_CONSTANTS.freshwater_moisture_threshold)))
                    // bias towards 100%
                    .powf(5.)
                }
                NaturalGood::ClayRepo if terrain_type.is_ground() => {
                    let mut mut_yield_f64: f64 = moisture.into();
                    if terrain_type.is_hill() {
                        mut_yield_f64 *= 0.8;
                    }
                    mut_yield_f64
                }
                NaturalGood::CoalRepo => {
                    let modifier = {
                        if terrain_type.is_hill_with_snow() {
                            0.75
                        } else if terrain_type.is_hill() {
                            1.
                        } else if terrain_type.is_mountain() {
                            0.75
                        } else {
                            0.
                        }
                    };
                    modifier
                }
                NaturalGood::CopperOreRepo if terrain_type.is_mountain() => 1.,
                NaturalGood::GemStoneRepo if terrain_type.is_mountain() => 1.,
                NaturalGood::IronOreRepo if terrain_type.is_mountain() => 1.,
                NaturalGood::MarbleRepo => {
                    if terrain_type.is_mountain() {
                        1.
                    } else if terrain_type.is_hill_with_snow() {
                        0.5
                    } else if terrain_type.is_hill() {
                        0.75
                    } else {
                        0.
                    }
                }
                NaturalGood::SaltRepo => {
                    if terrain_type.is_mountain() {
                        1.
                    } else if terrain_type == &TerrainType::SaltFlat {
                        1.
                    } else {
                        0.
                    }
                }
                NaturalGood::SilverOreRepo if terrain_type.is_mountain() => 1.,
                NaturalGood::StoneRepo => {
                    if terrain_type.is_mountain() {
                        1.
                    } else if terrain_type.is_hill_with_snow() {
                        0.5
                    } else if terrain_type.is_hill() {
                        0.75
                    } else {
                        0.
                    }
                }
                NaturalGood::Whale if terrain_type.is_ocean() && latitude.abs() > 70. => 1.,
                NaturalGood::WildFish if terrain_type.is_water() => 0.6,
                _ => 0.,
            };
            if yield_f64 > 0. {
                yields.insert(Good::NaturalGood(good), yield_f64.saturating_into());
            }
        }

        for good in HarvestableGood::iter() {
            let yield_f64 = match good {
                HarvestableGood::Game => match terrain_type {
                    TerrainType::WoodedHills => 0.75,
                    TerrainType::Taiga => 0.65,
                    TerrainType::TaigaHills => 0.45,
                    _ if terrain_type.is_rainforest() => 0.85,
                    _ if terrain_type.is_wooded() => 1.,
                    _ => 0.,
                },
                HarvestableGood::Tree => match terrain_type {
                    TerrainType::WoodedHills => 0.75,
                    TerrainType::Taiga => 0.65,
                    TerrainType::TaigaHills => 0.45,
                    _ if terrain_type.is_rainforest() => 1.,
                    _ if terrain_type.is_wooded() => 0.9,
                    _ => 0.,
                },
                HarvestableGood::Cattle => match terrain_type {
                    TerrainType::Grassland => 1.,
                    TerrainType::Hills => 0.75,
                    TerrainType::Tundra => 0.3,
                    _ => 0.,
                },
                HarvestableGood::CocoaPlant if latitude.abs() < 30. => 1.,
                HarvestableGood::CottonPlant => 1.,
                HarvestableGood::Ears => 1.,
                HarvestableGood::FlowerPlant => 1.,
                HarvestableGood::Grape => 1.,
                HarvestableGood::HempPlant => 1.,
                HarvestableGood::HopsPlant => 1.,
                HarvestableGood::IndigoPlant => 1.,
                HarvestableGood::PeltAnimal => 1.,
                HarvestableGood::PotatoPlant => 1.,
                HarvestableGood::Sheep => 1.,
                HarvestableGood::SilkWorm => 1.,
                HarvestableGood::SpicePlant => 1.,
                HarvestableGood::SugarCanePlant => 1.,
                HarvestableGood::TobaccoPlant => 1.,
                HarvestableGood::UntamedHorse => 1.,
                _ => 0.,
            };
            if yield_f64 > 0. {
                yields.insert(Good::HarvestableGood(good), yield_f64.saturating_into());
            }
        }
        yields
    }
}
