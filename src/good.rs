use strum_macros::{AsRefStr, EnumCount, EnumIter, IntoStaticStr};

pub use self::inventory::{Inventory, InventoryAmount, SpecializedInventory, WithFromInventory};

pub mod costs;
mod inventory;

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
    GoldOreRepo,
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
