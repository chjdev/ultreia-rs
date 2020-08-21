use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BuildingMaterial {
    Tool,
    Wood,
    Marble,
    Brick,
    Stone,
    Bells,
    Engineer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    Wheat,
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
    Wine,
    Wool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Weapon {
    Pike,
    Sword,
    Armor,
    Musket,
    Cannon,
    Mortar,
    WarHorse,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    NaturalGood(NaturalGood),
    BuildingMaterial(BuildingMaterial),
    HarvestableGood(HarvestableGood),
    ProductionGood(ProductionGood),
    Weapon(Weapon),
    ImmaterialGood(ImmaterialGood),
}

pub type Inventory = HashMap<Good, i32>;
