use crate::map::fow::FOW;
use crate::map::terrain::Terrain;
use crate::map::territories::Territories;

pub mod fow;
pub mod minimap;
pub mod terrain;
pub mod territories;

pub struct Map {
    rows: usize,
    columns: usize,
    terrain: Terrain,
    territories: Territories,
    fow: FOW,
}

impl Map {
    pub fn new(rows: usize, columns: usize, island_noise: f64) -> Self {
        Map {
            rows,
            columns,
            terrain: Terrain::new(rows, columns, island_noise),
            territories: Default::default(),
            fow: Default::default(),
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

    // pub fn can_construct(&self, at: &Coordinate, tile: Tiles) -> bool {
    //     return if let Some(territory) = self.territory_for_coordinate(&at) {
    //         territory.can_construct(&at, tile)
    //     } else {
    //         if tile == Tiles::Warehouse && self.territories.is_empty() {
    //             return TileFactory::instance().tile(Tiles::Warehouse).allowed(
    //                 at,
    //                 self.terrain(),
    //                 None,
    //             );
    //         }
    //         false
    //     };
    // }
    //
    // pub fn can_construct_range(&self, range: &Range, tile: Tiles) -> Vec<bool> {
    //     range
    //         .iter()
    //         .map(|coordinate| self.can_construct(coordinate, tile))
    //         .collect()
    // }
}
