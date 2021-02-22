use crate::good::{Good, HarvestableGood, Inventory, NaturalGood};
use crate::map::terrain::latlon::LatLon;
use crate::map::terrain::terrain_factory::terrain_type::TERRAIN_CONSTANTS;
use crate::map::terrain::{Elevation, Latitude, Longitude, Moisture, TerrainType};
use crate::saturating_from::SaturatingInto;
use crate::yields::Yield;
use noise::{NoiseFn, Perlin, Seedable};
use std::collections::HashMap;
use strum::{EnumCount, IntoEnumIterator};

pub type TerrainYields = Inventory<Yield>;

pub struct TerrainYieldsFactory {
    noise: HashMap<Good, Perlin>,
}

impl TerrainYieldsFactory {
    pub fn new(seed: u32) -> Self {
        let mut noise: HashMap<Good, Perlin> = HashMap::new();
        for (idx, good) in NaturalGood::iter().enumerate() {
            noise.insert(
                Good::NaturalGood(good),
                Perlin::new().set_seed(seed + (idx as u32)),
            );
        }
        for (idx, good) in HarvestableGood::iter().enumerate() {
            noise.insert(
                Good::HarvestableGood(good),
                Perlin::new().set_seed(seed + (NaturalGood::COUNT + idx) as u32),
            );
        }
        TerrainYieldsFactory { noise }
    }

    fn random(
        &self,
        good: &Good,
        latitude: &Latitude,
        longitude: &Longitude,
        base_noise: f64,
        num_harmonics: usize,
    ) -> f64 {
        if num_harmonics == 0 {
            return 0.;
        }
        self.noise.get(good).map_or(0., |perlin| {
            let mut value = 0.;
            for harmonic in 0..num_harmonics {
                let noise = base_noise * (usize::pow(2, harmonic as u32) as f64);
                value += perlin.get([
                    noise * latitude.normalized(),
                    noise * longitude.normalized(),
                ]) / (num_harmonics as f64);
            }
            value
        })
    }

    fn random_harvestable(
        &self,
        good: &HarvestableGood,
        latitude: &Latitude,
        longitude: &Longitude,
        base_noise: f64,
        num_harmonics: usize,
    ) -> f64 {
        self.random(
            &Good::HarvestableGood(*good),
            latitude,
            longitude,
            base_noise,
            num_harmonics,
        )
    }

    fn random_natural(
        &self,
        good: &NaturalGood,
        latitude: &Latitude,
        longitude: &Longitude,
        base_noise: f64,
        num_harmonics: usize,
    ) -> f64 {
        self.random(
            &Good::NaturalGood(*good),
            latitude,
            longitude,
            base_noise,
            num_harmonics,
        )
    }

