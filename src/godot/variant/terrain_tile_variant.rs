use crate::map::terrain::{TerrainTile, TerrainType};
use gdnative::core_types::{Dictionary, FromVariant, FromVariantError, ToVariant, Variant};

impl ToVariant for TerrainTile {
    fn to_variant(&self) -> Variant {
        let dict = Dictionary::new();
        dict.insert("elevation", self.elevation());
        dict.insert("moisture", self.moisture());
        dict.insert("terrain_type", self.terrain_type().to_variant());
        Variant::from_dictionary(&dict.into_shared())
    }
}

impl FromVariant for TerrainTile {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        if let Some(dict) = variant.try_to_dictionary() {
            let elevation = dict.get("elevation").to_f64();
            let moisture = dict.get("moisture").to_f64();
            let terrain_type_result = TerrainType::from_variant(&dict.get("terrain_type"));
            terrain_type_result
                .map(|terrain_type| TerrainTile::new(elevation, moisture, terrain_type))
        } else {
            Err(FromVariantError::custom(
                "could not convert variant into a TerrainTile",
            ))
        }
    }
}
