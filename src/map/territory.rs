use std::sync::Weak;

use crate::coordinate::indexed::Indexed;
use crate::coordinate::Coordinate;
use crate::tile::{Tiles, TileFactory, State, SomeTileInstance};
use crate::coordinate::range::Range;
use crate::map::terrain::Terrain;

pub type TerritoryMap = Indexed<SomeTileInstance>;

pub struct Territory {
    range: Range,
    warehouses: Range,
    map: TerritoryMap,
    terrain: Weak<Terrain>,
    tile_factory: Weak<TileFactory>,
}

impl Territory {
    pub fn new(terrain: Weak<Terrain>, tile_factory: Weak<TileFactory>) -> Self {
        Territory {
            range: Default::default(),
            warehouses: Default::default(),
            map: Default::default(),
            terrain,
            tile_factory,
        }
    }

    pub fn merge_territories(mut left: Territory, right: Territory) -> Self {
        assert_eq!(left.terrain.as_ptr(), right.terrain.as_ptr());
        assert_eq!(left.tile_factory.as_ptr(), right.tile_factory.as_ptr());
        left.range.extend(right.range);
        left.map.extend(right.map);
        left
    }

    pub fn merge(self, other: Territory) -> Self {
        Self::merge_territories(self, other)
    }

    pub fn contains(&self, coordinate: &Coordinate) -> bool {
        self.range.contains(coordinate)
    }

    pub fn can_construct(&self, at: &Coordinate, tile: Tiles) -> bool {
        if self.range.is_empty() {
            return tile == Tiles::Warehouse;
        }
        if !self.contains(at) {
            return false;
        }
        if self.map.contains_key(at) {
            return false;
        }
        if let (Some(tile_factory), Some(terrain)) = (self.tile_factory.upgrade(), self.terrain.upgrade()) {
            let tile_config = tile_factory.tile(tile);
            tile_config.allowed(at, terrain.as_ref(), Some(self));
            return if let Some(costs) = tile_config.costs() {
                self.state() > costs
            } else {
                true
            };
        }
        false
    }

    fn consume_costs_of(&mut self, tile: Tiles) {
        if let Some(tile_factory) = self.tile_factory.upgrade() {
            if let Some(costs) = tile_factory.tile(tile).costs() {
                for (good, value) in costs {
                    if *value <= 0 {
                        continue;
                    }
                    let mut rest = *value;
                    for warehouse_coordinate in &self.warehouses {
                        if rest <= 0 {
                            break;
                        }
                        let warehouse = self.get(&warehouse_coordinate).unwrap();
                        let mut state_lock = warehouse.state_mut();
                        let mut warehouse_state = state_lock.as_mut().unwrap();
                        let has = *warehouse_state.get(good);
                        let consumes = value.min(&has);
                        rest -= consumes;
                        warehouse_state -= (good, consumes);
                    }
                }
            }
        }
    }

    pub fn construct(&mut self, at: &Coordinate, tile: Tiles) -> bool {
        if !self.can_construct(at, tile) {
            return false;
        }
        if let Some(tile_factory) = self.tile_factory.upgrade() {
            self.consume_costs_of(tile);
            self.map.insert(*at, tile_factory.create(tile));
            if tile == Tiles::Warehouse {
                self.warehouses.insert(*at);
            }
            return true;
        }
        false
    }

    pub fn state(&self) -> State {
        self.warehouses.iter().map(|coordinate| self.get(coordinate)).fold(State::new(), |mut acc, maybe_warehouse| {
            if let Some(warehouse) = maybe_warehouse.filter(|instance| instance.tile() == &Tiles::Warehouse) {
                let state_lock = warehouse.state();
                let warehouse_state = state_lock.as_ref().unwrap();
                acc += &warehouse_state;
            }
            acc
        })
    }

    pub fn get(&self, coordinate: &Coordinate) -> Option<&SomeTileInstance> {
        self.map.get(coordinate)
    }
}

impl<'a> IntoIterator for &'a Territory {
    type Item = (&'a Coordinate, &'a SomeTileInstance);
    type IntoIter = std::collections::hash_map::Iter<'a, Coordinate, SomeTileInstance>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl<'a> IntoIterator for &'a mut Territory {
    type Item = (&'a Coordinate, &'a mut SomeTileInstance);
    type IntoIter = std::collections::hash_map::IterMut<'a, Coordinate, SomeTileInstance>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter_mut()
    }
}