    pub fn create(
        &self,
        latitude: Latitude,
        longitude: Longitude,
        _elevation: Elevation,
        moisture: Moisture,
        terrain_type: &TerrainType,
    ) -> TerrainYields {
        let mut yields = TerrainYields::new();
        for good in NaturalGood::iter() {
            let rand = |base_noise: f64, num_harmonics: usize| {
                self.random_natural(&good, &latitude, &longitude, base_noise, num_harmonics)
            };
            let yield_f64 = match good {
                NaturalGood::FreshWater if terrain_type == &TerrainType::FreshWater => {
                    1. - ((1. - Into::<f64>::into(moisture))
                        / (1. - Into::<f64>::into(TERRAIN_CONSTANTS.freshwater_moisture_threshold)))
                    // bias towards 100%
                    .powf(5.)
                }
                NaturalGood::ClayRepo if terrain_type.is_ground() => {
                    let mut productivity: f64 = moisture.into();
                    if terrain_type.is_hill() {
                        productivity *= 0.8;
                    }
                    productivity * rand(16., 1)
                }
                NaturalGood::CoalRepo => {
                    let productivity = {
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
                    productivity * rand(2., 1)
                }
                NaturalGood::CopperOreRepo if terrain_type.is_mountain() => rand(3., 2),
                NaturalGood::GemStoneRepo if terrain_type.is_mountain() => rand(6., 3),
                NaturalGood::IronOreRepo if terrain_type.is_mountain() => rand(1., 2),
                NaturalGood::MarbleRepo => {
                    let productivity = if terrain_type.is_mountain() {
                        1.
                    } else if terrain_type.is_hill_with_snow() {
                        0.5
                    } else if terrain_type.is_hill() {
                        0.75
                    } else {
                        0.
                    };
                    productivity * rand(1., 2)
                }
                NaturalGood::SaltRepo => {
                    if terrain_type.is_mountain() {
                        rand(1., 2)
                    } else if terrain_type == &TerrainType::SaltFlat {
                        1. / Into::<f64>::into(moisture)
                    } else {
                        0.
                    }
                }
                NaturalGood::SilverOreRepo if terrain_type.is_mountain() => rand(5., 3),
                NaturalGood::StoneRepo => {
                    if terrain_type.is_mountain() {
                        1.
                    } else if terrain_type.is_hill_with_snow() {
                        rand(1., 1).powf(0.25) / 1.5
                    } else if terrain_type.is_hill() {
                        rand(1., 1).powf(0.25)
                    } else {
                        0.
                    }
                }
                NaturalGood::Whale if terrain_type.is_ocean() && latitude.abs() > 70. => {
                    let productivity = Into::<f64>::into(moisture).powf(2.);
                    productivity * rand(32., 8)
                }
                NaturalGood::WildFish if terrain_type.is_water() => {
                    let productivity = Into::<f64>::into(moisture).powf(2.);
                    productivity * rand(128., 1)
                }
                _ => 0.,
            };
            if yield_f64 > 0.1 {
                yields.insert(Good::NaturalGood(good), yield_f64.saturating_into());
            }
        }

        for good in HarvestableGood::iter() {
            let rand = |base_noise: f64, num_harmonics: usize| {
                self.random_harvestable(&good, &latitude, &longitude, base_noise, num_harmonics)
            };
            let yield_f64 = match good {
                HarvestableGood::Game => {
                    let productivity = match terrain_type {
                        TerrainType::WoodedHills => 0.75,
                        TerrainType::Taiga => 0.65,
                        TerrainType::TaigaHills => 0.45,
                        _ if terrain_type.is_rainforest() => 0.85,
                        _ if terrain_type.is_wooded() => 1.,
                        _ => 0.,
                    };
                    productivity * rand(32., 3)
                }
                HarvestableGood::Tree => {
                    let productivity = match terrain_type {
                        TerrainType::WoodedHills => 0.75,
                        TerrainType::Taiga => 0.65,
                        TerrainType::TaigaHills => 0.45,
                        _ if terrain_type.is_rainforest() => 1.,
                        _ if terrain_type.is_wooded() => 0.9,
                        _ => 0.,
                    };
                    productivity * rand(1., 1).powf(0.25)
                }
                HarvestableGood::Cattle => {
                    let productivity = match terrain_type {
                        TerrainType::Grassland => 1.,
                        TerrainType::Hills => 0.75,
                        TerrainType::Tundra => 0.3,
                        _ => 0.,
                    };
                    productivity * rand(128., 1)
                }
                HarvestableGood::CocoaPlant
                    if latitude.abs() < 30. && terrain_type.is_flat_ground() =>
                {
                    rand(96., 5)
                }
                HarvestableGood::CottonPlant
                    if latitude.abs() < 30. && terrain_type.is_flat_ground() =>
                {
                    rand(8., 4)
                }
                HarvestableGood::Ears
                    if latitude.abs() > 30.
                        && latitude.abs() < 65.
                        && terrain_type.is_flat_ground() =>
                {
                    rand(8., 1)
                }
                HarvestableGood::FlowerPlant
                    if latitude.abs() < 80. && terrain_type.is_ground() =>
                {
                    let productivity = if terrain_type.is_hill() {
                        0.75 * Into::<f64>::into(moisture)
                    } else {
                        moisture.into()
                    };
                    productivity * rand(128., 1)
                }
                HarvestableGood::Grape
                    if latitude.abs() > 35. && latitude.abs() < 60. && terrain_type.is_ground() =>
                {
                    let productivity = if !terrain_type.is_hill() {
                        0.75 * Into::<f64>::into(moisture)
                    } else {
                        moisture.into()
                    };
                    productivity * rand(256., 2)
                }
                HarvestableGood::HempPlant if terrain_type.is_ground() && latitude.abs() < 70. => {
                    let productivity = if terrain_type.is_hill() {
                        0.75 * Into::<f64>::into(moisture)
                    } else {
                        moisture.into()
                    };
                    productivity * rand(1., 1)
                }
                HarvestableGood::HopsPlant
                    if latitude.abs() > 35. && latitude.abs() < 60. && terrain_type.is_ground() =>
                {
                    let productivity = if !terrain_type.is_hill() {
                        0.75 * Into::<f64>::into(moisture)
                    } else {
                        moisture.into()
                    };
                    productivity * rand(128., 3)
                }
                HarvestableGood::IndigoPlant
                    if latitude.abs() < 30. && terrain_type.is_flat_ground() =>
                {
                    let productivity: f64 = moisture.into();
                    productivity * rand(512., 6)
                }
                HarvestableGood::PeltAnimal
                    if terrain_type.is_ground()
                        && (latitude.abs() < 10.
                            || (latitude.abs() > 70. && latitude.abs() < 85.)) =>
                {
                    let productivity = if terrain_type.is_hill() { 0.75 } else { 1. };
                    productivity * rand(256., 6)
                }
                HarvestableGood::PotatoPlant if terrain_type.is_flat_ground() => {
                    let productivity: f64 = moisture.into();
                    productivity * rand(1., 1)
                }
                HarvestableGood::Sheep
                    if latitude.abs() > 15. && latitude.abs() < 70. && terrain_type.is_ground() =>
                {
                    let productivity = if terrain_type.is_hill() { 0.75 } else { 1. };
                    productivity * rand(1., 1)
                }
                HarvestableGood::SilkWorm
                    if latitude.abs() > 10.
                        && latitude.abs() < 35.
                        && longitude.abs() > 100.
                        && terrain_type.is_flat_ground() =>
                {
                    rand(512., 6)
                }
                HarvestableGood::SpicePlant
                    if latitude.abs() < 35. && terrain_type.is_flat_ground() =>
                {
                    rand(3., 3) * (1. / Into::<f64>::into(moisture))
                }
                HarvestableGood::SugarCanePlant
                    if latitude.abs() > 10. && latitude.abs() < 35. && terrain_type.is_ground() =>
                {
                    let productivity: f64 = moisture.into();
                    productivity * rand(128., 2)
                }
                HarvestableGood::TobaccoPlant
                    if latitude.abs() < 47. && terrain_type.is_flat_ground() =>
                {
                    let productivity: f64 = moisture.into();
                    productivity * rand(128., 2)
                }
                HarvestableGood::UntamedHorse
                    if latitude.abs() > 30.
                        && latitude.abs() < 70.
                        && longitude.abs() > 100.
                        && terrain_type.is_flat_ground() =>
                {
                    rand(32., 2)
                }
                _ => 0.,
            };
            if yield_f64 > 0.1 {
                yields.insert(Good::HarvestableGood(good), yield_f64.saturating_into());
            }
        }
        yields
    }
}
