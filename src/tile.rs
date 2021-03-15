use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString, EnumVariantNames};

use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
use crate::good::costs::Costs;
use crate::good::Inventory;
use crate::map::Map;
use crate::tile::consumes::Consumes;
use crate::tile::pioneer::Pioneer;
use crate::tile::produces::Produces;
use crate::tile::state::State;
use crate::tile::warehouse::Warehouse;

pub mod consumes;
mod pioneer;
pub mod produces;
pub mod state;
mod warehouse;

#[derive(Copy, Clone, PartialEq, Eq, Hash, EnumIter, AsRefStr, EnumString, EnumVariantNames)]
pub enum TileName {
    Pioneer,
    Warehouse,
}

impl Default for TileName {
    fn default() -> Self {
        TileName::Warehouse
    }
}

pub type SomeTile = Box<dyn Tile>;

pub trait Tile: Send + Sync {
    fn name(&self) -> &TileName;
    fn costs(&self) -> Option<&Costs> {
        None
    }
    fn consumes(&self) -> Option<&Consumes> {
        None
    }
    fn produces(&self) -> Option<&Produces> {
        None
    }
    fn allowed(&self, _at: &Coordinate, _map: &Map) -> bool {
        false
    }
    fn influence_at(&self, at: &Coordinate) -> Range;
    fn influence(&self) -> Range {
        self.influence_at(&Default::default())
    }
}

lazy_static! {
    static ref PIONEER: Pioneer = Pioneer::new();
    static ref WAREHOUSE: Warehouse = Warehouse::new();
    static ref INSTANCES: HashMap<TileName, &'static dyn Tile> = {
        let mut instances: HashMap<TileName, &'static dyn Tile> = HashMap::new();
        // so we don't forget one, match has to be exhaustive
        for tile_name in TileName::iter() {
            let tile: &'static dyn Tile = match tile_name {
                TileName::Pioneer => &*PIONEER,
                TileName::Warehouse => &*WAREHOUSE,
            };
            instances.insert(tile_name, tile);
        }
        instances
    };
}

impl Into<&'static dyn Tile> for &TileName {
    fn into(self) -> &'static dyn Tile {
        *INSTANCES.get(self).unwrap()
    }
}

impl Into<&'static dyn Tile> for TileName {
    fn into(self) -> &'static dyn Tile {
        (&self).into()
    }
}

impl<'a> From<&'a dyn Tile> for &'a TileName {
    fn from(tile: &'a dyn Tile) -> Self {
        tile.name()
    }
}

impl From<&dyn Tile> for TileName {
    fn from(tile: &dyn Tile) -> Self {
        *tile.name()
    }
}

pub struct TileInstance {
    tile: &'static dyn Tile,
    state: Option<State>,
}

impl TileInstance {
    fn new(tile: &'static dyn Tile, state: Option<State>) -> Self {
        TileInstance { tile, state }
    }

    pub fn from(tile: &'static dyn Tile) -> Self {
        TileInstance::new(tile, State::from(tile.consumes(), tile.produces()))
    }

    pub fn from_name(tile_name: &TileName) -> TileInstance {
        TileInstance::from(tile_name.into())
    }

    pub fn tile(&self) -> &'static dyn Tile {
        self.tile
    }

    pub fn state(&self) -> Option<&State> {
        self.state.as_ref()
    }

    fn state_mut(&mut self) -> Option<&mut State> {
        self.state.as_mut()
    }

    pub fn consume(&mut self, from: &mut Self) {
        let maybe_consumes = self.tile.consumes();
        if maybe_consumes.is_none() {
            return;
        }

        let maybe_state = self.state_mut();
        if maybe_state.is_none() {
            return;
        }
        let state = maybe_state.unwrap();
        let maybe_other_state = from.state_mut();
        if maybe_other_state.is_none() {
            return;
        }
        let other_state: &mut Inventory = &mut maybe_other_state.unwrap();
        for (consumption_good, amount) in maybe_consumes.unwrap().iter() {
            if !other_state.contains_key(&consumption_good) {
                continue;
            }
            let other_amount =
                other_state[consumption_good].min(amount.saturating_sub(state[consumption_good]));
            if other_amount > 0 {
                state[consumption_good] += other_amount;
                other_state[consumption_good] -= other_amount;
            }
        }
    }

    pub fn produce(&mut self) {
        let maybe_produces = self.tile.produces();
        if maybe_produces.is_none() {
            return;
        }

        let maybe_state = self.state_mut();
        if maybe_state.is_none() {
            return;
        }
        let state = maybe_state.unwrap();

        // very simple variant, usually <2 goods produced with ~2 ingredients each
        loop {
            let mut some_produced = false;
            for (production_good, ingredients) in maybe_produces.unwrap().iter() {
                let mut consumed = Inventory::new();
                let mut insufficient = false;
                for (ingredient, ingredient_amount) in ingredients.iter() {
                    if &state[ingredient] > ingredient_amount {
                        consumed[ingredient] = *ingredient_amount;
                    } else {
                        insufficient = true;
                        break;
                    }
                }
                if insufficient {
                    // move it back
                    *state += State::new(consumed);
                } else {
                    some_produced = true;
                    state[production_good] += 1;
                }
            }
            if !some_produced {
                break;
            }
        }
    }
}
