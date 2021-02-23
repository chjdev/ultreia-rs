use crate::map::territories::TerritoryID;
use gdnative::prelude::{FromVariant, FromVariantError, ToVariant, Variant};

impl ToVariant for TerritoryID {
    fn to_variant(&self) -> Variant {
        let value: usize = (*self).into();
        value.to_variant()
    }
}

impl FromVariant for TerritoryID {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        if let Some(value) = variant.try_to_u64() {
            Ok((value as usize).into())
        } else {
            Err(FromVariantError::custom(
                "could not convert variant into a TerrainID",
            ))
        }
    }
}
