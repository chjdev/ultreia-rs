use crate::map::buildings::Buildings;
use crate::map::fow::FOW;
use crate::map::terrain::Terrain;
use crate::map::territories::Territories;

pub mod buildings;
pub mod fow;
pub mod minimap;
pub mod terrain;
pub mod territories;

pub struct Map {
    terrain: Terrain,
    territories: Territories,
    fow: FOW,
    buildings: Buildings,
}

impl Map {
    pub fn new(rows: usize, columns: usize, island_noise: f64) -> Self {
        Map {
            terrain: Terrain::new(rows, columns, island_noise),
            territories: Default::default(),
            fow: Default::default(),
            buildings: Default::default(),
        }
    }

    pub fn fow(&self) -> &FOW {
        &self.fow
    }

    pub fn terrain(&self) -> &Terrain {
        &self.terrain
    }

    pub fn territories(&self) -> &Territories {
        &self.territories
    }
}
