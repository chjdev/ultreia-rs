use std::collections::HashMap;
use std::convert::AsRef;
use std::iter::FromIterator;
use std::ops::Add;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumCount, EnumIter, IntoStaticStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, AsRefStr, IntoStaticStr, EnumIter, EnumCount)]
pub enum NaturalGood {
    CoalRepo,
    CopperOreRepo,
    FreshWater,
    GemStoneRepo,
    IronOreRepo,
    MarbleRepo,
    SaltRepo,
    SilverOreRepo,
    StoneRepo,
    ClayRepo,
    Whale,
    WildFish,
}

impl Default for NaturalGood {
    fn default() -> Self {
        Self::FreshWater
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, AsRefStr, IntoStaticStr, EnumIter, EnumCount)]
pub enum BuildingMaterial {
    Bells,
    Brick,
    Engineer,
    Marble,
    Stone,
    Tool,
    Wood,
}

impl Default for BuildingMaterial {
    fn default() -> Self {
        Self::Wood
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, AsRefStr, IntoStaticStr, EnumIter, EnumCount)]
pub enum HarvestableGood {
    Cattle,
    CocoaPlant,
    CottonPlant,
    Ears,
    FlowerPlant,
    Game,
    Grape,
    HempPlant,
    HopsPlant,
    IndigoPlant,
    PeltAnimal,
    PotatoPlant,
    Sheep,
    SilkWorm,
    SpicePlant,
    SugarCanePlant,
    TobaccoPlant,
    Tree,
    UntamedHorse,
}

impl Default for HarvestableGood {
    fn default() -> Self {
        Self::Tree
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, AsRefStr, IntoStaticStr, EnumIter, EnumCount)]
pub enum ProductionGood {
    Alcohol,
    Amber,
    Beer,
    Bees,
    Book,
    Bread,
    BronzeBar,
    Ceramic,
    Clay,
    Cloth,
    Clothes,
    Coal,
    Cocoa,
    CopperBar,
    CopperOre,
    Cotton,
    Finery,
    Fish,
    Flour,
    Flowers,
    Food,
    GemStone,
    GoldBar,
    GunPowder,
    Hemp,
    Honey,
    Hops,
    Horse,
    Indigo,
    Ink,
    Instrument,
    IronBar,
    IronOre,
    Jewellery,
    LampOil,
    Leather,
    Meat,
    Paper,
    Pelt,
    Perfume,
    Pigment,
    Porcelain,
    Potato,
    RawHide,
    Rope,
    Sails,
    Salt,
    Silk,
    SilverBar,
    SilverOre,
    Slag,
    Spices,
    Spirit,
    Sugar,
    SugarCane,
    TinBar,
    Tobacco,
    TobaccoLeaf,
    WhaleTallow,
    Wheat,
    Wine,
    Wool,
}

impl Default for ProductionGood {
    fn default() -> Self {
        Self::Food
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, AsRefStr, IntoStaticStr, EnumIter, EnumCount)]
pub enum Weapon {
    Armor,
    Cannon,
    Mortar,
    Musket,
    Pike,
    Sword,
    WarHorse,
}

impl Default for Weapon {
    fn default() -> Self {
        Self::Sword
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, AsRefStr, IntoStaticStr, EnumIter, EnumCount)]
pub enum ImmaterialGood {
    Culture,
    Education,
    Faith,
    Hygiene,
    Money,
    Prestige,
}

impl Default for ImmaterialGood {
    fn default() -> Self {
        Self::Money
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, AsRefStr, EnumIter, EnumCount)]
pub enum Good {
    BuildingMaterial(BuildingMaterial),
    HarvestableGood(HarvestableGood),
    ImmaterialGood(ImmaterialGood),
    NaturalGood(NaturalGood),
    ProductionGood(ProductionGood),
    Weapon(Weapon),
}

impl Default for Good {
    fn default() -> Self {
        Self::ImmaterialGood(ImmaterialGood::Money)
    }
}

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

pub type InventoryAmount = u32;
pub type Inventory<T = InventoryAmount> = HashMap<Good, T>;
