use crate::clock::Clock;
use crate::map::Map;

#[derive(Copy, Clone)]
pub struct Configuration {
    rows: usize,
    columns: usize,
    island_noise: f64,
}

impl Configuration {
    pub fn new(rows: usize, columns: usize, island_noise: f64) -> Self {
        Configuration {
            rows,
            columns,
            island_noise,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn island_noise(&self) -> f64 {
        self.island_noise
    }
}

pub struct Game {
    configuration: Configuration,
    clock: Clock,
    map: Map,
}

impl Game {
    pub fn new(configuration: Configuration) -> Self {
        let clock = Clock::new();
        Game {
            configuration,
            map: Map::new(
                &clock,
                configuration.rows,
                configuration.columns,
                configuration.island_noise,
            ),
            clock,
        }
    }

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }
}

#[cfg(test)]
mod tests {
    use strum::EnumCount;

    use crate::coordinate::Coordinate;
    use crate::map::minimap::GetByCoordinate;
    use crate::map::terrain::{TerrainMeta, TerrainType};

    use super::*;

    #[test]
    fn test_smoke() {
        let game = Game::new(Configuration::new(100, 100, 4.));
        let coordinate = Coordinate::default();
        let terrain_type: TerrainType = game.map().terrain().get(&coordinate);
        assert!((terrain_type as usize) < TerrainType::COUNT);
        let terrain_meta: TerrainMeta = game.map().terrain().get(&coordinate);
        assert!(terrain_meta.moisture() >= 0.);
    }
}
