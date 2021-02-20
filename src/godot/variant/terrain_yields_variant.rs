use crate::yields::Yield;
use gdnative::core_types::{ToVariant, Variant};

impl ToVariant for Yield {
    fn to_variant(&self) -> Variant {
        Variant::from_f64(self.percent())
    }
}
