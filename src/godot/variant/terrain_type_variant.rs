use crate::map::terrain::TerrainType;
use gdnative::core_types::{FromVariant, FromVariantError, ToVariant, Variant};
use strum::{EnumCount, IntoEnumIterator};

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
                if (value as usize) >= TerrainType::COUNT {
                    return Err(FromVariantError::custom("number outside enum scope"));
                }
                Ok(TerrainType::iter().nth(value as usize).unwrap())
            })
    }
}
