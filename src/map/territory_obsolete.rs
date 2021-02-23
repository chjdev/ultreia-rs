use crate::coordinate::indexed::CoordinateIndexed;
use crate::coordinate::range::Range;
use crate::coordinate::Coordinate;
use crate::good::Good;
use crate::map::terrain::Terrain;
use crate::tile::state::State;
use crate::tile::{SomeTileInstance, TileFactory, Tiles};
use std::sync::{RwLockReadGuard, Weak};

pub type TerritoryMap = CoordinateIndexed<SomeTileInstance>;

pub struct Territory {
    range: Range,
    warehouses: Range,
    map: TerritoryMap,
    terrain: Weak<Terrain>,
}

impl Territory {
    pub fn new(terrain: Weak<Terrain>) -> Self {
        Territory {
            range: Default::default(),
            warehouses: Default::default(),
            map: Default::default(),
            terrain,
        }
    }

    pub fn merge_territories(mut left: Territory, right: Territory) -> Self {
        assert_eq!(left.terrain.as_ptr(), right.terrain.as_ptr());
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
        if let Some(terrain) = self.terrain.upgrade() {
            let tile_config = TileFactory::instance().tile(tile);
            tile_config.allowed(at, terrain.as_ref(), Some(self));
            return if let Some(costs) = tile_config.costs() {
                self.state() > costs
            } else {
                true
            };
        }
        false
    }

    fn consume_costs_of(&mut self, tile: Tiles) -> Result<(), &'static str> {
        let maybe_costs = TileFactory::instance().tile(tile).costs();
        if maybe_costs.is_none() {
            return Ok(());
        }
        let costs = maybe_costs.expect("costs should be present here");
        let goods: Vec<Good> = costs.keys().cloned().collect();
        let mut costs_val = costs.clone();
        let mut execution_plan = vec![];
        let mut states = vec![];
        for warehouse_coordinate in &self.warehouses {
            let warehouse = self.get(&warehouse_coordinate).ok_or("should be there")?;
            let warehouse_state = warehouse
                .state_mut()
                .ok_or("a warehouse always has a state!")?
                .ok()
                .ok_or("couldn't lock warehouse state")?;
            states.push(warehouse_state);
        }

        for (state_idx, warehouse_state) in states.iter().enumerate() {
            for good in &goods {
                let cost = costs_val.get_mut(good).unwrap();
                if *cost <= 0 {
                    continue;
                }
                let has = warehouse_state.get(good).cloned().unwrap_or(0);
                let consumes = (*cost).min(has);
                *cost -= consumes;
                execution_plan.push((state_idx, good, consumes));
            }
            if costs_val.values().any(|value| *value > 0) {
                return Err("couldn't satisfy cost");
            }
        }

        for (state_idx, good, consumes) in execution_plan {
            *states[state_idx] -= (good, &consumes);
        }
        Ok(())
    }

    pub fn construct(&mut self, at: &Coordinate, tile: Tiles) -> Result<(), &'static str> {
        if !self.can_construct(at, tile) {
            return Err("can't construct at this location");
        }
        self.consume_costs_of(tile)?;
        self.map.insert(*at, TileFactory::instance().create(tile));
        if tile == Tiles::Warehouse {
            self.warehouses.insert(*at);
        }
        return Ok(());
    }

    pub fn state(&self) -> State {
        self.warehouses
            .iter()
            .map(|coordinate| self.get(coordinate))
            .filter(Option::is_some)
            .map(|option| option.unwrap())
            .filter(|maybe_warehouse| maybe_warehouse.tile() == &Tiles::Warehouse)
            .map(|warehouse| -> Option<RwLockReadGuard<State>> { warehouse.state()?.ok() })
            .filter(|maybe_warehouse_state| maybe_warehouse_state.is_some())
            .map(|warehouse_state| warehouse_state.unwrap())
            .fold(State::new(), |mut acc, warehouse_state| {
                acc += &*warehouse_state;
                acc
            })
    }

    pub fn get(&self, coordinate: &Coordinate) -> Option<&SomeTileInstance> {
        self.map.get(coordinate)
    }

    pub fn get_mut(&mut self, coordinate: &Coordinate) -> Option<&mut SomeTileInstance> {
        self.map.get_mut(coordinate)
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
