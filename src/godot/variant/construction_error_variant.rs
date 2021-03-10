use crate::map::buildings::buildings_controller::ConstructionError;
use gdnative::core_types::{ToVariant, Variant};
use gdnative::prelude::{FromVariant, FromVariantError, VariantType};
use std::str::FromStr;
use strum::VariantNames;

enum_variant!(ConstructionError);
