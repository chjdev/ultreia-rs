use crate::good::*;
use gdnative::core_types::{ToVariant, ToVariantEq, Variant};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::Add;
use strum::IntoEnumIterator;

const SEP: &'static str = "::";
lazy_static! {
    static ref GOOD_STRINGS: HashMap<Good, &'static str> = HashMap::from_iter(
        Good::iter()
            .flat_map(|good| -> Vec<(Good, String)> {
                let bla: String = String::from(good.as_ref()) + SEP;
                match good {
                    Good::BuildingMaterial(_) => BuildingMaterial::iter()
                        .map(|sub| (Good::BuildingMaterial(sub), bla.clone().add(sub.as_ref())))
                        .collect(),
                    Good::HarvestableGood(_) => HarvestableGood::iter()
                        .map(|sub| (Good::HarvestableGood(sub), bla.clone().add(sub.as_ref())))
                        .collect(),
                    Good::ImmaterialGood(_) => ImmaterialGood::iter()
                        .map(|sub| (Good::ImmaterialGood(sub), bla.clone().add(sub.as_ref())))
                        .collect(),
                    Good::NaturalGood(_) => NaturalGood::iter()
                        .map(|sub| (Good::NaturalGood(sub), bla.clone().add(sub.as_ref())))
                        .collect(),
                    Good::ProductionGood(_) => ProductionGood::iter()
                        .map(|sub| (Good::ProductionGood(sub), bla.clone().add(sub.as_ref())))
                        .collect(),
                    Good::Weapon(_) => Weapon::iter()
                        .map(|sub| (Good::Weapon(sub), bla.clone().add(sub.as_ref())))
                        .collect(),
                }
            })
            .map(|(good, string)| (good, Box::leak(string.into_boxed_str()) as &'static str))
    );
}

impl From<&Good> for &'static str {
    fn from(good: &Good) -> Self {
        GOOD_STRINGS[good]
    }
}

impl From<Good> for &'static str {
    fn from(good: Good) -> Self {
        (&good).into()
    }
}

lazy_static! {
    static ref ALL_GOODS: Vec<Good> = Good::iter()
        .flat_map(|good| -> Vec<Good> {
            match good {
                Good::BuildingMaterial(_) => BuildingMaterial::iter()
                    .map(Good::BuildingMaterial)
                    .collect(),
                Good::HarvestableGood(_) => {
                    HarvestableGood::iter().map(Good::HarvestableGood).collect()
                }
                Good::ImmaterialGood(_) => {
                    ImmaterialGood::iter().map(Good::ImmaterialGood).collect()
                }
                Good::NaturalGood(_) => NaturalGood::iter().map(Good::NaturalGood).collect(),
                Good::ProductionGood(_) => {
                    ProductionGood::iter().map(Good::ProductionGood).collect()
                }
                Good::Weapon(_) => Weapon::iter().map(Good::Weapon).collect(),
            }
        })
        .collect();
    static ref REVERSE_ALL_GOODS: HashMap<&'static Good, usize> =
        HashMap::from_iter(ALL_GOODS.iter().enumerate().map(|(idx, good)| (good, idx)));
}

impl From<usize> for &Good {
    fn from(value: usize) -> Self {
        &ALL_GOODS[value.clamp(0, ALL_GOODS.len() - 1)]
    }
}

impl From<&Good> for usize {
    fn from(good: &Good) -> Self {
        REVERSE_ALL_GOODS[good]
    }
}

impl ToVariantEq for &Good {}

impl ToVariant for Good {
    fn to_variant(&self) -> Variant {
        let bla: usize = self.into();
        Variant::from_u64(bla as u64)
    }
}
