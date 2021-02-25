use crate::tile::TileName;
use gdnative::core_types::{ToVariant, Variant};

impl ToVariant for TileName {
    fn to_variant(&self) -> Variant {
        Variant::from_str(self.as_ref())
    }
}
