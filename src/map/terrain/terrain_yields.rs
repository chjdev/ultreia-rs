use crate::good::{Good, HarvestableGood, Inventory};
use crate::map::terrain::TerrainType;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Yield(u8);

const PERCENT100_YIELD: f64 = (u8::max_value() / 2) as f64;
const PERCENT200_YIELD: f64 = u8::max_value() as f64;

impl Yield {
    pub fn percent(&self) -> f64 {
        (self.0 as f64) / PERCENT100_YIELD
    }

    pub fn from_f64(value: f64) -> Option<Yield> {
        if value < 0. || value > 2. {
            None
        } else {
            Some(Yield((value * PERCENT100_YIELD) as u8))
        }
    }

    fn from_f64_saturating(value: f64) -> Yield {
        match value {
            _ if value < 0. => Yield(0),
            _ if value > 2. => Yield(u8::max_value()),
            value => Yield((value * PERCENT100_YIELD) as u8),
        }
    }
}

impl Into<f64> for Yield {
    fn into(self) -> f64 {
        self.percent()
    }
}

#[derive(Default, Eq)]
pub struct TerrainYields(Inventory<Yield>);

impl TerrainYields {
    pub fn new(_latitude: f64, _longitude: f64, _terrain_type: &TerrainType) -> Self {
        let mut yields = Inventory::<Yield>::new();
        yields.insert(
            Good::HarvestableGood(HarvestableGood::Game),
            Yield::from_f64_saturating(1.0),
        );
        Self(yields)
    }

    pub fn yields(&self) -> &Inventory<Yield> {
        &self.0
    }
}

impl PartialEq for TerrainYields {
    fn eq(&self, other: &Self) -> bool {
        other.yields().len() == self.yields().len()
            && other
                .yields()
                .keys()
                .all(|k| other.yields().get(k) == (self.yields().get(k)))
    }
}
