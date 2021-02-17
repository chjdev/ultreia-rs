use crate::map::terrain::TerrainType;
use gdnative::core_types::{FromVariant, FromVariantError, ToVariant, ToVariantEq, Variant};
use strum::IntoEnumIterator;

impl ToVariantEq for TerrainType {}

impl ToVariant for TerrainType {
    fn to_variant(&self) -> Variant {
        Variant::from_u64((*self) as u64)
    }
}

impl FromVariant for TerrainType {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        variant
            .try_to_u64()
            .ok_or(FromVariantError::custom(
                "could not convert variant into a TerrainType",
            ))
            .and_then(|value| {
                Ok(TerrainType::iter()
                    .nth(value as usize)
                    .unwrap_or(TerrainType::default()))
            })
    }
}
