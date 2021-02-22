use crate::coordinate::Coordinate;
use gdnative::core_types::{FromVariant, FromVariantError, ToVariant, Variant, Vector3};

impl ToVariant for Coordinate {
    fn to_variant(&self) -> Variant {
        Vector3::new(self.x() as f32, self.y() as f32, self.z() as f32).to_variant()
    }
}

impl FromVariant for Coordinate {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        if let Some(vec3) = variant.try_to_vector3() {
            if vec3.z == -vec3.x - vec3.y {
                Ok(Coordinate::round(vec3.x as f64, vec3.y as f64))
            } else {
                Err(FromVariantError::custom(
                    "could not convert variant into a Coordinate, does not fulfill z = -x - y!",
                ))
            }
        } else if let Some(vec2) = variant.try_to_vector2() {
            Ok(Coordinate::round(vec2.x as f64, vec2.y as f64))
        } else {
            Err(FromVariantError::custom(
                "could not convert variant into a Coordinate",
            ))
        }
    }
}
