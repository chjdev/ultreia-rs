use crate::map::terrain::{Elevation, Moisture, TerrainMeta};
use gdnative::core_types::{Dictionary, ToVariant, Variant};

impl ToVariant for Elevation {
    fn to_variant(&self) -> Variant {
        Variant::from_f64((*self).into())
    }
}

impl ToVariant for Moisture {
    fn to_variant(&self) -> Variant {
        Variant::from_f64((*self).into())
    }
}

impl ToVariant for TerrainMeta {
    fn to_variant(&self) -> Variant {
        let dict = Dictionary::new();
        dict.insert("elevation", self.elevation());
        dict.insert("moisture", self.moisture());
        dict.insert("terrain_type", self.terrain_type().to_variant());
        dict.insert("yields", self.yields());
        Variant::from_dictionary(&dict.into_shared())
    }
}
