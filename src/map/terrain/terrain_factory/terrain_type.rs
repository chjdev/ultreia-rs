use crate::map::terrain::{Elevation, Latitude, Moisture};
use crate::saturating_from::SaturatingInto;
use strum_macros::{EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

#[derive(PartialEq, Eq, Copy, Clone, EnumIter, EnumCount, IntoStaticStr, EnumVariantNames)]
pub enum TerrainType {
    Bare,
    Grassland,
    Ice,
    Marsh,
    Ocean,
    Scorched,
    Shrubland,
    Snow,
    SubtropicalDesert,
    Taiga,
    TemperateDeciduousForest,
    TemperateDesert,
    TemperateRainForest,
    TropicalRainForest,
    TropicalSeasonalForest,
    Tundra,
    TundraMarsh,
    DesertMountain,
    Mountain,
    WoodedHills,
    TaigaHills,
    SnowHills,
    DesertHills,
    Hills,
    FreshWater,
    SaltFlat,
}

impl TerrainType {
    pub fn is_ocean(&self) -> bool {
        self == &Self::Ocean
    }
    pub fn is_water(&self) -> bool {
        self.is_ocean() || self == &Self::FreshWater
    }
    pub fn is_hill_with_snow(&self) -> bool {
        match self {
            Self::TaigaHills | Self::SnowHills => true,
            _ => false,
        }
    }
    pub fn is_hill(&self) -> bool {
        match self {
            Self::Hills
            | Self::WoodedHills
            | Self::TaigaHills
            | Self::SnowHills
            | Self::DesertHills => true,
            _ => false,
        }
    }
    pub fn is_mountain(&self) -> bool {
        match self {
            Self::Mountain | Self::DesertMountain => true,
            _ => false,
        }
    }
    pub fn is_rainforest(&self) -> bool {
        match self {
            Self::TropicalRainForest | Self::TemperateRainForest => true,
            _ => false,
        }
    }
    pub fn is_wooded(&self) -> bool {
        match self {
            Self::TropicalRainForest
            | Self::TemperateRainForest
            | Self::TemperateDeciduousForest
            | Self::TropicalSeasonalForest => true,
            _ => false,
        }
    }
    pub fn is_ground(&self) -> bool {
        match self {
            Self::Ocean | Self::FreshWater | Self::Marsh | Self::Ice | Self::TundraMarsh => false,
            _ => self.is_mountain(),
        }
    }
    pub fn is_flat_ground(&self) -> bool {
        !self.is_hill() && self.is_ground()
    }
}

pub struct TerrainConstants {
    pub freshwater_moisture_threshold: Moisture,
    pub hill_elevation_threshold: Elevation,
    pub mountain_elevation_threshold: Elevation,
    pub ocean_elevation_threshold: Elevation,
    pub saltflat_elevation_threshold: Elevation,
}

// not individual constants since const_fn is unstable for initialization
lazy_static! {
    pub static ref TERRAIN_CONSTANTS: TerrainConstants = TerrainConstants {
        freshwater_moisture_threshold: SaturatingInto::saturating_from(&0.87),
        hill_elevation_threshold: SaturatingInto::saturating_from(&0.55),
        mountain_elevation_threshold: SaturatingInto::saturating_from(&0.75),
        ocean_elevation_threshold: SaturatingInto::saturating_from(&0.1),
        saltflat_elevation_threshold: SaturatingInto::saturating_from(&0.12),
    };
}

impl Default for TerrainType {
    fn default() -> Self {
        Self::Bare
    }
}

pub struct TerrainTypeFactory;

impl TerrainTypeFactory {
    pub fn new() -> Self {
        TerrainTypeFactory {}
    }

    pub fn create(
        &self,
        latitude: Latitude,
        elevation: Elevation,
        moisture: Moisture,
    ) -> TerrainType {
        if elevation > TERRAIN_CONSTANTS.mountain_elevation_threshold {
            if moisture < 0.1 {
                return TerrainType::DesertMountain;
            }
            return TerrainType::Mountain;
        }
        let base_terrain_type = Self::base_terrain_type(latitude, elevation, moisture);
        if elevation > TERRAIN_CONSTANTS.hill_elevation_threshold {
            if moisture < 0.1 {
                return TerrainType::DesertHills;
            }
            return match base_terrain_type {
                TerrainType::TemperateDeciduousForest
                | TerrainType::TemperateRainForest
                | TerrainType::TropicalSeasonalForest
                | TerrainType::TropicalRainForest => TerrainType::WoodedHills,
                TerrainType::Taiga => TerrainType::TaigaHills,
                TerrainType::Snow => TerrainType::SnowHills,
                _ => TerrainType::Hills,
            };
        }
        return base_terrain_type;
    }

    fn base_terrain_type(
        latitude: Latitude,
        elevation: Elevation,
        moisture: Moisture,
    ) -> TerrainType {
        let abs_latitude: f64 = Into::<f64>::into(latitude).abs();
        if abs_latitude > 89.25 {
            if elevation < 0.1 {
                return TerrainType::Ice;
            }
            return TerrainType::Snow;
        }
        if elevation > 0.2 && moisture > TERRAIN_CONSTANTS.freshwater_moisture_threshold {
            return TerrainType::FreshWater;
        }
        // arctic starts at 66.5
        if abs_latitude > 87. {
            if elevation < TERRAIN_CONSTANTS.ocean_elevation_threshold {
                if moisture > 0.5 {
                    return TerrainType::Ocean;
                }
                return TerrainType::Ice;
            }
            if moisture < 0.1 {
                return TerrainType::Scorched;
            }
            if moisture < 0.2 {
                return TerrainType::Bare;
            }
            if elevation < 0.7 {
                if moisture < 0.7 {
                    return TerrainType::Tundra;
                }
                return TerrainType::TundraMarsh;
            }
            return TerrainType::Snow;
        }

        if elevation < TERRAIN_CONSTANTS.ocean_elevation_threshold {
            return TerrainType::Ocean;
        }
        if elevation > 0.8 {
            if moisture < 0.1 {
                return TerrainType::Scorched;
            }
            if moisture < 0.2 {
                return TerrainType::Bare;
            }
            return TerrainType::Snow;
        }

        if elevation < TERRAIN_CONSTANTS.saltflat_elevation_threshold && moisture < 0.2 {
            return TerrainType::SaltFlat;
        }

        if abs_latitude > 80. {
            if moisture < 0.1 {
                return TerrainType::Scorched;
            }
            if moisture < 0.2 {
                return TerrainType::Bare;
            }
            if moisture < 0.85 {
                return TerrainType::Taiga;
            }
            return TerrainType::Marsh;
        }

        if abs_latitude > 30. {
            if elevation > 0.6 {
                if moisture < 0.33 {
                    return TerrainType::TemperateDesert;
                }
                if moisture < 0.66 {
                    return TerrainType::Shrubland;
                }
                return TerrainType::Grassland;
            }

            if moisture < 0.1 {
                return TerrainType::TemperateDesert;
            }
            if moisture < 0.70 {
                return TerrainType::Grassland;
            }
            if moisture < 0.83 {
                return TerrainType::TemperateDeciduousForest;
            }
            return TerrainType::TemperateRainForest;
        }

        if moisture < 0.1 {
            return TerrainType::SubtropicalDesert;
        }
        if moisture < 0.5 {
            return TerrainType::Grassland;
        }
        if moisture < 0.72 {
            return TerrainType::TropicalSeasonalForest;
        }
        return TerrainType::TropicalRainForest;
    }
}
