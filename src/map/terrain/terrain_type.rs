use strum_macros::{EnumIter, EnumVariantNames, IntoStaticStr};

#[derive(PartialEq, Eq, Copy, Clone, EnumIter, IntoStaticStr, EnumVariantNames)]
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
}

impl Default for TerrainType {
    fn default() -> Self {
        Self::Bare
    }
}

impl TerrainType {
    pub fn new(latitude: f64, elevation: f64, moisture: f64) -> Self {
        if elevation > 0.75 {
            if moisture < 0.1 {
                return TerrainType::DesertMountain;
            }
            return TerrainType::Mountain;
        }
        let base_terrain_type = Self::base_terrain_type(latitude, elevation, moisture);
        if elevation > 0.55 {
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

    fn base_terrain_type(latitude: f64, elevation: f64, moisture: f64) -> Self {
        let abs_latitude = latitude.abs();
        if abs_latitude > 89.25 {
            if elevation < 0.1 {
                return TerrainType::Ice;
            }
            return TerrainType::Snow;
        }
        // arctic starts at 66.5
        if abs_latitude > 83. {
            if elevation < 0.1 {
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

        if elevation < 0.1 {
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

        if abs_latitude > 67. {
            if moisture < 0.1 {
                return TerrainType::Scorched;
            }
            if moisture < 0.2 {
                return TerrainType::Bare;
            }
            if moisture < 0.7 {
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
