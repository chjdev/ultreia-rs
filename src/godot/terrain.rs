use gdnative::prelude::*;

use crate::coordinate::Coordinate;
use crate::godot::game_instance::GameController;
use crate::map::terrain::{TerrainTile, TerrainType};
use strum::{IntoEnumIterator, VariantNames};

lazy_static! {
    static ref TERRAIN_ENUM: Dictionary<Shared> = {
        let dictionary = Dictionary::new();
        for terrain_type in TerrainType::iter() {
            dictionary.insert(String::from(Into::<&str>::into(terrain_type)), terrain_type);
            dictionary.insert(terrain_type, String::from(Into::<&str>::into(terrain_type)));
        }
        dictionary.into_shared()
    };
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Terrain;

impl Terrain {
    fn new(_owner: &Node) -> Self {
        Terrain {}
    }
}

#[methods]
impl Terrain {
    #[export]
    fn terrain_enum(&self, _owner: &Node) -> Dictionary<Unique> {
        TERRAIN_ENUM.duplicate()
    }

    #[export]
    fn to_string(&self, _onwer: &Node, terrain_type: TerrainType) -> &str {
        terrain_type.into()
    }

    #[export]
    fn terrain_types(&self, _owner: &Node) -> Vec<TerrainType> {
        TerrainType::iter().collect()
    }

    #[export]
    fn terrain_type_strings(&self, _owner: &Node) -> Vec<&str> {
        TerrainType::VARIANTS.to_vec()
    }

    #[export]
    fn at(&self, _owner: &Node, coordinate: Coordinate) -> Option<TerrainTile> {
        Some(GameController.game()?.map().terrain().get(&coordinate))
    }

    #[export]
    fn minimap(&self, _owner: &Node, width: u16, height: u16) -> Option<Vec<TerrainType>> {
        Some(
            GameController
                .game()?
                .map()
                .terrain()
                .minimap(width, height),
        )
    }
}
