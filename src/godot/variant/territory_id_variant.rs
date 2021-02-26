use crate::map::territories::TerritoryID;
use gdnative::prelude::{FromVariant, FromVariantError, ToVariant, Variant, VariantType};

impl ToVariant for TerritoryID {
    fn to_variant(&self) -> Variant {
        let value: usize = (*self).into();
        value.to_variant()
    }
}

impl FromVariant for TerritoryID {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        variant
            .try_to_u64()
            .ok_or(FromVariantError::InvalidVariantType {
                variant_type: variant.get_type(),
                expected: VariantType::I64,
            })
            .map(|value| (value as usize).into())
    }
}
