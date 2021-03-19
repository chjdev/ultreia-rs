use strum_macros::{AsRefStr, Display, EnumCount, EnumIter, IntoStaticStr};

pub use self::inventory::{Inventory, InventoryAmount, SpecializedInventory, WithFromInventory};

pub mod costs;
mod inventory;

macro_rules! make_good {
    ($name:tt, default $default:tt, $($arg:tt),+) => {
        #[derive(
            Debug, Display, Copy, Clone, PartialEq, Eq, Hash, AsRefStr, IntoStaticStr, EnumIter, EnumCount,
        )]
        pub enum $name {
            $($arg),+
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default
            }
        }

        impl Into<Good> for $name {
            fn into(self) -> Good {
                Good::$name(self)
            }
        }

        impl Good {
        $(
            #[allow(non_snake_case)]
            pub const fn $arg() -> Self {
                Good::$name($name::$arg)
            }
        )+
        }
    };
}

make_good!(
    NaturalGood,
    default FreshWater,
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
    WildFish
);

make_good!(
    BuildingMaterial,
    default Wood,
    Bells,
    Brick,
    Engineer,
    Marble,
    Stone,
    Tool,
    Wood
);

make_good!(
    HarvestableGood,
    default Tree,
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
    UntamedHorse
);

make_good!(
    ProductionGood,
    default Food,
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
    Wool
);

make_good!(Weapon, default Sword, Armor, Cannon, Mortar, Musket, Pike, Sword, WarHorse);

make_good!(
    ImmaterialGood,
    default Money,
    Culture,
    Education,
    Faith,
    Hygiene,
    Money,
    Prestige
);

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, Hash, AsRefStr, EnumIter, EnumCount)]
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
