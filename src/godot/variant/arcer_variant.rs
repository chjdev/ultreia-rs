use crate::godot::arcer::Arcer;
use gdnative::prelude::{ToVariant, Variant};
use std::ops::Deref;

impl<T: ToVariant> ToVariant for Arcer<T> {
    fn to_variant(&self) -> Variant {
        self.deref().to_variant()
    }
}
