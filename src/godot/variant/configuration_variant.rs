use crate::game::Configuration;
use gdnative::core_types::{Dictionary, FromVariant, FromVariantError, ToVariant, Variant};

impl ToVariant for Configuration {
    fn to_variant(&self) -> Variant {
        let dict = Dictionary::new();
        dict.insert("rows", self.rows());
        dict.insert("columns", self.columns());
        dict.insert("island_noise", self.island_noise());
        Variant::from_dictionary(&dict.into_shared())
    }
}

impl FromVariant for Configuration {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        if let Some(dict) = variant.try_to_dictionary() {
            let rows = dict.get("rows").to_u64() as usize;
            let columns = dict.get("columns").to_u64() as usize;
            let island_noise = dict.get("island_noise").to_f64();
            Ok(Configuration::new(rows, columns, island_noise))
        } else {
            Err(FromVariantError::custom(
                "could not convert variant into a TerrainTile",
            ))
        }
    }
}
