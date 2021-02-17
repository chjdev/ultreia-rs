use crate::good::Good;
use gdnative::core_types::{ToVariant, ToVariantEq, Variant};

impl ToVariantEq for &Good {}

impl ToVariant for Good {
    fn to_variant(&self) -> Variant {
        let bla: usize = self.into();
        Variant::from_u64(bla as u64)
    }
}
