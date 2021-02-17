use crate::map::terrain::{TerrainYields, Yield};
use gdnative::core_types::{Dictionary, ToVariant, Variant};

impl ToVariant for Yield {
    fn to_variant(&self) -> Variant {
        Variant::from_f64(self.percent())
    }
}

impl ToVariant for TerrainYields {
    fn to_variant(&self) -> Variant {
        let dict = Dictionary::new();
        for (key, value) in self.yields() {
            dict.insert(key, value);
        }
        Variant::from_dictionary(&dict.into_shared())
    }
}
