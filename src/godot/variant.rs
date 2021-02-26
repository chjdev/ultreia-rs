macro_rules! enum_variant {
    ($enum:ty) => {
        impl ToVariant for $enum {
            fn to_variant(&self) -> Variant {
                Variant::from_str(self.as_ref())
            }
        }

        impl FromVariant for $enum {
            fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
                variant
                    .try_to_string()
                    .ok_or(FromVariantError::InvalidVariantType {
                        variant_type: variant.get_type(),
                        expected: VariantType::GodotString,
                    })
                    .and_then(|string| {
                        <$enum>::from_str(string.as_str()).map_err(|_| {
                            FromVariantError::UnknownEnumVariant {
                                variant: string,
                                expected: <$enum>::VARIANTS,
                            }
                        })
                    })
            }
        }
    };
}

mod arcer_variant;
mod configuration_variant;
mod construction_error_variant;
mod coordinate_variant;
pub mod good_variant;
mod make_dict;
mod terrain_meta_variant;
mod terrain_type_variant;
mod terrain_yields_variant;
mod territory_id_variant;
mod tile_name_variant;
