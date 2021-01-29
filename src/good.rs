use std::collections::HashMap;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
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
    Whale,
    WildFish,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum BuildingMaterial {
    Bells,
    Brick,
    Engineer,
    Marble,
    Stone,
    Tool,
    Wood,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Weapon {
    Armor,
    Cannon,
    Mortar,
    Musket,
    Pike,
    Sword,
    WarHorse,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum ImmaterialGood {
    Culture,
    Education,
    Faith,
    Hygiene,
    Money,
    Prestige,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Good {
    BuildingMaterial(BuildingMaterial),
    HarvestableGood(HarvestableGood),
    ImmaterialGood(ImmaterialGood),
    NaturalGood(NaturalGood),
    ProductionGood(ProductionGood),
    Weapon(Weapon),
}

pub type InventoryAmount = u32;
pub type Inventory = HashMap<Good, InventoryAmount>;
